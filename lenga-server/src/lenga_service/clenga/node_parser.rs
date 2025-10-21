use std::{fs::File, io::Read};

use language::language::{
    Language,
    c::{self, C},
};

use crate::lenga_service::clenga::proto;

pub fn parse_file(path: &str) -> proto::SourceFile {
    let file = File::open(&path).unwrap();

    let content: Vec<u8> = file.bytes().map(|b| b.unwrap()).collect();

    let c = C::new();
    let src_file = c.parse_nodes(content).unwrap();
    source_file_to_proto(src_file)
}

pub fn c_language_object_to_proto(
    c_object: c::language_object::LanguageObject,
) -> proto::LanguageObject {
    match c_object {
        c::language_object::LanguageObject::SourceFile(source_file) => {
            let source_file_msg = source_file_to_proto(source_file);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::SourceFile(
                    source_file_msg,
                )),
            }
        }
        c::language_object::LanguageObject::AssignmentExpression(assignment_expression) => {
            let assignment_expression_msg = assignment_expression_to_proto(assignment_expression);
            proto::LanguageObject {
                language_object: Some(
                    proto::language_object::LanguageObject::AssignmentExpression(
                        assignment_expression_msg,
                    ),
                ),
            }
        }
        c::language_object::LanguageObject::BinaryExpression(binary_expression) => {
            let binary_expression_msg = binary_expression_to_proto(binary_expression);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::BinaryExpression(
                    binary_expression_msg,
                )),
            }
        }
        c::language_object::LanguageObject::CallExpression(call_expression) => {
            let call_expression_msg = call_expression_to_proto(call_expression);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::CallExpression(
                    call_expression_msg,
                )),
            }
        }
        c::language_object::LanguageObject::Comment(comment) => {
            let comment_msg = comment_to_proto(comment);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::Comment(comment_msg)),
            }
        }
        c::language_object::LanguageObject::Declaration(declaration) => {
            let declaration_msg = declaration_to_proto(declaration);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::Declaration(
                    declaration_msg,
                )),
            }
        }
        c::language_object::LanguageObject::ElseClause(else_clause) => {
            let else_clause_msg = else_clause_to_proto(else_clause);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::ElseClause(
                    else_clause_msg,
                )),
            }
        }
        c::language_object::LanguageObject::FunctionDeclaration(function_declaration) => {
            let function_declaration_msg = function_declaration_to_proto(function_declaration);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::FunctionDeclaration(
                    function_declaration_msg,
                )),
            }
        }
        c::language_object::LanguageObject::FunctionDefinition(function_definition) => {
            let function_definition_msg = function_definition_to_proto(function_definition);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::FunctionDefinition(
                    function_definition_msg,
                )),
            }
        }
        c::language_object::LanguageObject::FunctionParameter(function_parameter) => {
            let function_parameter_msg = function_parameter_to_proto(function_parameter);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::FunctionParameter(
                    function_parameter_msg,
                )),
            }
        }
        c::language_object::LanguageObject::IfStatement(if_statement) => {
            let if_statement_msg = if_statement_to_proto(if_statement);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::IfStatement(
                    if_statement_msg,
                )),
            }
        }
        c::language_object::LanguageObject::NumberLiteral(number_literal) => {
            let number_literal_msg = number_literal_to_proto(number_literal);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::NumberLiteral(
                    number_literal_msg,
                )),
            }
        }
        c::language_object::LanguageObject::PreprocInclude(preproc_include) => {
            let preproc_include_msg = preproc_include_to_proto(preproc_include);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::PreprocInclude(
                    preproc_include_msg,
                )),
            }
        }
        c::language_object::LanguageObject::Reference(reference) => {
            let reference_msg = reference_to_proto(reference);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::Reference(
                    reference_msg,
                )),
            }
        }
        c::language_object::LanguageObject::ReturnStatement(return_statement) => {
            let return_statement_msg = return_statement_to_proto(return_statement);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::ReturnStatement(
                    return_statement_msg,
                )),
            }
        }
        c::language_object::LanguageObject::StringLiteral(string_literal) => {
            let string_literal_msg = string_literal_to_proto(string_literal);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::StringLiteral(
                    string_literal_msg,
                )),
            }
        }
        c::language_object::LanguageObject::CompoundStatement(compound_statement) => {
            let compound_statement_msg = compound_statement_to_proto(compound_statement);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::CompoundStatement(
                    compound_statement_msg,
                )),
            }
        }
        c::language_object::LanguageObject::Unknown(unknown) => {
            let unknown_msg = unknown_to_proto(unknown);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::Unknown(unknown_msg)),
            }
        }
    }
}
pub fn source_file_to_proto(
    src_file: c::language_object::special_object::source_file::SourceFile,
) -> proto::SourceFile {
    let mut code: Vec<proto::DeclarationObject> = Vec::new();
    for o in src_file.code {
        code.push(c_declaration_object_to_proto(o))
    }

    proto::SourceFile {
        id: src_file.id.to_string(),
        code,
    }
}

