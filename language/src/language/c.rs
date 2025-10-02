pub mod language_object;
pub mod parsers;
pub mod writers;

use std::io::Cursor;

use crate::language::{
    Language,
    c::{
        language_object::{
            LanguageObject as CLanguageObject,
            source_file::SourceFile as CSourceFile,
        },
        parsers::{nodes::NodeParser, text::TreeSitterParser},
        writers::{
            Writer,
            node_writer::NodeWriter,
            text_writer::{TextWriter, style::Style},
        },
    },
};
use tree_sitter::Parser;
use uuid::Uuid;

pub mod object_types;

pub struct C {}

impl C {
    pub fn new() -> Self {
        Self {}
    }
}

impl Language for C {
    type Object = CLanguageObject;
    type SourceFile = CSourceFile;

    fn file_extension(&self) -> String {
        "c".to_string()
    }

    fn name(&self) -> String {
        "C".to_string()
    }

    fn parse_text(&self, source_code: &str) -> Result<Self::SourceFile, String> {
        let mut parser = Parser::new();

        parser
            .set_language(&tree_sitter_c::LANGUAGE.into())
            .expect("Error loading C grammar");

        let tree = parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();

        assert_eq!(
            root_node.kind(),
            "translation_unit",
            "Root node should be a translation unit"
        );

        let objects: Vec<CLanguageObject> =
            TreeSitterParser::parse_with_tree(root_node.child(0).unwrap(), source_code)?;

        return Ok(CSourceFile {
            id: Uuid::new_v4(),
            code: objects,
        });
    }

    fn parse_nodes(&self, nodes: Vec<u8>) -> Result<Self::SourceFile, String> {
        let mut node_reader = NodeParser::new();
        let src_file = node_reader.read_file(nodes).unwrap();
        Ok(src_file)
    }

    fn write_to_text(&self, src_file: Self::SourceFile) -> Result<String, String> {
        let mut buf: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        let stringwriter: Box<&mut dyn std::io::Write> = Box::new(&mut cursor);
        let mut writer = TextWriter::new(stringwriter, Style::gnu_style());

        writer.write_file(&src_file).unwrap();
        let string = String::from_utf8(buf).unwrap();
        Ok(string)
    }

    fn write_to_nodes(&self, src_file: Self::SourceFile) -> Result<Vec<u8>, String> {
        let mut buf: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        let writer: Box<&mut dyn std::io::Write> = Box::new(&mut cursor);
        NodeWriter::new(writer).write_file(&src_file).unwrap();
        Ok(buf)
    }
}

fn dfs(walker: &mut tree_sitter::TreeCursor, source_code: &str) -> Vec<String> {
    let node = walker.node();
    let mut lines = Vec::new();
    lines.append(
        &mut format!("'{}': {}", node.kind(), node.content(source_code))
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    );

    if walker.goto_first_child() {
        loop {
            for line in dfs(walker, source_code) {
                lines.push(format!("  {}", line));
            }
            if !walker.goto_next_sibling() {
                break;
            }
        }
        walker.goto_parent();
    }
    lines
}

pub trait TreeSitterNodeExt {
    fn content(&self, source_code: &str) -> String;
    fn dump(&self, source_code: &str) -> String;
}

