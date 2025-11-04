pub mod node_type;
use std::collections::HashMap;

use uuid::Uuid;

use crate::language::c::language_object::{
    LanguageObject as CLanguageObject,
    declaration_object::{
        declaration::Declaration,
        function_declaration::{FunctionDeclaration, function_parameter::FunctionParameter},
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
        compound_statement::CompoundStatement,
        if_statement::{ElseStatement, IfStatement, else_clause::ElseClause},
        return_statement::ReturnStatement,
    },
};

use crate::language::c::{
    parsers::context::Context,
    writers::{Cursor, node_writer::node_type::NodeType},
};
use crate::node::{Node, ToNode, ToTags};

use super::{Writer, writer_error::WriterError};

pub struct NodeWriter<'a> {
    writer: &'a mut dyn std::io::Write,
    cursor: NodeCursor<'a>,
}

pub struct NodeCursor<'a> {
    pub nodes: Vec<Node>,
    pub context: Context<'a>,
}

impl<'a> NodeCursor<'a> {
    fn new() -> NodeCursor<'a> {
        NodeCursor {
            nodes: Vec::new(),
            context: Context::new(),
        }
    }

    fn branch(&self) -> NodeCursor {
        NodeCursor {
            nodes: vec![],
            context: self.context.branch(),
        }
    }

    fn to_node(&self, value: &CLanguageObject) -> Result<Vec<Node>, WriterError> {
        let mut sub_writer = self.branch();
        value.write(&mut sub_writer)?;
        let children = sub_writer.nodes;
        Ok(children)
    }

    fn to_nodes(&self, code: &Vec<CLanguageObject>) -> Result<Vec<Node>, WriterError> {
        let mut sub_writer = self.branch();
        sub_writer.save_objects(code)?;
        let children = sub_writer.nodes;
        Ok(children)
    }

    fn save_objects(&mut self, code: &Vec<CLanguageObject>) -> Result<(), WriterError> {
        for object in code {
            object.write(self)?;
        }
        Ok(())
    }
}

impl<'a> NodeWriter<'a> {
    pub fn new(writer: &'a mut dyn std::io::Write) -> Self {
        NodeWriter {
            writer,
            cursor: NodeCursor::new(),
        }
    }
}

impl<'a> Writer for NodeWriter<'a> {
    fn write_file(&mut self, src_file: &SourceFile) -> Result<(), WriterError> {
        self.cursor.write_source_file(src_file)?;
        bincode::serialize_into(&mut self.writer, &self.cursor.nodes).unwrap();
        Ok(())
    }
}

impl<'a> Cursor for NodeCursor<'a> {
    fn write_source_file(&mut self, src_file: &SourceFile) -> Result<(), WriterError> {
        self.nodes.push(Node {
            id: Uuid::new_v4(),
            node_type: NodeType::SourceFile.as_u64(),
            content: "".to_string(), // TODO maybe the path?
            tags: HashMap::new(),
            children: self
                .branch()
                .to_nodes(&src_file.code.iter().map(|o| o.clone().into()).collect())?,
        });
        Ok(())
    }

    fn write_unknown(&mut self, unknown: &Unknown) -> Result<(), WriterError> {
        self.nodes.push(Node {
            id: unknown.id,
            node_type: NodeType::Unknown.as_u64(),
            content: unknown.content.clone(),
            tags: HashMap::new(),
            children: vec![],
        });
        Ok(())
    }

    fn write_assignment_expression(
        &mut self,
        assignment_expression: &AssignmentExpression,
    ) -> Result<(), WriterError> {
        self.nodes.push(Node {
            id: assignment_expression.id,
            node_type: NodeType::AssignmentExpression.as_u64(),
            content: assignment_expression.id_declaration.to_string(),
            tags: HashMap::new(),
            children: self.to_node(&assignment_expression.value.as_language_object())?,
        });
        Ok(())
    }

    fn write_binary_expression(
        &mut self,
        binary_expression: &BinaryExpression,
    ) -> Result<(), WriterError> {
        let left = self.to_node(&binary_expression.left.as_language_object())?;
        let right = self.to_node(&binary_expression.right.as_language_object())?;
        self.nodes.push(Node {
            id: Uuid::new_v4(),
            node_type: NodeType::BinaryExpression.as_u64(),
            content: binary_expression.operator.clone(),
            tags: vec![("left", left), ("right", right)].to_tags(),
            children: vec![],
        });
        Ok(())
    }

