use uuid::Uuid;

use crate::{
    language::c::{
        language_object::{
            LanguageObject as CLanguageObject, assignment_expression::AssignmentExpression,
            binary_expression::BinaryExpression, call_expression::CallExpression, comment::Comment,
            compound_statement::CompoundStatement, declaration::Declaration,
            else_clause::ElseClause, expression_statement::ExpressionStatement,
            function_declaration::FunctionDeclaration, function_definition::FunctionDefinition,
            function_parameter::FunctionParameter, if_statement::IfStatement,
            number_literal::NumberLiteral, preproc_include::PreprocInclude, reference::Reference,
            return_statement::ReturnStatement, source_file::SourceFile,
            string_literal::StringLiteral,
        },
        object_types::c_type::CType,
        parsers::context::{Context, SymbolAlreadyExists},
        writers::node_writer::node_type::NodeType,
    },
    node::Node,
};

#[derive(Debug)]
pub enum NodeParserError {
    SymbolAlreadyExists(SymbolAlreadyExists),
    MissingSymbol(String),
    EmptyVec,
}

impl From<SymbolAlreadyExists> for NodeParserError {
    fn from(err: SymbolAlreadyExists) -> Self {
        NodeParserError::SymbolAlreadyExists(err)
    }
}

impl From<NodeParserError> for String {
    fn from(err: NodeParserError) -> Self {
        match err {
            NodeParserError::SymbolAlreadyExists(_) => "Symbol already exists".to_string(),
            NodeParserError::MissingSymbol(name) => format!("Missing symbol: {}", name),
            NodeParserError::EmptyVec => "Tried to parse empty vec".to_string(),
        }
    }
}

pub struct NodeParser<'a> {
    objects: Vec<CLanguageObject>,
    context: Context<'a>,
}

