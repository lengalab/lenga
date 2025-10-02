pub mod assignment_expression;
pub mod binary_expression;
pub mod call_expression;
pub mod comment;
pub mod compound_statement;
pub mod declaration;
pub mod else_clause;
pub mod expression_statement;
pub mod function_declaration;
pub mod function_definition;
pub mod function_parameter;
pub mod if_statement;
pub mod number_literal;
pub mod preproc_include;
pub mod reference;
pub mod return_statement;
pub mod source_file;
pub mod string_literal;

use crate::language::c::{
    language_object::source_file::SourceFile,
    writers::{Cursor, writer_error::WriterError},
};
use std::{any::Any, fmt::Debug};
use uuid::Uuid;

use assignment_expression::AssignmentExpression;
use binary_expression::BinaryExpression;
use call_expression::CallExpression;
use comment::Comment;
use compound_statement::CompoundStatement;
use declaration::Declaration;
use else_clause::ElseClause;
use expression_statement::ExpressionStatement;
use function_declaration::FunctionDeclaration;
use function_definition::FunctionDefinition;
use function_parameter::FunctionParameter;
use if_statement::IfStatement;
use number_literal::NumberLiteral;
use preproc_include::PreprocInclude;
use reference::Reference;
use return_statement::ReturnStatement;
use string_literal::StringLiteral;

// fn write(&self, w: &mut dyn Writer) -> Result<(), WriterError>;
// fn as_any(&self) -> &dyn Any;
#[derive(Debug, PartialEq, Clone)]
pub enum LanguageObject {
    SourceFile(SourceFile),
    AssignmentExpression(AssignmentExpression),
    BinaryExpression(BinaryExpression),
    CallExpression(CallExpression),
    Comment(Comment),
    Declaration(Declaration),
    ElseClause(ElseClause),
    ExpressionStatement(ExpressionStatement),
    FunctionDeclaration(FunctionDeclaration),
    FunctionDefinition(FunctionDefinition),
    FunctionParameter(FunctionParameter),
    IfStatement(IfStatement),
    NumberLiteral(NumberLiteral),
    PreprocInclude(PreprocInclude),
    Reference(Reference),
    ReturnStatement(ReturnStatement),
    StringLiteral(StringLiteral),
    CompoundStatement(CompoundStatement),
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
            LanguageObject::ExpressionStatement(expression_statement) => {
                expression_statement.write(w)
            }
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
        }
    }

    pub fn id(&self) -> Uuid {
        match self {
            LanguageObject::SourceFile(src_file) => src_file.id,
            LanguageObject::AssignmentExpression(assignment_expression) =>assignment_expression.id,
            LanguageObject::BinaryExpression(binary_expression) => binary_expression.id,
            LanguageObject::CallExpression(call_expression) => call_expression.id,
            LanguageObject::Comment(comment) => comment.id,
            LanguageObject::Declaration(declaration) => declaration.id,
            LanguageObject::ElseClause(else_clause) => else_clause.id,
            LanguageObject::ExpressionStatement(expression_statement) => expression_statement.id,
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
        }
    }
}

impl crate::language::LanguageObject for LanguageObject {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