impl TreeSitterNodeExt for tree_sitter::Node<'_> {
    fn content(&self, source_code: &str) -> String {
        source_code[self.start_byte()..self.end_byte()].to_string()
    }

    fn dump(&self, source_code: &str) -> String {
        dfs(&mut self.walk(), source_code).join("\n").to_string()
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::vec;

    use crate::language::c::{
        language_object::{
            assignment_expression::AssignmentExpression,
            binary_expression::BinaryExpression,
            call_expression::CallExpression,
            comment::Comment,
            compound_statement::CompoundStatement,
            declaration::Declaration,
            else_clause::ElseClause,
            function_declaration::FunctionDeclaration,
            function_definition::FunctionDefinition,
            function_parameter::FunctionParameter,
            if_statement::IfStatement,
            number_literal::NumberLiteral,
            preproc_include::PreprocInclude,
            reference::Reference,
            return_statement::ReturnStatement,
            string_literal::StringLiteral,
        },
        object_types::c_type::CType,
        parsers::{context::SymbolAlreadyExists, text::TreeSitterParserError},
    };

    use super::*;

    #[test]
    fn test_parse_empty_function_definition() {
        let c_code = "
int main() {
}
";
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    return_type: CType::Int,
                    identifier,
                    parameter_list,
                    compound_statement: CompoundStatement { code_block, .. },
                    ..
                }),
            ] => {
                assert_eq!(identifier, "main");
                assert!(parameter_list.is_empty());
                assert!(code_block.is_empty());
            }
            _ => panic!("AST did not match expected empty function"),
        }

        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_src_file = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_src_file);
    }

    #[test]
    fn test_parse_function_declaration_no_arguments() {
        let c_code = "
int main();
";
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDeclaration(FunctionDeclaration {
                    return_type: CType::Int,
                    identifier,
                    parameter_list,
                    ..
                }),
            ] => {
                assert_eq!(identifier, "main");
                assert!(parameter_list.is_empty());
            }
            _ => panic!("AST did not match expected function declaration"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_function_declaration() {
        let c_code = "
int first(int a, int b);
";
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDeclaration(FunctionDeclaration {
                    return_type: CType::Int,
                    identifier,
                    parameter_list,
                    ..
                }),
            ] => {
                assert_eq!(identifier, "first");
                match parameter_list.as_slice() {
                    [
                        FunctionParameter {
                            identifier: identifier_1,
                            param_type: param_type_1,
                            ..
                        },
                        FunctionParameter {
                            identifier: identifier_2,
                            param_type: param_type_2,
                            ..
                        },
                    ] => {
                        assert_eq!(identifier_1, "a");
                        assert_eq!(identifier_2, "b");
                        assert_eq!(param_type_1.clone(), CType::Int);
                        assert_eq!(param_type_2.clone(), CType::Int);
                    }
                    _ => panic!("AST did not match expected parameters"),
                }
            }
            _ => panic!("AST did not match expected function declaration"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_empty_function_definition_no_arguments() {
        let c_code = "
int main() {}
";
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [CLanguageObject::FunctionDefinition(FunctionDefinition { identifier, .. })] => {
                assert_eq!(identifier, "main");
            }
            _ => panic!("AST did not match expected function declaration"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_function_declaration_and_definition() {
        let c_code = "
int main();

int main() {
}
";
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDeclaration(FunctionDeclaration {
                    id: declaration_id,
                    identifier: declaration_identifier,
                    ..
                }),
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    id: definition_id,
                    identifier: definition_identifier,
                    ..
                }),
            ] => {
                assert_eq!(declaration_identifier, "main");
                assert_eq!(definition_identifier, "main");
                assert_eq!(declaration_id, definition_id);
            }
            _ => panic!("AST did not match expected function declaration"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_comment() {
        let c_code = "
// This is a line comment
int main() {

    /*
     This is a block comment
   */
}
";
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::Comment(Comment {
                    content: line_comment, ..
                }),
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    compound_statement: CompoundStatement { code_block, .. },
                    ..
                }),
            ] => {
                assert_eq!(line_comment, "// This is a line comment");
                match code_block.as_slice() {
                    [
                        CLanguageObject::Comment(Comment {
                            content: block_comment, ..
                        }),
                    ] => {
                        assert_eq!(block_comment, "/*\n     This is a block comment\n   */");
                    }
                    _ => panic!("AST did not match expected block comment"),
                }
            }
            _ => panic!("AST did not match expected comments and function definition"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_variable_init() {
        let c_code = r#"
int main() {
    int a = 5;
    int b = "Hello, World!\n";
}
"#;
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    return_type: CType::Int,
                    identifier,
                    parameter_list,
                    compound_statement: CompoundStatement { code_block, .. },
                    ..
                }),
            ] => {
                assert_eq!(identifier, "main");
                assert!(parameter_list.is_empty());
                match code_block.as_slice() {
                    [
                        CLanguageObject::Declaration(Declaration {
                            primitive_type: CType::Int,
                            identifier: a_identifier,
                            value: Some(a_value),
                            ..
                        }),
                        CLanguageObject::Declaration(Declaration {
                            primitive_type: CType::Int,
                            identifier: b_identifier,
                            value: Some(b_value),
                            ..
                        }),
                    ] => {
                        assert_eq!(a_identifier, "a");
                        assert_eq!(b_identifier, "b");
                        assert_eq!(
                            a_value,
                            &Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                                id: Uuid::new_v4(),
                                value: "5".to_string(),
                            }))
                        );
                        assert_eq!(
                            b_value,
                            &Box::new(CLanguageObject::StringLiteral(StringLiteral {
                                id: Uuid::new_v4(),
                                value: "Hello, World!\\n".to_string()
                            }))
                        );
                    }
                    _ => panic!("AST did not match expected variable declarations"),
                }
            }
            _ => panic!("AST did not match expected function definition"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_variable_declaration() {
        let c_code = "
int main() {
    int a;
}
";
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    return_type: CType::Int,
                    identifier,
                    parameter_list,
                    compound_statement: CompoundStatement { code_block, ..},
                    ..
                }),
            ] => {
                assert_eq!(identifier, "main");
                assert!(parameter_list.is_empty());
                match code_block.as_slice() {
                    [
                        CLanguageObject::Declaration(Declaration {
                            primitive_type: CType::Int,
                            identifier: a_identifier,
                            value: None,
                            ..
                        }),
                    ] => {
                        assert_eq!(a_identifier, "a");
                    }
                    _ => panic!("AST did not match expected variable declaration"),
                }
            }
            _ => panic!("AST did not match expected function definition"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_variable_assignment() {
        let c_code = "
int main() {
    int a;
    a = 5;
}
";
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    return_type: CType::Int,
                    identifier,
                    parameter_list,
                    compound_statement: CompoundStatement { code_block, .. },
                    ..
                }),
            ] => {
                assert_eq!(identifier, "main");
                assert!(parameter_list.is_empty());
                match code_block.as_slice() {
                    [
                        CLanguageObject::Declaration(Declaration {
                            id: declaration_id,
                            primitive_type: CType::Int,
                            identifier: a_identifier,
                            value: None,
                        }),
                        CLanguageObject::AssignmentExpression(AssignmentExpression {
                            id_declaration: assignment_id,
                            identifier: a_assignment_identifier,
                            value: a_assignment_value,
                            ..
                        }),
                    ] => {
                        assert_eq!(a_identifier, "a");
                        assert_eq!(a_assignment_identifier, "a");
                        assert_eq!(assignment_id, declaration_id);
                        assert_eq!(
                            a_assignment_value,
                            &Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                                id: Uuid::new_v4(),
                                value: "5".to_string()
                            }))
                        );
                    }
                    _ => {
                        panic!("AST did not match expected variable declaration and assignment")
                    }
                }
            }
            _ => panic!("AST did not match expected function definition"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_redeclare_variable() {
        {
            let c_code = "
            int main() {
                int a;
                int a;
                }
                ";
            let c_language = C::new();
            assert_eq!(
                c_language.parse_text(c_code),
                Err(String::from(TreeSitterParserError::SymbolAlreadyExists(
                    SymbolAlreadyExists {}
                )))
            );
        }

        {
            let c_code = "
            int main() {
                int a = 5;
                int a;
                }
                ";
            let c_language = C::new();
            assert_eq!(
                c_language.parse_text(c_code),
                Err(String::from(TreeSitterParserError::SymbolAlreadyExists(
                    SymbolAlreadyExists {}
                )))
            );
        }

        {
            let c_code = "
            int main() {
                int a;
                int a = 5;
                }
                ";
            let c_language = C::new();
            assert_eq!(
                c_language.parse_text(c_code),
                Err(String::from(TreeSitterParserError::SymbolAlreadyExists(
                    SymbolAlreadyExists {}
                )))
            );
        }
    }

    #[test]
    fn test_parse_scopes() {
        let c_code = r#"
int main() {
    int a = 1;
    int b = 2;
    int c;
    {
        b = a;
        int a = 3;
        c = a + b;
    }
}
"#;
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    return_type: CType::Int,
                    identifier,
                    parameter_list,
                    compound_statement: CompoundStatement { code_block, .. },
                    ..
                }),
            ] if *identifier == "main".to_string() && *parameter_list == vec![] => {
                match code_block.as_slice() {
                    [
                        CLanguageObject::Declaration(Declaration {
                            id: a_declaration_id,
                            primitive_type: CType::Int,
                            identifier: a_declaration_identifier,
                            value: Some(a_declaration_value),
                        }),
                        CLanguageObject::Declaration(Declaration {
                            id: b_declaration_id,
                            primitive_type: CType::Int,
                            identifier: b_declaration_identifier,
                            value: Some(b_declaration_value),
                        }),
                        CLanguageObject::Declaration(Declaration {
                            id: c_declaration_id,
                            primitive_type: CType::Int,
                            identifier: c_declaration_identifier,
                            value: None,
                        }),
                        CLanguageObject::CompoundStatement(CompoundStatement {
                            code_block: inner_scope,
                            ..
                        }),
                    ] if a_declaration_identifier == "a"
                        && b_declaration_identifier == "b"
                        && c_declaration_identifier == "c" =>
                    {
                        match a_declaration_value.as_ref() {
                            CLanguageObject::NumberLiteral(NumberLiteral { value, .. })
                                if value == "1" => {}
                            _ => panic!(),
                        }
                        match b_declaration_value.as_ref() {
                            CLanguageObject::NumberLiteral(NumberLiteral { value, .. })
                                if value == "2" => {}
                            _ => panic!(),
                        }
                        match inner_scope.as_slice() {
                            [
                                CLanguageObject::AssignmentExpression(AssignmentExpression {
                                    id_declaration: b_assignment_id,
                                    identifier: b_assignment_identifier,
                                    value: b_assignment_value,
                                    ..
                                }),
                                CLanguageObject::Declaration(Declaration {
                                    id: inner_a_declaration_id,
                                    primitive_type: CType::Int,
                                    identifier: inner_a_declaration_identifier,
                                    value: Some(inner_a_declaration_value),
                                }),
                                CLanguageObject::AssignmentExpression(AssignmentExpression {
                                    id_declaration: c_assignment_id,
                                    identifier: c_assignment_identifier,
                                    value: c_assignment_value,
                                    ..
                                }),
                            ] => {
                                assert_eq!(b_assignment_identifier, "b");
                                assert_eq!(b_assignment_id, b_declaration_id);
                                match b_assignment_value.as_ref() {
                                    CLanguageObject::Reference(Reference { identifier, declaration_id, .. })
                                        if identifier == "a" && declaration_id == a_declaration_id => {}
                                    _ => panic!(),
                                }

                                assert_eq!(inner_a_declaration_identifier, "a");
                                assert_eq!(c_assignment_identifier, "c");
                                assert_eq!(c_assignment_id, c_declaration_id);
                                assert_ne!(a_declaration_id, inner_a_declaration_id);
                                match inner_a_declaration_value.as_ref() {
                                    CLanguageObject::NumberLiteral(NumberLiteral { value, .. })
                                        if value == "3" => {}
                                    _ => panic!(),
                                }
                                match c_assignment_value.as_ref() {
                                    CLanguageObject::BinaryExpression(BinaryExpression {
                                        left,
                                        operator,
                                        right,
                                        ..
                                    }) if *operator == "+" => {
                                        match left.as_ref() {
                                            CLanguageObject::Reference(Reference {
                                                identifier,
                                                declaration_id,
                                                ..
                                            }) if identifier == "a"
                                                && declaration_id == inner_a_declaration_id => {}
                                            _ => panic!(),
                                        }
                                        match right.as_ref() {
                                            CLanguageObject::Reference(Reference {
                                                identifier,
                                                declaration_id,
                                                ..
                                            }) if identifier == "b" && declaration_id == b_declaration_id => {}
                                            _ => panic!(),
                                        }
                                    }
                                    _ => panic!(),
                                }
                            }
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_include() {
        let c_code = "
    #include <stdio.h>
    ";

        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        let expected: Vec<CLanguageObject> =
            vec![CLanguageObject::PreprocInclude(PreprocInclude {
                id: Uuid::new_v4(),
                content: "<stdio.h>".to_string(),
            })];
        assert_eq!(src_file.code, expected);
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_function_definition() {
        let c_code = "
            int first(int a, int b, int c)
            {
                return a;
            }
        ";
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    return_type: CType::Int,
                    identifier,
                    parameter_list,
                    compound_statement: CompoundStatement { code_block, .. },
                    ..
                }),
            ] => {
                assert_eq!(identifier, "first");
                match parameter_list.as_slice() {
                    [
                        FunctionParameter {
                            id: param_a_id,
                            identifier: param_a_identifier,
                            param_type: CType::Int,
                            ..
                        },
                        FunctionParameter {
                            identifier: param_b_identifier,
                            param_type: CType::Int,
                            ..
                        },
                        FunctionParameter {
                            identifier: param_c_identifier,
                            param_type: CType::Int,
                            ..
                        },
                    ] => {
                        assert_eq!(param_a_identifier, "a");
                        assert_eq!(param_b_identifier, "b");
                        assert_eq!(param_c_identifier, "c");

                        match code_block.as_slice() {
                            [CLanguageObject::ReturnStatement(ReturnStatement { value, .. })] => {
                                match &**value {
                                    CLanguageObject::Reference(Reference { declaration_id, identifier , ..}) => {
                                        assert_eq!(declaration_id, param_a_id);
                                        assert_eq!(identifier, "a");
                                    }
                                    _ => {
                                        panic!("AST did not match expected return statement value")
                                    }
                                }
                            }
                            _ => panic!("AST did not match expected return statement"),
                        }
                    }
                    _ => panic!("AST did not match expected function parameters"),
                }
            }
            _ => panic!("AST did not match expected function definition"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_fn_call() {
        let c_code = "
    int first (int a, int b) {
        return a;
    }
    
    int main() {
        int result = first(1, 2);
    }
    ";
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    id: first_id,
                    identifier: first_identifier,
                    ..
                }),
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    return_type: CType::Int,
                    identifier,
                    parameter_list,
                    compound_statement: CompoundStatement { code_block, .. },
                    ..
                }),
            ] => {
                assert_eq!(first_identifier, "first");
                assert_eq!(identifier, "main");
                assert!(parameter_list.is_empty());
                match code_block.as_slice() {
                    [
                        CLanguageObject::Declaration(Declaration {
                            primitive_type: CType::Int,
                            identifier: result_identifier,
                            value: Some(value),
                            ..
                        }),
                    ] => {
                        assert_eq!(result_identifier, "result");
                        match value.as_ref() {
                            CLanguageObject::CallExpression(CallExpression {
                                id_declaration: call_id,
                                identifier: call_identifier,
                                argument_list,
                                ..
                            }) => {
                                assert_eq!(call_identifier, "first");
                                assert_eq!(call_id, first_id);
                                match argument_list.as_slice() {
                                    [
                                        CLanguageObject::NumberLiteral(NumberLiteral { value: a, .. }),
                                        CLanguageObject::NumberLiteral(NumberLiteral { value: b, .. }),
                                    ] => {
                                        assert_eq!(a, "1");
                                        assert_eq!(b, "2");
                                    }
                                    _ => {
                                        panic!("AST did not match expected function call argument")
                                    }
                                }
                            }
                            _ => panic!("AST did not match expected function call"),
                        }
                    }
                    _ => {
                        panic!("AST did not match expected variable declaration with function call")
                    }
                }
            }
            _ => panic!("AST did not match expected function definition"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_statement() {
        let c_code = r#"
    int main() {
        printf("Hello, World!\n");
    }
    "#;
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    return_type: CType::Int,
                    identifier,
                    parameter_list,
                    compound_statement: CompoundStatement { code_block, .. },
                    ..
                }),
            ] => {
                assert_eq!(identifier, "main");
                assert!(parameter_list.is_empty());
                match code_block.as_slice() {
                    [
                        CLanguageObject::CallExpression(CallExpression {
                            id: _,
                            id_declaration: _, // TODO verify id with id assigned to stdlib function
                            identifier: call_identifier,
                            argument_list,
                        }),
                    ] => {
                        assert_eq!(call_identifier, "printf");
                        match argument_list.as_slice() {
                            [CLanguageObject::StringLiteral(StringLiteral { value, .. })]
                                if value == "Hello, World!\\n" => {}
                            _ => {
                                panic!("AST did not match expected function call argument list")
                            }
                        }
                    }
                    _ => panic!(
                        "AST did not match expected expression statement: {:?}",
                        code_block
                    ),
                }
            }
            _ => panic!("AST did not match expected function definition"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_comparisons() {
        let c_code = r#"
                int main() {
                    1 == 2;
                    1 < 2;
                    1 > 2;
                    1 != 2;
                    1 <= 2;
                    1 >= 2;
                }
                "#;
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        let code: Vec<CLanguageObject> = vec![
            CLanguageObject::BinaryExpression(BinaryExpression {
                id: Uuid::new_v4(),
                left: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                    id: Uuid::new_v4(),
                    value: "1".to_string(),
                })),
                operator: "==".to_string(),
                right: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                    id: Uuid::new_v4(),
                    value: "2".to_string(),
                })),
            }),
            CLanguageObject::BinaryExpression(BinaryExpression {
                id: Uuid::new_v4(),
                left: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                    id: Uuid::new_v4(),
                    value: "1".to_string(),
                })),
                operator: "<".to_string(),
                right: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                    id: Uuid::new_v4(),
                    value: "2".to_string(),
                })),
            }),
            CLanguageObject::BinaryExpression(BinaryExpression {
                id: Uuid::new_v4(),
                left: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                    id: Uuid::new_v4(),
                    value: "1".to_string(),
                })),
                operator: ">".to_string(),
                right: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                    id: Uuid::new_v4(),
                    value: "2".to_string(),
                })),
            }),
            CLanguageObject::BinaryExpression(BinaryExpression {
                id: Uuid::new_v4(),
                left: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                    id: Uuid::new_v4(),
                    value: "1".to_string(),
                })),
                operator: "!=".to_string(),
                right: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                    id: Uuid::new_v4(),
                    value: "2".to_string(),
                })),
            }),
            CLanguageObject::BinaryExpression(BinaryExpression {
                id: Uuid::new_v4(),
                left: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                    id: Uuid::new_v4(),
                    value: "1".to_string(),
                })),
                operator: "<=".to_string(),
                right: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                    id: Uuid::new_v4(),
                    value: "2".to_string(),
                })),
            }),
            CLanguageObject::BinaryExpression(BinaryExpression {
                id: Uuid::new_v4(),
                left: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                    id: Uuid::new_v4(),
                    value: "1".to_string(),
                })),
                operator: ">=".to_string(),
                right: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                    id: Uuid::new_v4(),
                    value: "2".to_string(),
                })),
            }),
        ];

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    return_type: CType::Int,
                    identifier,
                    parameter_list,
                    compound_statement: CompoundStatement { code_block, .. },
                    ..
                }),
            ] => {
                assert_eq!(identifier, "main");
                assert!(parameter_list.is_empty());
                assert_eq!(code_block, &code);
            }
            _ => panic!("AST did not match expected function definition"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_if_statement() {
        let c_code = r#"
                int main() {
                    if (5 > 0) {
                        int a;
                    }
                }
                "#;
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    return_type: CType::Int,
                    identifier,
                    parameter_list,
                    compound_statement: CompoundStatement { code_block, .. },
                    ..
                }),
            ] => {
                assert_eq!(identifier, "main");
                assert!(parameter_list.is_empty());
                match code_block.as_slice() {
                    [
                        CLanguageObject::IfStatement(IfStatement {
                            condition,
                            compound_statement: CompoundStatement { code_block, .. },
                            else_clause: None,
                            ..
                        }),
                    ] => {
                        assert_eq!(
                            condition.as_ref(),
                            &CLanguageObject::BinaryExpression(BinaryExpression {
                                id: Uuid::new_v4(),
                                left: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                                    id: Uuid::new_v4(),
                                    value: "5".to_string(),
                                })),
                                operator: ">".to_string(),
                                right: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                                    id: Uuid::new_v4(),
                                    value: "0".to_string(),
                                })),
                            })
                        );
                        match code_block.as_slice() {
                            [
                                CLanguageObject::Declaration(Declaration {
                                    primitive_type: CType::Int,
                                    identifier: a_identifier,
                                    value: None,
                                    ..
                                }),
                            ] => {
                                assert_eq!(a_identifier, "a");
                            }
                            _ => panic!("AST did not match expected if statement body"),
                        }
                    }
                    _ => panic!("AST did not match expected if statement"),
                }
            }
            _ => panic!("AST did not match expected function definition"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_if_else_statement() {
        let c_code = r#"
                int main() {
                    if (5 > 0) {
                        int a;
                    } else {
                        int b;
                    }
                }
                "#;
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    return_type: CType::Int,
                    identifier,
                    parameter_list,
                    compound_statement: CompoundStatement { code_block, .. },
                    ..
                }),
            ] => {
                assert_eq!(identifier, "main");
                assert!(parameter_list.is_empty());
                match code_block.as_slice() {
                    [
                        CLanguageObject::IfStatement(IfStatement {
                            condition,
                            compound_statement: CompoundStatement { code_block, .. },
                            else_clause:
                                Some(ElseClause {
                                    condition: None,
                                    compound_statement:
                                        CompoundStatement {
                                            code_block: else_code_block,
                                            ..
                                        },
                                    ..
                                }),
                            ..
                        }),
                    ] => {
                        assert_eq!(
                            condition.as_ref(),
                            &CLanguageObject::BinaryExpression(BinaryExpression {
                                id: Uuid::new_v4(),
                                left: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                                    id: Uuid::new_v4(),
                                    value: "5".to_string(),
                                })),
                                operator: ">".to_string(),
                                right: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                                    id: Uuid::new_v4(),
                                    value: "0".to_string(),
                                })),
                            })
                        );
                        match code_block.as_slice() {
                            [
                                CLanguageObject::Declaration(Declaration {
                                    primitive_type: CType::Int,
                                    identifier: a_identifier,
                                    value: None,
                                    ..
                                }),
                            ] => {
                                assert_eq!(a_identifier, "a");
                            }
                            _ => panic!("AST did not match expected if statement body"),
                        }
                        match else_code_block.as_slice() {
                            [
                                CLanguageObject::Declaration(Declaration {
                                    primitive_type: CType::Int,
                                    identifier: b_identifier,
                                    value: None,
                                    ..
                                }),
                            ] => {
                                assert_eq!(b_identifier, "b");
                            }
                            _ => panic!("AST did not match expected else statement body"),
                        }
                    }
                    _ => panic!("AST did not match expected if statement"),
                }
            }
            _ => panic!("AST did not match expected function definition"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_if_else_if_statement() {
        let c_code = r#"
                int main() {
                    if (5 > 0) {
                        int a;
                    } else if (5 < 0) {
                        int b;
                    }
                }
                "#;
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    return_type: CType::Int,
                    identifier,
                    parameter_list,
                    compound_statement: CompoundStatement { code_block, .. },
                    ..
                }),
            ] => {
                assert_eq!(identifier, "main");
                assert!(parameter_list.is_empty());
                match code_block.as_slice() {
                    [
                        CLanguageObject::IfStatement(IfStatement {
                            condition,
                            compound_statement: CompoundStatement { code_block, .. },
                            else_clause:
                                Some(ElseClause {
                                    condition: Some(else_condition),
                                    compound_statement:
                                        CompoundStatement {
                                            code_block: else_code_block,
                                            ..
                                        },
                                    ..
                                }),
                            ..
                        }),
                    ] => {
                        assert_eq!(
                            condition.as_ref(),
                            &CLanguageObject::BinaryExpression(BinaryExpression {
                                id: Uuid::new_v4(),
                                left: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                                    id: Uuid::new_v4(),
                                    value: "5".to_string(),
                                })),
                                operator: ">".to_string(),
                                right: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                                    id: Uuid::new_v4(),
                                    value: "0".to_string(),
                                })),
                            })
                        );
                        match code_block.as_slice() {
                            [
                                CLanguageObject::Declaration(Declaration {
                                    primitive_type: CType::Int,
                                    identifier: a_identifier,
                                    value: None,
                                    ..
                                }),
                            ] => {
                                assert_eq!(a_identifier, "a");
                            }
                            _ => panic!("AST did not match expected if statement body"),
                        }
                        assert_eq!(
                            else_condition.as_ref(),
                            &CLanguageObject::BinaryExpression(BinaryExpression {
                                id: Uuid::new_v4(),
                                left: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                                    id: Uuid::new_v4(),
                                    value: "5".to_string(),
                                })),
                                operator: "<".to_string(),
                                right: Box::new(CLanguageObject::NumberLiteral(NumberLiteral {
                                    id: Uuid::new_v4(),
                                    value: "0".to_string(),
                                })),
                            })
                        );
                        match else_code_block.as_slice() {
                            [
                                CLanguageObject::Declaration(Declaration {
                                    primitive_type: CType::Int,
                                    identifier: b_identifier,
                                    value: None,
                                    ..
                                }),
                            ] => {
                                assert_eq!(b_identifier, "b");
                            }
                            _ => panic!("AST did not match expected else statement body"),
                        }
                    }
                    _ => panic!("AST did not match expected if statement"),
                }
            }
            _ => panic!("AST did not match expected function definition"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_recursive_function() {
        let c_code = r#"
        int test(int a) {
            return test(a);
        }
        "#;
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    id: definition_id,
                    identifier: definition_indentifier,
                    compound_statement: CompoundStatement { code_block, .. },
                    ..
                }),
            ] => {
                assert_eq!(definition_indentifier, "test");

                match code_block.as_slice() {
                    [
                        CLanguageObject::ReturnStatement(ReturnStatement {
                            value: return_value,
                            ..
                        }),
                    ] => match return_value.as_ref() {
                        CLanguageObject::CallExpression(CallExpression {
                            id_declaration,
                            identifier: call_identifier,
                            ..
                        }) => {
                            assert_eq!(id_declaration, definition_id);
                            assert_eq!(call_identifier, "test");
                        }
                        _ => panic!("AST did not match expected return statement"),
                    },
                    _ => panic!("AST did not match expected function body"),
                }
            }
            _ => panic!("AST did not match expected function definition"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }

    #[test]
    fn test_parse_function_and_variable_with_same_name() {
        let c_code = r#"
        int test(int a) {
            int test = a;
            return test;
        }
        "#;
        let c_language = C::new();
        let src_file = c_language.parse_text(c_code).unwrap();

        match src_file.code.as_slice() {
            [
                CLanguageObject::FunctionDefinition(FunctionDefinition {
                    id: function_id,
                    identifier: function_identifier,
                    compound_statement: CompoundStatement { code_block, .. },
                    ..
                }),
            ] => {
                assert_eq!(function_identifier, "test");

                match code_block.as_slice() {
                    [
                        CLanguageObject::Declaration(Declaration {
                            id: variable_id,
                            identifier: variable_identifier,
                            ..
                        }),
                        CLanguageObject::ReturnStatement(ReturnStatement {
                            value: return_value,
                            ..
                        }),
                    ] => {
                        assert_eq!(variable_identifier, "test");
                        assert_ne!(variable_id, function_id);
                        match return_value.as_ref() {
                            CLanguageObject::Reference(Reference { declaration_id, identifier, .. }) => {
                                assert_eq!(declaration_id, variable_id);
                                assert_eq!(identifier, "test");
                            }
                            _ => panic!("AST did not match expected return statement value"),
                        }
                    }
                    _ => panic!("AST did not match expected function body"),
                }
            }
            _ => panic!("AST did not match expected function definitions"),
        }
        let nodes = c_language.write_to_nodes(src_file.clone()).unwrap();
        let parsed_objects = c_language.parse_nodes(nodes).unwrap();
        assert_eq!(src_file, parsed_objects);
    }
}