impl<'a> NodeParser<'a> {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            context: Context::new(),
        }
    }

    fn branch(&'a self) -> Self {
        Self {
            objects: Vec::new(),
            context: self.context.branch(),
        }
    }

    pub fn read_file(&mut self, nodes: Vec<u8>) -> Result<SourceFile, String> {
        let mut nodes_loaded = bincode::deserialize::<Vec<Node>>(&nodes).unwrap();
        assert_eq!(nodes_loaded.len(), 1);
        let file = nodes_loaded.pop().unwrap();
        assert_eq!(file.node_type, NodeType::SourceFile.as_u64());
        Ok(self.source_file_from_node(file)?)
    }

    fn clanguageobject_from_node(
        &mut self,
        node: Node,
    ) -> Result<CLanguageObject, NodeParserError> {
        Ok(match NodeType::from(node.node_type) {
            NodeType::SourceFile => CLanguageObject::SourceFile(self.source_file_from_node(node)?),
            NodeType::AssignmentExpression => {
                CLanguageObject::AssignmentExpression(self.assignment_expression_from_node(node)?)
            }
            NodeType::BinaryExpression => {
                CLanguageObject::BinaryExpression(self.binary_expression_from_node(node)?)
            }
            NodeType::CallExpression => {
                CLanguageObject::CallExpression(self.call_expression_from_node(node)?)
            }
            NodeType::Comment => CLanguageObject::Comment(self.comment_from_node(node)?),
            NodeType::Declaration => {
                CLanguageObject::Declaration(self.declaration_from_node(node)?)
            }
            NodeType::ElseClause => CLanguageObject::ElseClause(self.else_clause_from_node(node)?),
            NodeType::ExpressionStatement => {
                CLanguageObject::ExpressionStatement(self.expression_statement_from_node(node)?)
            }
            NodeType::FunctionDeclaration => {
                CLanguageObject::FunctionDeclaration(self.function_declaration_from_node(node)?)
            }
            NodeType::FunctionDefinition => {
                CLanguageObject::FunctionDefinition(self.function_definition_from_node(node)?)
            }
            NodeType::IfStatement => {
                CLanguageObject::IfStatement(self.if_statement_from_node(node)?)
            }
            NodeType::NumberLiteral => {
                CLanguageObject::NumberLiteral(self.number_literal_from_node(node)?)
            }
            NodeType::PreprocInclude => {
                CLanguageObject::PreprocInclude(self.preproc_include_from_node(node)?)
            }
            NodeType::Reference => CLanguageObject::Reference(self.reference_from_node(node)?),
            NodeType::ReturnStatement => {
                CLanguageObject::ReturnStatement(self.return_statement_from_node(node)?)
            }
            NodeType::StringLiteral => {
                CLanguageObject::StringLiteral(self.string_literal_from_node(node)?)
            }
            NodeType::FunctionParameter => panic!("this does not represent a clanguage object"),
            NodeType::TypeDescription => panic!("this does not represent a clanguage object"),
            NodeType::Symbol => panic!("this does not represent a clanguage object"),
            NodeType::CompoundStatement => {
                CLanguageObject::CompoundStatement(self.compound_statement_from_node(node)?)
            }
        })
    }

    fn source_file_from_node(&self, node: Node) -> Result<SourceFile, NodeParserError> {
        assert_eq!(node.node_type, NodeType::SourceFile.as_u64());

        let mut branch = self.branch();
        let mut code: Vec<CLanguageObject> = Vec::new();
        for child in node.children {
            code.push(branch.clanguageobject_from_node(child)?)
        }
        Ok(SourceFile {
            id: node.id,
            code,
        })
    }

    fn assignment_expression_from_node(
        &mut self,
        node: Node,
    ) -> Result<AssignmentExpression, NodeParserError> {
        assert_eq!(node.node_type, NodeType::AssignmentExpression.as_u64());
        Ok(AssignmentExpression {
            id: node.id,
            identifier: node.content,
            value: self.unpack_parse(node.children)?,
        })
    }

    fn unpack_parse(
        &mut self,
        mut children: Vec<Node>,
    ) -> Result<Box<CLanguageObject>, NodeParserError> {
        Ok(Box::new(self.clanguageobject_from_node(
            children.pop().ok_or(NodeParserError::EmptyVec)?,
        )?))
    }

    fn binary_expression_from_node(
        &mut self,
        mut node: Node,
    ) -> Result<BinaryExpression, NodeParserError> {
        assert_eq!(node.node_type, NodeType::BinaryExpression.as_u64());
        let left = node.tags.remove("left").unwrap().pop().unwrap();
        let right = node.tags.remove("right").unwrap().pop().unwrap();
        Ok(BinaryExpression {
            id: node.id,
            left: Box::new(self.clanguageobject_from_node(left)?),
            operator: node.content,
            right: Box::new(self.clanguageobject_from_node(right)?),
        })
    }

    fn call_expression_from_node(&mut self, node: Node) -> Result<CallExpression, NodeParserError> {
        assert_eq!(node.node_type, NodeType::CallExpression.as_u64());

        let (id_declaration, identifier) = match Uuid::parse_str(&node.content) {
            Ok(id_declaration) => (id_declaration, self.context.get_symbol_identifier(&id_declaration).unwrap()),
            Err(_) => (Uuid::nil(), node.content),
        };
            
        Ok(CallExpression {
            id: node.id,
            id_declaration,
            identifier,
            argument_list: node
                .children
                .into_iter()
                .map(|arg| self.clanguageobject_from_node(arg))
                .collect::<Result<Vec<CLanguageObject>, NodeParserError>>()?,
        })
    }

    fn comment_from_node(&mut self, node: Node) -> Result<Comment, NodeParserError> {
        assert_eq!(node.node_type, NodeType::Comment.as_u64());
        Ok(Comment {
            id: node.id,
            content: node.content,
        })
    }

    fn declaration_from_node(&mut self, mut node: Node) -> Result<Declaration, NodeParserError> {
        assert_eq!(node.node_type, NodeType::Declaration.as_u64());
        Ok(Declaration {
            id: self
                .context
                .insert_symbol_with_id(&node.content, node.id, false)?,
            primitive_type: CType::from_str(
                &node.tags.remove("type").unwrap().pop().unwrap().content,
            )
            .unwrap(),
            identifier: node.content,
            value: node
                .children
                .pop()
                .map(|value| self.clanguageobject_from_node(value).map(Box::new))
                .transpose()?,
        })
    }

    fn else_clause_from_node(&mut self, mut node: Node) -> Result<ElseClause, NodeParserError> {
        assert_eq!(node.node_type, NodeType::ElseClause.as_u64());
        Ok(ElseClause {
            id: node.id,
            condition: node
                .tags
                .remove("condition")
                .map(|value| self.unpack_parse(value))
                .transpose()?,
            compound_statement: self.compound_statement_from_node(node.children.pop().unwrap())?,
        })
    }

    fn expression_statement_from_node(
        &mut self,
        node: Node,
    ) -> Result<ExpressionStatement, NodeParserError> {
        assert_eq!(node.node_type, NodeType::ExpressionStatement.as_u64());
        Ok(ExpressionStatement {
            id: node.id,
            identifier: self.context.get_symbol_identifier(&node.id).unwrap(),
            argument_list: node
                .children
                .into_iter()
                .map(|arg| self.clanguageobject_from_node(arg))
                .collect::<Result<Vec<CLanguageObject>, NodeParserError>>()?,
        })
    }

    fn function_declaration_from_node(
        &mut self,
        mut node: Node,
    ) -> Result<FunctionDeclaration, NodeParserError> {
        assert_eq!(node.node_type, NodeType::FunctionDeclaration.as_u64());
        Ok(FunctionDeclaration {
            id: self
                .context
                .insert_symbol_with_id(&node.content, node.id, true)
                .unwrap_or(node.id),
            return_type: CType::from_str(
                &node
                    .tags
                    .remove("return_type")
                    .unwrap()
                    .pop()
                    .unwrap()
                    .content,
            )
            .unwrap(),
            identifier: node.content,
            parameter_list: node
                .tags
                .remove("args")
                .unwrap()
                .into_iter()
                .map(|mut param| {
                    Ok(FunctionParameter {
                        id: param.id,
                        identifier: param.content,
                        param_type: CType::from_str(
                            &param.tags.remove("type").unwrap().pop().unwrap().content,
                        )
                        .unwrap(),
                    })
                })
                .collect::<Result<Vec<FunctionParameter>, SymbolAlreadyExists>>()?,
        })
    }

    fn function_definition_from_node(
        &mut self,
        mut node: Node,
    ) -> Result<FunctionDefinition, NodeParserError> {
        assert_eq!(node.node_type, NodeType::FunctionDefinition.as_u64());
        let id = self
            .context
            .insert_symbol_with_id(&node.content, node.id, true)
            .unwrap_or(node.id);
        let mut subcontext = self.branch();

        Ok(FunctionDefinition {
            id,
            return_type: CType::from_str(
                &node
                    .tags
                    .remove("return_type")
                    .unwrap()
                    .pop()
                    .unwrap()
                    .content,
            )
            .unwrap(),
            identifier: node.content,
            parameter_list: node
                .tags
                .remove("args")
                .unwrap()
                .into_iter()
                .map(|mut param| {
                    Ok(FunctionParameter {
                        id: subcontext.context.insert_symbol_with_id(
                            &param.content,
                            param.id,
                            false,
                        )?,
                        identifier: param.content,
                        param_type: CType::from_str(
                            &param.tags.remove("type").unwrap().pop().unwrap().content,
                        )
                        .unwrap(),
                    })
                })
                .collect::<Result<Vec<FunctionParameter>, SymbolAlreadyExists>>()?,
            compound_statement: subcontext
                .compound_statement_from_node(node.children.pop().unwrap())?,
        })
    }

    fn if_statement_from_node(&mut self, mut node: Node) -> Result<IfStatement, NodeParserError> {
        assert_eq!(node.node_type, NodeType::IfStatement.as_u64());
        Ok(IfStatement {
            id: node.id,
            condition: self.unpack_parse(node.tags.remove("condition").unwrap())?,
            compound_statement: self
                .branch()
                .compound_statement_from_node(node.children.pop().unwrap())?,
            else_clause: node
                .tags
                .remove("else_clause")
                .map(|mut else_clause| {
                    self.branch()
                        .else_clause_from_node(else_clause.pop().unwrap())
                })
                .transpose()?,
        })
    }

    fn number_literal_from_node(&mut self, node: Node) -> Result<NumberLiteral, NodeParserError> {
        assert_eq!(node.node_type, NodeType::NumberLiteral.as_u64());
        Ok(NumberLiteral {
            id: node.id,
            value: node.content,
        })
    }

    fn preproc_include_from_node(&mut self, node: Node) -> Result<PreprocInclude, NodeParserError> {
        assert_eq!(node.node_type, NodeType::PreprocInclude.as_u64());
        Ok(PreprocInclude {
            id: node.id,
            content: node.content,
        })
    }

    fn reference_from_node(&mut self, node: Node) -> Result<Reference, NodeParserError> {
        assert_eq!(node.node_type, NodeType::Reference.as_u64());

        let declaration_id = Uuid::parse_str(&node.content).unwrap();

        Ok(Reference {
            id: node.id,
            declaration_id,
            identifier: self.context.get_symbol_identifier(&declaration_id).unwrap(),
        })
    }

    fn return_statement_from_node(
        &mut self,
        node: Node,
    ) -> Result<ReturnStatement, NodeParserError> {
        assert_eq!(node.node_type, NodeType::ReturnStatement.as_u64());
        Ok(ReturnStatement {
            id: node.id,
            value: self.unpack_parse(node.children)?,
        })
    }

    fn string_literal_from_node(&mut self, node: Node) -> Result<StringLiteral, NodeParserError> {
        assert_eq!(node.node_type, NodeType::StringLiteral.as_u64());
        Ok(StringLiteral {
            id: node.id,
            value: node.content,
        })
    }

    fn compound_statement_from_node(
        &mut self,
        node: Node,
    ) -> Result<CompoundStatement, NodeParserError> {
        assert_eq!(node.node_type, NodeType::CompoundStatement.as_u64());

        let mut branch = self.branch();
        let mut code_block: Vec<CLanguageObject> = Vec::new();
        for child in node.children {
            code_block.push(branch.clanguageobject_from_node(child)?)
        }
        Ok(CompoundStatement { 
            id: node.id,
            code_block,
        })
    }
}
