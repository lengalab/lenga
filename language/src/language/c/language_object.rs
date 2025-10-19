pub mod declaration_object;
pub mod expression_object;
pub mod special_object;
pub mod statement_object;

#[cfg(test)]
mod field_inspect_test;

use crate::language::c::{
    language_object::declaration_object::function_declaration::function_parameter::FunctionParameter,
    parsers::{nodes::NodeParserError, text::TreeSitterParserError},
    writers::{Cursor, writer_error::WriterError},
};
use std::{any::Any, fmt::Debug};
use uuid::Uuid;

use declaration_object::{
    DeclarationObject, declaration::Declaration, function_declaration::FunctionDeclaration,
    function_definition::FunctionDefinition, preproc_include::PreprocInclude,
};
use expression_object::{
    ExpressionObject, assignment_expression::AssignmentExpression,
    binary_expression::BinaryExpression, call_expression::CallExpression,
    number_literal::NumberLiteral, reference::Reference, string_literal::StringLiteral,
};
use special_object::{comment::Comment, source_file::SourceFile, unknown::Unknown};
use statement_object::{
    StatementObject,
    compound_statement::CompoundStatement,
    if_statement::{IfStatement, else_clause::ElseClause},
    return_statement::ReturnStatement,
};

// fn write(&self, w: &mut dyn Writer) -> Result<(), WriterError>;
// fn as_any(&self) -> &dyn Any;
#[derive(Debug, PartialEq, Clone, field_inspect_derive::FieldInspect)]
pub enum LanguageObject {
    SourceFile(SourceFile),

    AssignmentExpression(AssignmentExpression),
    BinaryExpression(BinaryExpression),
    CallExpression(CallExpression),
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),

    CompoundStatement(CompoundStatement),
    IfStatement(IfStatement),
    ReturnStatement(ReturnStatement),

    Declaration(Declaration),
    FunctionDeclaration(FunctionDeclaration),
    FunctionDefinition(FunctionDefinition),
    PreprocInclude(PreprocInclude),
    Reference(Reference),

    ElseClause(ElseClause),
    FunctionParameter(FunctionParameter),
    Unknown(Unknown),
    Comment(Comment),
}

impl LanguageObject {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        match self {
            LanguageObject::SourceFile(src_file) => src_file.write(w),
            LanguageObject::AssignmentExpression(assignment_expression) => {
                assignment_expression.write(w)
            }
            LanguageObject::BinaryExpression(binary_expression) => binary_expression.write(w),
            LanguageObject::CallExpression(call_expression) => call_expression.write(w),
            LanguageObject::Comment(comment) => comment.write(w),
            LanguageObject::Declaration(declaration) => declaration.write(w),
            LanguageObject::ElseClause(else_clause) => else_clause.write(w),
            LanguageObject::FunctionDeclaration(function_declaration) => {
                function_declaration.write(w)
            }
            LanguageObject::FunctionDefinition(function_definition) => function_definition.write(w),
            LanguageObject::FunctionParameter(function_parameter) => function_parameter.write(w),
            LanguageObject::IfStatement(if_statement) => if_statement.write(w),
            LanguageObject::NumberLiteral(number_literal) => number_literal.write(w),
            LanguageObject::PreprocInclude(preproc_include) => preproc_include.write(w),
            LanguageObject::Reference(reference) => reference.write(w),
            LanguageObject::ReturnStatement(return_statement) => return_statement.write(w),
            LanguageObject::StringLiteral(string_literal) => string_literal.write(w),
            LanguageObject::CompoundStatement(compound_statement) => compound_statement.write(w),
            LanguageObject::Unknown(unknown) => unknown.write(w),
        }
    }

    pub fn id(&self) -> Uuid {
        match self {
            LanguageObject::SourceFile(src_file) => src_file.id,
            LanguageObject::AssignmentExpression(assignment_expression) => assignment_expression.id,
            LanguageObject::BinaryExpression(binary_expression) => binary_expression.id,
            LanguageObject::CallExpression(call_expression) => call_expression.id,
            LanguageObject::Comment(comment) => comment.id,
            LanguageObject::Declaration(declaration) => declaration.id,
            LanguageObject::ElseClause(else_clause) => else_clause.id,
            LanguageObject::FunctionDeclaration(function_declaration) => function_declaration.id,
            LanguageObject::FunctionDefinition(function_definition) => function_definition.id,
            LanguageObject::FunctionParameter(function_parameter) => function_parameter.id,
            LanguageObject::IfStatement(if_statement) => if_statement.id,
            LanguageObject::NumberLiteral(number_literal) => number_literal.id,
            LanguageObject::PreprocInclude(preproc_include) => preproc_include.id,
            LanguageObject::Reference(reference) => reference.id,
            LanguageObject::ReturnStatement(return_statement) => return_statement.id,
            LanguageObject::StringLiteral(string_literal) => string_literal.id,
            LanguageObject::CompoundStatement(compound_statement) => compound_statement.id,
            LanguageObject::Unknown(unknown) => unknown.id,
        }
    }
}

impl Default for LanguageObject {
    fn default() -> Self {
        LanguageObject::Unknown(Unknown::default())
    }
}

impl crate::language::LanguageObject for LanguageObject {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct ConversionError(String);

impl From<ConversionError> for NodeParserError {
    fn from(value: ConversionError) -> Self {
        NodeParserError::WrongType(value.0)
    }
}

impl From<ConversionError> for TreeSitterParserError {
    fn from(value: ConversionError) -> Self {
        TreeSitterParserError::WrongType(value.0)
    }
}

impl From<ConversionError> for String {
    fn from(value: ConversionError) -> Self {
        value.0
    }
}
