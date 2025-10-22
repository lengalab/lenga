use uuid::Uuid;

use crate::{
    language::c::{
        c_type::CType,
        language_object::{
            ConversionError, LanguageObject as CLanguageObject,
            declaration_object::{
                DeclarationObject,
                declaration::Declaration,
                function_declaration::{
                    FunctionDeclaration, function_parameter::FunctionParameter,
                },
                function_definition::FunctionDefinition,
                preproc_include::PreprocInclude,
            },
            expression_object::{
                ExpressionObject, assignment_expression::AssignmentExpression,
                binary_expression::BinaryExpression, call_expression::CallExpression,
                number_literal::NumberLiteral, reference::Reference, string_literal::StringLiteral,
            },
            special_object::{comment::Comment, source_file::SourceFile, unknown::Unknown},
            statement_object::{
                StatementObject,
                compound_statement::CompoundStatement,
                if_statement::{ElseStatement, IfStatement, else_clause::ElseClause},
                return_statement::ReturnStatement,
            },
        },
        parsers::context::{Context, SymbolAlreadyExists},
        writers::node_writer::node_type::NodeType,
    },
    node::Node,
};

use crate::language::c::TreeSitterNodeExt;

#[derive(Debug)]
pub enum TreeSitterParserError {
    SymbolAlreadyExists(SymbolAlreadyExists),
    MissingSymbol(String),
    WrongType(String),
}

impl From<SymbolAlreadyExists> for TreeSitterParserError {
    fn from(err: SymbolAlreadyExists) -> Self {
        TreeSitterParserError::SymbolAlreadyExists(err)
    }
}

impl From<TreeSitterParserError> for String {
    fn from(err: TreeSitterParserError) -> Self {
        match err {
            TreeSitterParserError::SymbolAlreadyExists(_) => "Symbol already exists".to_string(),
            TreeSitterParserError::MissingSymbol(name) => {
                format!("Missing symbol: {}", name)
            }
            TreeSitterParserError::WrongType(ty) => format!("Wrong type: {}", ty),
        }
    }
}

pub struct TreeSitterParser<'a> {
    objects: Vec<CLanguageObject>,
    context: Context<'a>,
}