    fn write_call_expression(
        &mut self,
        call_expression: &CallExpression,
    ) -> Result<(), WriterError> {
        self.nodes.push(Node {
            id: call_expression.id,
            node_type: NodeType::CallExpression.as_u64(),
            content: if call_expression.id_declaration != Uuid::nil() {
                call_expression.id_declaration.to_string()
            } else {
                call_expression.identifier.clone()
            }, // TODO we won't need to save this when we figure out importing symbols from libraries
            tags: HashMap::new(),
            children: self.to_nodes(&ExpressionObject::as_language_objects(
                &call_expression.argument_list,
            ))?,
        });
        Ok(())
    }

    fn write_comment(&mut self, comment: &Comment) -> Result<(), WriterError> {
        self.nodes.push(Node {
            id: Uuid::new_v4(),
            node_type: NodeType::Comment.as_u64(),
            content: comment.content.clone(),
            tags: HashMap::new(),
            children: vec![],
        });
        Ok(())
    }

    fn write_declaration(&mut self, declaration: &Declaration) -> Result<(), WriterError> {
        self.nodes.push(Node {
            id: declaration.id,
            node_type: NodeType::Declaration.as_u64(),
            content: declaration.identifier.clone(),
            tags: vec![(
                "type",
                vec![
                    declaration
                        .primitive_type
                        .as_str()
                        .to_str_node(NodeType::TypeDescription.as_u64()),
                ],
            )]
            .to_tags(),
            children: if let Some(value) = &declaration.value {
                self.to_node(&value.as_language_object())?
            } else {
                vec![]
            },
        });
        Ok(())
    }

    fn write_else_clause(&mut self, else_clause: &ElseClause) -> Result<(), WriterError> {
        self.nodes.push(Node {
            id: Uuid::new_v4(),
            node_type: NodeType::ElseClause.as_u64(),
            content: "".to_string(),
            tags: HashMap::new(),
            children: {
                let mut branch = self.branch();
                else_clause.body.write(&mut branch)?;
                branch.nodes
            },
        });
        Ok(())
    }

    fn write_function_declaration(
        &mut self,
        function_declaration: &FunctionDeclaration,
    ) -> Result<(), WriterError> {
        let args_mapped = function_declaration
            .parameter_list
            .iter()
            .map(
                |FunctionParameter {
                     id,
                     identifier,
                     param_type,
                 }| {
                    Node {
                        id: *id,
                        node_type: NodeType::FunctionParameter.as_u64(),
                        content: identifier.clone(),
                        tags: vec![(
                            "type",
                            vec![
                                param_type
                                    .as_str()
                                    .to_str_node(NodeType::TypeDescription.as_u64()),
                            ],
                        )]
                        .to_tags(),
                        children: vec![],
                    }
                },
            )
            .collect::<Vec<Node>>();

        self.nodes.push(Node {
            node_type: NodeType::FunctionDeclaration.as_u64(),
            id: self
                .context
                .insert_symbol_with_id(
                    &function_declaration.identifier,
                    function_declaration.id,
                    true,
                )
                .unwrap_or(function_declaration.id),
            tags: vec![
                (
                    "return_type",
                    vec![
                        function_declaration
                            .return_type
                            .as_str()
                            .to_str_node(NodeType::TypeDescription.as_u64()),
                    ],
                ),
                ("args", args_mapped),
            ]
            .to_tags(),
            children: vec![],
            content: function_declaration.identifier.clone(),
        });
        Ok(())
    }