fn c_declaration_object_to_proto(
    declaration_object: c::language_object::declaration_object::DeclarationObject,
) -> proto::DeclarationObject {
    match declaration_object {
        c::language_object::declaration_object::DeclarationObject::Declaration(declaration) => {
            proto::DeclarationObject {
                declaration_object: Some(
                    proto::declaration_object::DeclarationObject::Declaration(
                        declaration_to_proto(declaration),
                    ),
                ),
            }
        }
        c::language_object::declaration_object::DeclarationObject::FunctionDeclaration(
            function_declaration,
        ) => proto::DeclarationObject {
            declaration_object: Some(
                proto::declaration_object::DeclarationObject::FunctionDeclaration(
                    function_declaration_to_proto(function_declaration),
                ),
            ),
        },
        c::language_object::declaration_object::DeclarationObject::FunctionDefinition(
            function_definition,
        ) => proto::DeclarationObject {
            declaration_object: Some(
                proto::declaration_object::DeclarationObject::FunctionDefinition(
                    function_definition_to_proto(function_definition),
                ),
            ),
        },
        c::language_object::declaration_object::DeclarationObject::PreprocInclude(
            preproc_include,
        ) => proto::DeclarationObject {
            declaration_object: Some(
                proto::declaration_object::DeclarationObject::PreprocInclude(
                    preproc_include_to_proto(preproc_include),
                ),
            ),
        },
        c::language_object::declaration_object::DeclarationObject::Comment(comment) => {
            proto::DeclarationObject {
                declaration_object: Some(proto::declaration_object::DeclarationObject::Comment(
                    comment_to_proto(comment),
                )),
            }
        }
        c::language_object::declaration_object::DeclarationObject::Unknown(unknown) => {
            proto::DeclarationObject {
                declaration_object: Some(proto::declaration_object::DeclarationObject::Unknown(
                    unknown_to_proto(unknown),
                )),
            }
        }
    }
}

fn assignment_expression_to_proto(
    assignment_expression: c::language_object::expression_object::assignment_expression::AssignmentExpression,
) -> proto::AssignmentExpression {
    let value_proto = Some(Box::new(c_expression_object_to_proto(
        *assignment_expression.value,
    )));

    proto::AssignmentExpression {
        id: assignment_expression.id.to_string(),
        id_declaration: assignment_expression.id_declaration.to_string(),
        identifier: assignment_expression.identifier,
        value: value_proto,
    }
}

fn c_expression_object_to_proto(
    expression_object: c::language_object::expression_object::ExpressionObject,
) -> proto::ExpressionObject {
    match expression_object {
        c::language_object::expression_object::ExpressionObject::AssignmentExpression(
            assignment_expression,
        ) => proto::ExpressionObject {
            expression_object: Some(
                proto::expression_object::ExpressionObject::AssignmentExpression(Box::new(
                    assignment_expression_to_proto(assignment_expression),
                )),
            ),
        },
        c::language_object::expression_object::ExpressionObject::BinaryExpression(
            binary_expression,
        ) => proto::ExpressionObject {
            expression_object: Some(
                proto::expression_object::ExpressionObject::BinaryExpression(Box::new(
                    binary_expression_to_proto(binary_expression),
                )),
            ),
        },
        c::language_object::expression_object::ExpressionObject::CallExpression(
            call_expression,
        ) => proto::ExpressionObject {
            expression_object: Some(proto::expression_object::ExpressionObject::CallExpression(
                call_expression_to_proto(call_expression),
            )),
        },
        c::language_object::expression_object::ExpressionObject::NumberLiteral(number_literal) => {
            proto::ExpressionObject {
                expression_object: Some(proto::expression_object::ExpressionObject::NumberLiteral(
                    number_literal_to_proto(number_literal),
                )),
            }
        }
        c::language_object::expression_object::ExpressionObject::Reference(reference) => {
            proto::ExpressionObject {
                expression_object: Some(proto::expression_object::ExpressionObject::Reference(
                    reference_to_proto(reference),
                )),
            }
        }
        c::language_object::expression_object::ExpressionObject::StringLiteral(string_literal) => {
            proto::ExpressionObject {
                expression_object: Some(proto::expression_object::ExpressionObject::StringLiteral(
                    string_literal_to_proto(string_literal),
                )),
            }
        }
        c::language_object::expression_object::ExpressionObject::Unknown(unknown) => {
            proto::ExpressionObject {
                expression_object: Some(proto::expression_object::ExpressionObject::Unknown(
                    unknown_to_proto(unknown),
                )),
            }
        }
    }
}