impl<'a> TreeSitterParser<'a> {
    pub fn new() -> TreeSitterParser<'a> {
        TreeSitterParser {
            objects: Vec::new(),
            context: Context::new(),
        }
    }
    pub fn parse_with_tree(
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<Vec<CLanguageObject>, TreeSitterParserError> {
        let this = Self::new();
        this.file_from_tree_sitter_nodes(node, source_code)
    }

    fn branch(&'a self) -> Self {
        Self {
            objects: Vec::new(),
            context: self.context.branch(),
        }
    }

    fn file_from_tree_sitter_nodes(
        &self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<Vec<CLanguageObject>, TreeSitterParserError> {
        let mut current_node = node;
        let mut branch = self.branch();
        loop {
            let object = branch.object_from_tree_sitter_node(current_node, source_code)?;
            branch.objects.push(object);
            let Some(next_sibling) = current_node.next_sibling() else {
                break;
            };
            current_node = next_sibling;
        }
        Ok(branch.objects)
    }

    fn compound_statement_from_tree_sitter_nodes(
        &self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<CompoundStatement, TreeSitterParserError> {
        let mut current_node = node;
        assert_eq!(current_node.kind(), "{");
        current_node = current_node.next_sibling().unwrap();
        let mut branch = self.branch();
        loop {
            if current_node.kind() == "}" {
                break;
            }
            let object = branch.object_from_tree_sitter_node(current_node, source_code)?;
            branch.objects.push(object);
            current_node = current_node.next_sibling().unwrap();
        }
        Ok(CompoundStatement {
            id: Uuid::new_v4(),
            code_block: branch
                .objects
                .into_iter()
                .map(|o| o.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    pub fn object_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<CLanguageObject, TreeSitterParserError> {
        let result: CLanguageObject = match node.kind() {
            "preproc_include" => CLanguageObject::PreprocInclude(
                self.preproc_include_from_tree_sitter_node(node, source_code)?,
            ),
            "function_definition" => CLanguageObject::FunctionDefinition(
                self.function_definition_from_tree_sitter_node(node, source_code)?,
            ),
            "declaration" => self.declaration_from_tree_sitter_node(node, source_code)?,
            "identifier" => {
                CLanguageObject::Reference(self.reference_from_tree_sitter_node(node, source_code)?)
            }
            "return_statement" => CLanguageObject::ReturnStatement(
                self.return_statement_from_tree_sitter_node(node, source_code)?,
            ),
            "number_literal" => {
                let value = node.content(source_code);
                CLanguageObject::NumberLiteral(NumberLiteral {
                    id: Uuid::new_v4(),
                    value,
                })
            }
            "string_literal" => CLanguageObject::StringLiteral(
                self.string_literal_from_tree_sitter_node(node, source_code)?,
            ),
            "call_expression" => CLanguageObject::CallExpression(
                self.call_expression_from_tree_sitter_node(node, source_code)?,
            ),
            "binary_expression" => CLanguageObject::BinaryExpression(
                self.binary_expression_from_tree_sitter_node(node, source_code),
            ),
            "expression_statement" => {
                self.expression_statement_from_tree_sitter_node(node, source_code)?
            }

            "comment" => {
                CLanguageObject::Comment(self.comment_from_tree_sitter_node(node, source_code))
            }
            "if_statement" => CLanguageObject::IfStatement(
                self.if_statement_from_tree_sitter_node(node, source_code)?,
            ),
            "assignment_expression" => CLanguageObject::AssignmentExpression(
                self.assignment_expression_from_tree_sitter_node(node, source_code),
            ),
            "compound_statement" => CLanguageObject::CompoundStatement(
                self.branch().compound_statement_from_tree_sitter_nodes(
                    node.child(0).unwrap(),
                    source_code,
                )?,
            ),
            "{" | "}" | "(" | ")" | "," => {
                // Ignore delimiters
                panic!("Delimiter node should not be processed: {}", node.kind());
            }
            other => panic!("Unknown node type: {}", other),
        };
        Ok(result)
    }

    fn declaration_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<CLanguageObject, TreeSitterParserError> {
        let type_node = node.child(0).unwrap();
        assert_eq!(type_node.kind(), "primitive_type");
        let primitive_type = CType::from_tree_sitter_node(type_node, source_code);

        let declarator_node = node.child(1).unwrap();
        match declarator_node.kind() {
            "init_declarator" => {
                let (identifier, value) = {
                    let node = declarator_node;
                    let identifier_node = node.child(0).unwrap();
                    assert_eq!(identifier_node.kind(), "identifier");
                    let identifier = identifier_node.content(source_code).to_string();
                    assert_eq!(node.child(1).unwrap().kind(), "=");

                    let value_node = node.child(2).unwrap();
                    let value = self
                        .branch()
                        .object_from_tree_sitter_node(value_node, source_code)
                        .unwrap();
                    (identifier, value)
                };
                Ok(CLanguageObject::Declaration(Declaration {
                    id: self.context.insert_symbol(&identifier, false)?,
                    primitive_type,
                    identifier,
                    value: Some(Box::new(value.try_into()?)),
                }))
            }
            "identifier" => {
                let identifier = declarator_node.content(source_code);
                Ok(CLanguageObject::Declaration(Declaration {
                    id: self.context.insert_symbol(&identifier, false)?,
                    primitive_type,
                    identifier,
                    value: None,
                }))
            }
            "function_declarator" => Ok(CLanguageObject::FunctionDeclaration(
                self.function_declaration_from_tree_sitter_node(
                    primitive_type,
                    declarator_node,
                    source_code,
                )?,
            )),
            other => {
                panic!("Unexpected declarator type: {}", other);
            }
        }
    }

    fn preproc_include_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<PreprocInclude, TreeSitterParserError> {
        assert_eq!(node.child(0).unwrap().kind(), "#include");
        let lib = node.child(1).unwrap();
        assert_eq!(lib.kind(), "system_lib_string");
        Ok(PreprocInclude {
            id: Uuid::new_v4(),
            content: lib.content(source_code),
        })
    }

    fn string_literal_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<StringLiteral, TreeSitterParserError> {
        let value = node.content(source_code);
        if let Some(value) = value.strip_prefix('"') {
            return Ok(StringLiteral {
                id: Uuid::new_v4(),
                value: value.strip_suffix('"').unwrap().to_string(),
            });
        }
        panic!("unexpected delimitators for string literal: {}", value);
    }

    fn return_statement_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<ReturnStatement, TreeSitterParserError> {
        assert_eq!(node.child(0).unwrap().kind(), "return");
        let child = node.child(1).unwrap();
        let value: Option<ExpressionObject> = match child.kind() {
            ";" => None,
            _ => Some(
                self.object_from_tree_sitter_node(child, source_code)
                    .unwrap()
                    .try_into()?,
            ),
        };
        Ok(ReturnStatement {
            id: Uuid::new_v4(),
            value,
        })
    }

    fn reference_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<Reference, TreeSitterParserError> {
        assert_eq!(node.kind(), "identifier");
        let identifier = node.content(source_code).to_string();
        let declaration_id = self
            .context
            .get_symbol_id(&identifier, false)
            .ok_or(TreeSitterParserError::MissingSymbol(identifier.to_string()))?;
        Ok(Reference {
            id: Uuid::new_v4(),
            declaration_id,
            identifier,
        })
    }

    pub fn number_literal_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> NumberLiteral {
        let value = node.content(source_code);
        NumberLiteral {
            id: Uuid::new_v4(),
            value,
        }
    }

    fn if_statement_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<IfStatement, TreeSitterParserError> {
        assert_eq!(node.child(0).unwrap().kind(), "if");
        let parenthesized_expression = node.child(1).unwrap();
        assert_eq!(parenthesized_expression.kind(), "parenthesized_expression");
        let compound_statement = node.child(2).unwrap();
        assert_eq!(compound_statement.kind(), "compound_statement");

        assert_eq!(parenthesized_expression.child(0).unwrap().kind(), "(");
        let condition = self
            .branch()
            .object_from_tree_sitter_node(parenthesized_expression.child(1).unwrap(), source_code)
            .unwrap();
        assert_eq!(parenthesized_expression.child(2).unwrap().kind(), ")");
        let code_block = self.compound_statement_from_tree_sitter_nodes(
            compound_statement.child(0).unwrap(),
            source_code,
        )?;

        let else_clause = if let Some(else_node) = node.child(3) {
            assert_eq!(else_node.kind(), "else_clause");
            Some(self.else_clause_from_tree_sitter_node(else_node, source_code)?)
        } else {
            None
        };

        Ok(IfStatement {
            id: Uuid::new_v4(),
            condition: Box::new(condition.try_into()?),
            compound_statement: code_block.try_into()?, // TODO support other types of statements
            else_statement: else_clause,
        })
    }

    fn function_parameter_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<FunctionParameter, TreeSitterParserError> {
        let type_node = node.child(0).unwrap();
        assert_eq!(type_node.kind(), "primitive_type");
        let param_type = CType::from_tree_sitter_node(type_node, source_code);

        let name_node = node.child(1).unwrap();
        assert_eq!(name_node.kind(), "identifier");
        let identifier = name_node.content(source_code).to_string();

        Ok(FunctionParameter {
            id: self.context.insert_symbol(&identifier, false)?,
            identifier,
            param_type,
        })
    }

    pub fn function_declaration_from_tree_sitter_node(
        &mut self,
        return_type: CType,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<FunctionDeclaration, TreeSitterParserError> {
        let (identifier, parameter_list, id, _) =
            self.parse_function_parameter_list(node, source_code)?;
        Ok(FunctionDeclaration {
            id,
            return_type,
            identifier,
            parameter_list,
        })
    }

    pub fn function_definition_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<FunctionDefinition, TreeSitterParserError> {
        let return_type = CType::from_tree_sitter_node(node.child(0).unwrap(), source_code);
        let (identifier, parameter_list, id, subcontext) =
            self.parse_function_parameter_list(node.child(1).unwrap(), source_code)?;
        let code_block_node = node.child(2).unwrap();
        assert_eq!(code_block_node.kind(), "compound_statement");
        let code_block = subcontext.compound_statement_from_tree_sitter_nodes(
            code_block_node.child(0).unwrap(),
            source_code,
        )?;

        Ok(FunctionDefinition {
            id,
            return_type,
            identifier,
            parameter_list,
            compound_statement: code_block,
        })
    }

    pub fn else_clause_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<ElseStatement, TreeSitterParserError> {
        assert_eq!(node.child(0).unwrap().kind(), "else");
        let compound_statement = node.child(1).unwrap();
        match compound_statement.kind() {
            "compound_statement" => {
                let code_block = self.compound_statement_from_tree_sitter_nodes(
                    compound_statement.child(0).unwrap(),
                    source_code,
                )?;

                Ok(ElseStatement::ElseClause(Box::new(ElseClause {
                    id: Uuid::new_v4(),
                    compound_statement: code_block.try_into()?, // TODO support other types of statements
                })))
            }
            "if_statement" => {
                let if_statement =
                    self.if_statement_from_tree_sitter_node(compound_statement, source_code)?;

                Ok(ElseStatement::ElseIf(Box::new(IfStatement {
                    id: Uuid::new_v4(),
                    condition: if_statement.condition,
                    compound_statement: if_statement.compound_statement,
                    else_statement: if_statement.else_statement,
                })))
            }
            other => panic!("Unexpected node kind: {}", other),
        }
    }

    pub fn comment_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Comment {
        let content = node.content(source_code);
        Comment {
            id: Uuid::new_v4(),
            content,
        }
    }

    fn call_expression_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<CallExpression, TreeSitterParserError> {
        let identifier_node = node.child(0).unwrap();
        assert_eq!(identifier_node.kind(), "identifier");
        let identifier = identifier_node.content(source_code).to_string();
        let id_declaration = self
            .context
            .get_symbol_id(&identifier, true)
            .unwrap_or(Uuid::nil()); // TODO this symbols should be registered from imported libraries

        // TODO check if this works and replace on function_declaration.rs
        let argument_list_node = node.child(1).unwrap();
        assert_eq!(argument_list_node.kind(), "argument_list");

        let mut argument_list = Vec::new();
        assert_eq!(argument_list_node.child(0).unwrap().kind(), "(");
        let mut argument = argument_list_node.child(1).unwrap();
        if argument.kind() != ")" {
            loop {
                argument_list.push(
                    self.branch()
                        .object_from_tree_sitter_node(argument, source_code)?,
                );
                let delimitator = argument.next_sibling().unwrap();
                if delimitator.kind() == ")" {
                    break;
                }
                assert_eq!(delimitator.kind(), ",");
                argument = delimitator.next_sibling().unwrap();
            }
        }

        Ok(CallExpression {
            id: Uuid::new_v4(),
            id_declaration,
            identifier,
            argument_list: argument_list
                .into_iter()
                .map(|e| e.try_into())
                .collect::<Result<_, ConversionError>>()?,
        })
    }

    pub fn binary_expression_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> BinaryExpression {
        let left = self
            .branch()
            .object_from_tree_sitter_node(node.child(0).unwrap(), source_code)
            .unwrap();
        let operator = node.child(1).unwrap().content(source_code);
        let right = self
            .branch()
            .object_from_tree_sitter_node(node.child(2).unwrap(), source_code)
            .unwrap();
        BinaryExpression {
            id: Uuid::new_v4(),
            left: Box::new(left.try_into().unwrap()),
            operator,
            right: Box::new(right.try_into().unwrap()),
        }
    }

    fn assignment_expression_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> AssignmentExpression {
        let identifier = node.child(0).unwrap().content(source_code);
        assert_eq!(node.child(1).unwrap().kind(), "=");
        let value = self
            .branch()
            .object_from_tree_sitter_node(node.child(2).unwrap(), source_code)
            .unwrap();
        let id_declaration = self.context.get_or_insert_symbol(&identifier, false);
        AssignmentExpression {
            id: Uuid::new_v4(),
            id_declaration,
            identifier,
            value: Box::new(value.try_into().unwrap()),
        }
    }

    fn expression_statement_from_tree_sitter_node(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<CLanguageObject, TreeSitterParserError> {
        let child_node = node.child(0).unwrap();
        self.branch()
            .object_from_tree_sitter_node(child_node, source_code)
    }

    fn parse_function_parameter_list(
        &mut self,
        node: tree_sitter::Node<'_>,
        source_code: &str,
    ) -> Result<(String, Vec<FunctionParameter>, Uuid, TreeSitterParser), TreeSitterParserError>
    {
        let identifier_node = node.child(0).unwrap();
        assert_eq!(identifier_node.kind(), "identifier");
        let identifier = identifier_node.content(source_code).to_string();
        let id = self.context.get_or_insert_symbol(&identifier, true);
        let mut subcontext = self.branch();

        let parameters_node = node.child(1).unwrap();
        assert_eq!(parameters_node.kind(), "parameter_list");
        let mut parameter_list: Vec<FunctionParameter> = Vec::new();
        if let Some(mut parameter) = parameters_node.child(0) {
            loop {
                if parameter.kind() == "parameter_declaration" {
                    parameter_list.push(
                        subcontext
                            .function_parameter_from_tree_sitter_node(parameter, source_code)?,
                    );
                }
                if let Some(next_param) = parameter.next_sibling() {
                    parameter = next_param;
                } else {
                    break;
                }
            }
        }
        Ok((identifier, parameter_list, id, subcontext))
    }
}