    fn write_function_definition(
        &mut self,
        function_definition: &FunctionDefinition,
    ) -> Result<(), WriterError> {
        let mut subcontext = self.branch();

        let args_mapped = function_definition
            .parameter_list
            .iter()
            .map(
                |FunctionParameter {
                     id,
                     identifier,
                     param_type,
                 }| {
                    Node {
                        id: *id,
                        node_type: NodeType::FunctionParameter.as_u64(),
                        content: identifier.clone(),
                        tags: vec![(
                            "type",
                            vec![
                                param_type
                                    .as_str()
                                    .to_str_node(NodeType::TypeDescription.as_u64()),
                            ],
                        )]
                        .to_tags(),
                        children: vec![],
                    }
                },
            )
            .collect::<Vec<Node>>();

        subcontext.write_compound_statement(&function_definition.compound_statement)?;
        let children = subcontext.nodes;

        self.nodes.push(Node {
            node_type: NodeType::FunctionDefinition.as_u64(),
            id: function_definition.id,
            tags: vec![
                (
                    "return_type",
                    vec![
                        function_definition
                            .return_type
                            .as_str()
                            .to_str_node(NodeType::TypeDescription.as_u64()),
                    ],
                ),
                ("args", args_mapped),
            ]
            .to_tags(),
            children,
            content: function_definition.identifier.clone(),
        });
        Ok(())
    }

    fn write_if_statement(&mut self, if_statement: &IfStatement) -> Result<(), WriterError> {
        let mut tags = vec![(
            "condition",
            self.to_node(&if_statement.condition.as_language_object())?,
        )]
        .to_tags();

        if let Some(else_clause) = &if_statement.else_statement {
            match else_clause {
                ElseStatement::ElseIf(else_if) => {
                    tags.insert(
                        "else_if".to_string(),
                        self.to_node(&CLanguageObject::IfStatement(*else_if.clone()))?,
                    );
                }
                ElseStatement::ElseClause(else_clause) => {
                    tags.insert(
                        "else_clause".to_string(),
                        self.to_node(&CLanguageObject::ElseClause(*else_clause.clone()))?,
                    );
                }
            }
        };
        self.nodes.push(Node {
            id: Uuid::new_v4(),
            node_type: NodeType::IfStatement.as_u64(),
            content: "".to_string(),
            tags,
            children: {
                let mut branch = self.branch();
                if_statement.body.write(&mut branch)?;
                branch.nodes
            },
        });
        Ok(())
    }

    fn write_number_literal(&mut self, number_literal: &NumberLiteral) -> Result<(), WriterError> {
        self.nodes.push(
            number_literal
                .value
                .to_string()
                .to_str_node(NodeType::NumberLiteral.as_u64()),
        );
        Ok(())
    }

    fn write_preproc_include(
        &mut self,
        preproc_include: &PreprocInclude,
    ) -> Result<(), WriterError> {
        self.nodes.push(Node {
            node_type: NodeType::PreprocInclude.as_u64(),
            id: Uuid::new_v4(),
            tags: HashMap::new(),
            content: preproc_include.content.clone(),
            children: vec![],
        });
        Ok(())
    }

    fn write_reference(&mut self, reference: &Reference) -> Result<(), WriterError> {
        let value = Node {
            id: reference.id,
            content: reference.declaration_id.to_string(),
            node_type: NodeType::Reference.as_u64(),
            tags: HashMap::new(),
            children: vec![],
        };
        self.nodes.push(value);
        Ok(())
    }

    fn write_return_statement(
        &mut self,
        return_statement: &ReturnStatement,
    ) -> Result<(), WriterError> {
        self.nodes.push(Node {
            node_type: NodeType::ReturnStatement.as_u64(),
            id: Uuid::new_v4(),
            tags: HashMap::new(),
            content: "".to_string(),
            children: match &return_statement.value {
                Some(value) => self.to_node(&value.as_language_object())?,
                None => vec![],
            },
        });
        Ok(())
    }

    fn write_string_literal(&mut self, string_literal: &StringLiteral) -> Result<(), WriterError> {
        self.nodes.push(
            string_literal
                .value
                .to_string()
                .to_str_node(NodeType::StringLiteral.as_u64()),
        );
        Ok(())
    }

    fn write_compound_statement(
        &mut self,
        compound_statement: &CompoundStatement,
    ) -> Result<(), WriterError> {
        self.nodes.push(Node {
            id: Uuid::new_v4(),
            node_type: NodeType::CompoundStatement.as_u64(),
            content: "".to_string(),
            tags: HashMap::new(),
            children: self.to_nodes(
                &compound_statement
                    .code_block
                    .iter()
                    .map(|o| o.clone().into())
                    .collect(),
            )?,
        });
        Ok(())
    }
}