fn binary_expression_to_proto(
    binary_expression: c::language_object::expression_object::binary_expression::BinaryExpression,
) -> proto::BinaryExpression {
    let left_proto = Some(Box::new(c_expression_object_to_proto(
        *binary_expression.left,
    )));

    let right_proto = Some(Box::new(c_expression_object_to_proto(
        *binary_expression.right,
    )));

    proto::BinaryExpression {
        id: binary_expression.id.to_string(),
        left: left_proto,
        operator: binary_expression.operator,
        right: right_proto,
    }
}

fn call_expression_to_proto(
    call_expression: c::language_object::expression_object::call_expression::CallExpression,
) -> proto::CallExpression {
    let mut argument_list: Vec<proto::ExpressionObject> = Vec::new();
    for argument in call_expression.argument_list {
        argument_list.push(c_expression_object_to_proto(argument));
    }

    proto::CallExpression {
        id: call_expression.id.to_string(),
        id_declaration: call_expression.id_declaration.to_string(),
        identifier: call_expression.identifier,
        argument_list: argument_list,
    }
}

fn comment_to_proto(
    comment: c::language_object::special_object::comment::Comment,
) -> proto::Comment {
    proto::Comment {
        id: comment.id.to_string(),
        content: comment.content,
    }
}

fn declaration_to_proto(
    declaration: c::language_object::declaration_object::declaration::Declaration,
) -> proto::Declaration {
    let value = declaration
        .value
        .map(|value| c_expression_object_to_proto(*value));

    proto::Declaration {
        id: declaration.id.to_string(),
        primitive_type: declaration.primitive_type.as_str().to_string(),
        identifier: declaration.identifier,
        value,
    }
}

fn else_clause_to_proto(
    else_clause: c::language_object::statement_object::if_statement::else_clause::ElseClause,
) -> proto::ElseClause {
    let condition = else_clause
        .condition
        .map(|condition| c_expression_object_to_proto(*condition));
    let compound_statement = c_statement_object_to_proto(*else_clause.compound_statement);

    proto::ElseClause {
        id: else_clause.id.to_string(),
        condition: condition,
        compound_statement: Some(Box::new(compound_statement)),
    }
}

fn c_statement_object_to_proto(
    statement_object: c::language_object::statement_object::StatementObject,
) -> proto::StatementObject {
    match statement_object {
        c::language_object::statement_object::StatementObject::CompoundStatement(
            compound_statement,
        ) => {
            let compound_statement_msg = compound_statement_to_proto(compound_statement);
            proto::StatementObject {
                statement_object: Some(
                    proto::statement_object::StatementObject::CompoundStatement(
                        compound_statement_msg,
                    ),
                ),
            }
        }
        c::language_object::statement_object::StatementObject::IfStatement(if_statement) => {
            let if_statement_msg = if_statement_to_proto(if_statement);
            proto::StatementObject {
                statement_object: Some(proto::statement_object::StatementObject::IfStatement(
                    Box::new(if_statement_msg),
                )),
            }
        }
        c::language_object::statement_object::StatementObject::ReturnStatement(
            return_statement,
        ) => {
            let return_statement_msg = return_statement_to_proto(return_statement);
            proto::StatementObject {
                statement_object: Some(proto::statement_object::StatementObject::ReturnStatement(
                    return_statement_msg,
                )),
            }
        }
        c::language_object::statement_object::StatementObject::Unknown(unknown) => {
            let unknown_msg = unknown_to_proto(unknown);
            proto::StatementObject {
                statement_object: Some(proto::statement_object::StatementObject::Unknown(
                    unknown_msg,
                )),
            }
        }
    }
}

fn function_declaration_to_proto(
    function_declaration: c::language_object::declaration_object::function_declaration::FunctionDeclaration,
) -> proto::FunctionDeclaration {
    let mut parameter_list: Vec<proto::FunctionParameter> = Vec::new();
    for parameter in function_declaration.parameter_list {
        parameter_list.push(function_parameter_to_proto(parameter));
    }

    proto::FunctionDeclaration {
        id: function_declaration.id.to_string(),
        return_type: function_declaration.return_type.as_str().to_string(),
        identifier: function_declaration.identifier,
        parameter_list: parameter_list,
    }
}

fn function_definition_to_proto(
    function_definition: c::language_object::declaration_object::function_definition::FunctionDefinition,
) -> proto::FunctionDefinition {
    let mut parameter_list: Vec<proto::FunctionParameter> = Vec::new();
    for parameter in function_definition.parameter_list {
        parameter_list.push(function_parameter_to_proto(parameter));
    }

    let compound_statement = compound_statement_to_proto(function_definition.compound_statement);

    proto::FunctionDefinition {
        id: function_definition.id.to_string(),
        return_type: function_definition.return_type.as_str().to_string(),
        identifier: function_definition.identifier,
        parameter_list: parameter_list,
        compound_statement: Some(compound_statement),
    }
}

fn function_parameter_to_proto(
    function_parameter: c::language_object::declaration_object::function_declaration::function_parameter::FunctionParameter,
) -> proto::FunctionParameter {
    proto::FunctionParameter {
        id: function_parameter.id.to_string(),
        identifier: function_parameter.identifier,
        param_type: function_parameter.param_type.as_str().to_string(),
    }
}

fn if_statement_to_proto(
    if_statement: c::language_object::statement_object::if_statement::IfStatement,
) -> proto::IfStatement {
    let condition = c_expression_object_to_proto(*if_statement.condition);
    let compound_statement = c_statement_object_to_proto(*if_statement.compound_statement);
    let else_clause = if_statement
        .else_statement
        .map(|else_clause| Box::new(else_clause_to_proto(else_clause)));

    proto::IfStatement {
        id: if_statement.id.to_string(),
        condition: Some(condition),
        compound_statement: Some(Box::new(compound_statement)),
        else_clause: else_clause,
    }
}

fn number_literal_to_proto(
    number_literal: c::language_object::expression_object::number_literal::NumberLiteral,
) -> proto::NumberLiteral {
    proto::NumberLiteral {
        id: number_literal.id.to_string(),
        value: number_literal.value,
    }
}

fn preproc_include_to_proto(
    preproc_include: c::language_object::declaration_object::preproc_include::PreprocInclude,
) -> proto::PreprocInclude {
    proto::PreprocInclude {
        id: preproc_include.id.to_string(),
        content: preproc_include.content,
    }
}

fn reference_to_proto(
    reference: c::language_object::expression_object::reference::Reference,
) -> proto::Reference {
    proto::Reference {
        id: reference.id.to_string(),
        declaration_id: reference.declaration_id.to_string(),
        identifier: reference.identifier,
    }
}

fn return_statement_to_proto(
    return_statement: c::language_object::statement_object::return_statement::ReturnStatement,
) -> proto::ReturnStatement {
    let value = c_expression_object_to_proto(*return_statement.value);

    proto::ReturnStatement {
        id: return_statement.id.to_string(),
        value: Some(value),
    }
}

fn string_literal_to_proto(
    string_literal: c::language_object::expression_object::string_literal::StringLiteral,
) -> proto::StringLiteral {
    proto::StringLiteral {
        id: string_literal.id.to_string(),
        value: string_literal.value,
    }
}

fn compound_statement_to_proto(
    compound_statement: c::language_object::statement_object::compound_statement::CompoundStatement,
) -> proto::CompoundStatement {
    let mut code_block: Vec<proto::CompoundStatementObject> = Vec::new();
    for statement in compound_statement.code_block {
        code_block.push(c_compound_statement_object_to_proto(statement));
    }

    proto::CompoundStatement {
        id: compound_statement.id.to_string(),
        code_block: code_block,
    }
}

fn c_compound_statement_object_to_proto(
    statement: c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject,
) -> proto::CompoundStatementObject {
    match statement {
        c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::Declaration(declaration) => {
                        proto::CompoundStatementObject {
                            compound_statement_object: Some(proto::compound_statement_object::CompoundStatementObject::Declaration(
                                declaration_to_proto(declaration),
                            )),
                        }
            }
        c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::IfStatement(if_statement) => {
                let if_statement_msg = if_statement_to_proto(if_statement);
                proto::CompoundStatementObject {
                    compound_statement_object: Some(proto::compound_statement_object::CompoundStatementObject::IfStatement(
                        if_statement_msg,
                    )),
                }
            }
        c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::ReturnStatement(return_statement) => {
                let return_statement_msg = return_statement_to_proto(return_statement);
                proto::CompoundStatementObject {
                    compound_statement_object: Some(proto::compound_statement_object::CompoundStatementObject::ReturnStatement(
                        return_statement_msg,
                    )),
                }
            }
        c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::Unknown(unknown) => {
                let unknown_msg = unknown_to_proto(unknown);
                proto::CompoundStatementObject {
                    compound_statement_object: Some(proto::compound_statement_object::CompoundStatementObject::Unknown(
                        unknown_msg,
                    )),
                }
            }
        c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::AssignmentExpression(assignment_expression) => {
                let assignment_expression_msg = assignment_expression_to_proto(assignment_expression);
                proto::CompoundStatementObject {
                    compound_statement_object: Some(proto::compound_statement_object::CompoundStatementObject::AssignmentExpression(
                        assignment_expression_msg,
                    )),
                }
        },
        c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::BinaryExpression(binary_expression) => {
            let binary_expression_msg = binary_expression_to_proto(binary_expression);
            proto::CompoundStatementObject {
                compound_statement_object: Some(proto::compound_statement_object::CompoundStatementObject::BinaryExpression(
                    binary_expression_msg,
                )),
            }
        },
        c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::CallExpression(call_expression) => {
            let call_expression_msg = call_expression_to_proto(call_expression);
            proto::CompoundStatementObject {
                compound_statement_object: Some(proto::compound_statement_object::CompoundStatementObject::CallExpression(
                    call_expression_msg,
                )),
            }
        },
        c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::NumberLiteral(number_literal) => {
            let number_literal_msg = number_literal_to_proto(number_literal);
            proto::CompoundStatementObject {
                compound_statement_object: Some(proto::compound_statement_object::CompoundStatementObject::NumberLiteral(
                    number_literal_msg,
                )),
            }
        },
        c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::Reference(reference) => {
            let reference_msg = reference_to_proto(reference);
            proto::CompoundStatementObject {
                compound_statement_object: Some(proto::compound_statement_object::CompoundStatementObject::Reference(
                    reference_msg,
                )),
            }
        },
        c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::StringLiteral(string_literal) => {
            let string_literal_msg = string_literal_to_proto(string_literal);
            proto::CompoundStatementObject {
                compound_statement_object: Some(proto::compound_statement_object::CompoundStatementObject::StringLiteral(
                    string_literal_msg,
                )),
            }
        },
        c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::CompoundStatement(compound_statement) => {
            let compound_statement_msg = compound_statement_to_proto(compound_statement);
            proto::CompoundStatementObject {
                compound_statement_object: Some(proto::compound_statement_object::CompoundStatementObject::CompoundStatement(
                    compound_statement_msg,
                )),
            }
        },
        c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::Comment(comment) => {
            let comment_msg = comment_to_proto(comment);
            proto::CompoundStatementObject {
                compound_statement_object: Some(proto::compound_statement_object::CompoundStatementObject::Comment(
                    comment_msg,
                )),
            }
        },
    }
}

fn unknown_to_proto(
    unknown: c::language_object::special_object::unknown::Unknown,
) -> proto::Unknown {
    proto::Unknown {
        id: unknown.id.to_string(),
        content: unknown.content,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_01_source_file_with_empty_code() {
        let id = Uuid::new_v4();
        let src = c::language_object::special_object::source_file::SourceFile { id, code: vec![] };
        let proto_src = source_file_to_proto(src);

        assert_eq!(proto_src.id, id.to_string());
        assert!(proto_src.code.is_empty());
    }

    #[test]
    fn test_02_source_file_with_code_elements() {
        let comment_id = Uuid::new_v4();
        let content = "this is a comment";
        let comment = c::language_object::special_object::comment::Comment {
            id: comment_id,
            content: content.to_string(),
        };

        let id = Uuid::new_v4();
        let src = c::language_object::special_object::source_file::SourceFile {
            id,
            code: vec![c::language_object::declaration_object::DeclarationObject::Comment(comment)],
        };
        let proto_src = source_file_to_proto(src);

        assert_eq!(proto_src.id, id.to_string());
        match &proto_src.code[0].declaration_object {
            Some(proto::declaration_object::DeclarationObject::Comment(proto_comment)) => {
                assert_eq!(proto_comment.id, comment_id.to_string());
                assert_eq!(proto_comment.content, content);
            }
            _ => panic!("expected Some(Comment)"),
        }
    }

    #[test]
    fn test_03_assignment_expression() {
        let number_id = Uuid::new_v4();
        let value = "42";

        let id = Uuid::new_v4();
        let id_declaration = Uuid::new_v4();
        let identifier = "test";
        let assignment =
            c::language_object::expression_object::assignment_expression::AssignmentExpression {
                id,
                id_declaration,
                identifier: identifier.to_string(),
                value: Box::new(
                    c::language_object::expression_object::ExpressionObject::NumberLiteral(
                        c::language_object::expression_object::number_literal::NumberLiteral {
                            id: number_id,
                            value: value.to_string(),
                        },
                    ),
                ),
            };
        let proto_assignment = assignment_expression_to_proto(assignment);

        assert_eq!(proto_assignment.id, id.to_string());
        assert_eq!(proto_assignment.identifier, identifier);

        if let Some(inner) = &proto_assignment.value {
            match &inner.expression_object {
                Some(proto::expression_object::ExpressionObject::NumberLiteral(proto_number)) => {
                    assert_eq!(proto_number.id, number_id.to_string());
                    assert_eq!(proto_number.value, value);
                }
                _ => panic!("expected Some(NumberLiteral)"),
            }
        } else {
            panic!("expected Some(Box<NumberLiteral>)");
        }
    }

    #[test]
    fn test_04_binary_expression() {
        let left_id = Uuid::new_v4();
        let left_value = "1";

        let right_id = Uuid::new_v4();
        let right_value = "2";

        let id = Uuid::new_v4();
        let operator = "+";
        let bin = c::language_object::expression_object::binary_expression::BinaryExpression {
            id,
            left: Box::new(
                c::language_object::expression_object::ExpressionObject::NumberLiteral(
                    c::language_object::expression_object::number_literal::NumberLiteral {
                        id: left_id,
                        value: left_value.to_string(),
                    },
                ),
            ),
            operator: operator.to_string(),
            right: Box::new(
                c::language_object::expression_object::ExpressionObject::NumberLiteral(
                    c::language_object::expression_object::number_literal::NumberLiteral {
                        id: right_id,
                        value: right_value.to_string(),
                    },
                ),
            ),
        };
        let proto_bin = binary_expression_to_proto(bin);

        assert_eq!(proto_bin.id, id.to_string());
        assert_eq!(proto_bin.operator, operator);

        if let Some(inner) = &proto_bin.left {
            match &inner.expression_object {
                Some(proto::expression_object::ExpressionObject::NumberLiteral(proto_number)) => {
                    assert_eq!(proto_number.id, left_id.to_string());
                    assert_eq!(proto_number.value, left_value);
                }
                _ => panic!("expected Some(NumberLiteral)"),
            }
        } else {
            panic!("expected Some(Box<NumberLiteral>)");
        }

        if let Some(inner) = &proto_bin.right {
            match &inner.expression_object {
                Some(proto::expression_object::ExpressionObject::NumberLiteral(proto_number)) => {
                    assert_eq!(proto_number.id, right_id.to_string());
                    assert_eq!(proto_number.value, right_value);
                }
                _ => panic!("expected Some(NumberLiteral)"),
            }
        } else {
            panic!("expected Some(Box<NumberLiteral>)");
        }
    }

    #[test]
    fn test_05_call_expression() {
        let decl_id = Uuid::new_v4();
        let id = Uuid::new_v4();
        let call_identifier = "foo";
        let param_id = Uuid::new_v4();
        let param_value = "42".to_string();
        let param = c::language_object::expression_object::ExpressionObject::NumberLiteral(
            c::language_object::expression_object::number_literal::NumberLiteral {
                id: param_id,
                value: param_value.clone(),
            },
        );
        let call = c::language_object::expression_object::call_expression::CallExpression {
            id: id,
            id_declaration: decl_id,
            identifier: call_identifier.to_string(),
            argument_list: vec![param],
        };
        let proto_call = call_expression_to_proto(call);

        assert_eq!(proto_call.id, id.to_string());
        assert_eq!(proto_call.id_declaration, decl_id.to_string());
        assert_eq!(proto_call.identifier, call_identifier);

        match &proto_call.argument_list[0].expression_object {
            Some(proto::expression_object::ExpressionObject::NumberLiteral(proto_param)) => {
                assert_eq!(proto_param.id, param_id.to_string());
                assert_eq!(proto_param.value, param_value);
            }
            _ => panic!("expected Some(Comment)"),
        }
    }

    #[test]
    fn test_06_comment() {
        let id = Uuid::new_v4();
        let content = "test";
        let comment = c::language_object::special_object::comment::Comment {
            id,
            content: content.to_string(),
        };
        let proto_comment = comment_to_proto(comment);

        assert_eq!(proto_comment.id, id.to_string());
        assert_eq!(proto_comment.content, content);
    }

    #[test]
    fn test_07_declaration() {
        let number_id = Uuid::new_v4();
        let number_value = "42";
        let number_literal = c::language_object::expression_object::number_literal::NumberLiteral {
            id: number_id,
            value: number_value.to_string(),
        };

        let id = Uuid::new_v4();
        let primitive_type = c::c_type::CType::Int;
        let identifier = "test";
        let declaration = c::language_object::declaration_object::declaration::Declaration {
            id,
            primitive_type: primitive_type.clone(),
            identifier: identifier.to_string(),
            value: Some(Box::new(
                c::language_object::expression_object::ExpressionObject::NumberLiteral(
                    number_literal,
                ),
            )),
        };
        let proto_declaration = declaration_to_proto(declaration);

        assert_eq!(proto_declaration.id, id.to_string());
        assert_eq!(proto_declaration.primitive_type, primitive_type.as_str());
        assert_eq!(proto_declaration.identifier, identifier);

        if let Some(inner) = &proto_declaration.value {
            match &inner.expression_object {
                Some(proto::expression_object::ExpressionObject::NumberLiteral(proto_number)) => {
                    assert_eq!(proto_number.id, number_id.to_string());
                    assert_eq!(proto_number.value, number_value);
                }
                _ => panic!("expected Some(NumberLiteral)"),
            }
        } else {
            panic!("expected Some(Box<NumberLiteral>)");
        }
    }

    #[test]
    fn test_08_else_clause() {
        let comp_id = Uuid::new_v4();
        let compound_statement =
            c::language_object::statement_object::compound_statement::CompoundStatement {
                id: comp_id,
                code_block: vec![],
            };

        let id = Uuid::new_v4();
        let else_clause =
            c::language_object::statement_object::if_statement::else_clause::ElseClause {
                id,
                condition: None,
                compound_statement: Box::new(
                    c::language_object::statement_object::StatementObject::CompoundStatement(
                        compound_statement,
                    ),
                ),
            };
        let proto_else = else_clause_to_proto(else_clause);

        assert_eq!(proto_else.id, id.to_string());
        assert!(proto_else.condition.is_none());
        match proto_else
            .compound_statement
            .unwrap()
            .statement_object
            .unwrap()
        {
            proto::statement_object::StatementObject::CompoundStatement(compound_statement) => {
                assert_eq!(compound_statement.id, comp_id.to_string());
                assert_eq!(compound_statement.code_block.len(), 0);
            }
            _ => panic!("expected Some(CompoundStatement)"),
        }
    }

    #[test]
    fn test_10_function_declaration() {
        let param_id = Uuid::new_v4();
        let param = c::language_object::declaration_object::function_declaration::function_parameter::FunctionParameter {
            id: param_id,
            identifier: "p".into(),
            param_type: c::c_type::CType::Float,
        };

        let id = Uuid::new_v4();
        let return_type = c::c_type::CType::Int;
        let identifier = "my_fn";
        let decl =
            c::language_object::declaration_object::function_declaration::FunctionDeclaration {
                id,
                return_type: return_type.clone(),
                identifier: identifier.to_string(),
                parameter_list: vec![param],
            };

        let proto_decl = function_declaration_to_proto(decl);
        assert_eq!(proto_decl.id, id.to_string());
        assert_eq!(proto_decl.return_type, return_type.as_str());
        assert_eq!(proto_decl.identifier, identifier);
        assert_eq!(proto_decl.parameter_list.len(), 1);

        let p = &proto_decl.parameter_list[0];
        assert_eq!(p.id, param_id.to_string());
        assert_eq!(p.identifier, "p");
        assert_eq!(p.param_type, c::c_type::CType::Float.as_str());
    }

    #[test]
    fn test_11_function_definition() {
        let param_id = Uuid::new_v4();
        let param = c::language_object::declaration_object::function_declaration::function_parameter::FunctionParameter {
            id: param_id,
            identifier: "p".into(),
            param_type: c::c_type::CType::Float,
        };

        let comp_id = Uuid::new_v4();
        let compound_statement =
            c::language_object::statement_object::compound_statement::CompoundStatement {
                id: comp_id,
                code_block: vec![],
            };

        let id = Uuid::new_v4();
        let return_type = c::c_type::CType::Int;
        let identifier = "my_def";
        let def = c::language_object::declaration_object::function_definition::FunctionDefinition {
            id,
            return_type: return_type.clone(),
            identifier: identifier.to_string(),
            parameter_list: vec![param],
            compound_statement,
        };

        let proto_def = function_definition_to_proto(def);
        assert_eq!(proto_def.id, id.to_string());
        assert_eq!(proto_def.return_type, return_type.as_str());
        assert_eq!(proto_def.identifier, identifier);
        assert_eq!(proto_def.parameter_list.len(), 1);

        // compound_statement is Some(...) in the proto
        match proto_def.compound_statement {
            Some(comp) => {
                assert_eq!(comp.id, comp_id.to_string());
                assert!(comp.code_block.is_empty());
            }
            _ => panic!("expected Some(compound_statement)"),
        }
    }

    #[test]
    fn test_12_function_parameter() {
        let id = Uuid::new_v4();
        let identifier = "arg0";
        let param_type = c::c_type::CType::Char;
        let param = c::language_object::declaration_object::function_declaration::function_parameter::FunctionParameter {
            id,
            identifier: identifier.to_string(),
            param_type: param_type.clone(),
        };

        let proto_param = function_parameter_to_proto(param);
        assert_eq!(proto_param.id, id.to_string());
        assert_eq!(proto_param.identifier, identifier);
        assert_eq!(proto_param.param_type, param_type.as_str());
    }

    #[test]
    fn test_13_if_statement() {
        let cond_id = Uuid::new_v4();
        let cond_value = "1";
        let cond = c::language_object::expression_object::ExpressionObject::NumberLiteral(
            c::language_object::expression_object::number_literal::NumberLiteral {
                id: cond_id,
                value: cond_value.to_string(),
            },
        );

        let comp_id = Uuid::new_v4();
        let id = Uuid::new_v4();
        let compound_statement =
            c::language_object::statement_object::StatementObject::CompoundStatement(
                c::language_object::statement_object::compound_statement::CompoundStatement {
                    id: comp_id,
                    code_block: vec![],
                },
            );
        let if_stmt = c::language_object::statement_object::if_statement::IfStatement {
            id,
            condition: Box::new(cond),
            compound_statement: Box::new(compound_statement),
            else_statement: None,
        };

        let proto_if = if_statement_to_proto(if_stmt);
        assert_eq!(proto_if.id, id.to_string());

        match &proto_if
            .condition
            .expect("expected Some(condition)")
            .expression_object
            .unwrap()
        {
            proto::expression_object::ExpressionObject::NumberLiteral(proto_num) => {
                assert_eq!(proto_num.id, cond_id.to_string());
                assert_eq!(proto_num.value, cond_value);
            }
            _ => panic!("expected NumberLiteral in condition"),
        }

        // compound_statement should be present
        match proto_if
            .compound_statement
            .unwrap()
            .statement_object
            .unwrap()
        {
            proto::statement_object::StatementObject::CompoundStatement(compound_statement) => {
                assert_eq!(compound_statement.id, comp_id.to_string());
                assert!(compound_statement.code_block.is_empty());
            }
            _ => panic!("expected Some(compound_statement)"),
        }

        // else_clause should be None
        assert!(proto_if.else_clause.is_none());
    }

    #[test]
    fn test_14_number_literal() {
        let id = Uuid::new_v4();
        let value = "314";
        let num = c::language_object::expression_object::number_literal::NumberLiteral {
            id,
            value: value.to_string(),
        };
        let proto_num = number_literal_to_proto(num);
        assert_eq!(proto_num.id, id.to_string());
        assert_eq!(proto_num.value, value);
    }

    #[test]
    fn test_15_preproc_include() {
        let id = Uuid::new_v4();
        let content = "#include <stdio.h>";
        let pre = c::language_object::declaration_object::preproc_include::PreprocInclude {
            id,
            content: content.to_string(),
        };
        let proto_pre = preproc_include_to_proto(pre);
        assert_eq!(proto_pre.id, id.to_string());
        assert_eq!(proto_pre.content, content);
    }

    #[test]
    fn test_16_reference() {
        let id = Uuid::new_v4();
        let identifier = "test";
        let decl_id = Uuid::new_v4();
        let reference = c::language_object::expression_object::reference::Reference {
            id,
            identifier: identifier.to_string(),
            declaration_id: decl_id,
        };

        let proto_ref = reference_to_proto(reference);
        assert_eq!(proto_ref.id, id.to_string());
        assert_eq!(proto_ref.identifier, identifier);
        assert_eq!(proto_ref.declaration_id, decl_id.to_string());
    }

    #[test]
    fn test_17_return_statement() {
        let value_id = Uuid::new_v4();
        let id = Uuid::new_v4();
        let value = "42";
        let ret = c::language_object::statement_object::return_statement::ReturnStatement {
            id,
            value: Box::new(
                c::language_object::expression_object::ExpressionObject::NumberLiteral(
                    c::language_object::expression_object::number_literal::NumberLiteral {
                        id: value_id,
                        value: value.to_string(),
                    },
                ),
            ),
        };

        let proto_ret = return_statement_to_proto(ret);
        assert_eq!(proto_ret.id, id.to_string());

        match proto_ret
            .value
            .expect("expected some")
            .expression_object
            .unwrap()
        {
            proto::expression_object::ExpressionObject::NumberLiteral(proto_number) => {
                assert_eq!(proto_number.id, value_id.to_string());
                assert_eq!(proto_number.value, value);
            }
            _ => panic!("expected Some(NumberLiteral)"),
        }
    }

    #[test]
    fn test_18_string_literal() {
        let id = Uuid::new_v4();
        let val = "hello world";
        let s = c::language_object::expression_object::string_literal::StringLiteral {
            id,
            value: val.to_string(),
        };
        let proto_s = string_literal_to_proto(s);
        assert_eq!(proto_s.id, id.to_string());
        assert_eq!(proto_s.value, val);
    }

    #[test]
    fn test_19_compound_statement() {
        let comment_id = Uuid::new_v4();
        let comment = c::language_object::special_object::comment::Comment {
            id: comment_id,
            content: "ok".to_string(),
        };

        let comp_id = Uuid::new_v4();
        let compound =
            c::language_object::statement_object::compound_statement::CompoundStatement {
                id: comp_id,
                code_block: vec![c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::Comment(comment)],
            };

        let proto_comp = compound_statement_to_proto(compound);
        assert_eq!(proto_comp.id, comp_id.to_string());
        assert_eq!(proto_comp.code_block.len(), 1);

        match proto_comp.code_block[0]
            .compound_statement_object
            .as_ref()
            .unwrap()
        {
            proto::compound_statement_object::CompoundStatementObject::Comment(proto_comment) => {
                assert_eq!(proto_comment.id, comment_id.to_string());
                assert_eq!(proto_comment.content, "ok");
            }
            _ => panic!("expected Some(Comment)"),
        }
    }
}
