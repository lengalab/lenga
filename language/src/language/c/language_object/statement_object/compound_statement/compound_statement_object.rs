use crate::language::c::language_object::LanguageObject;
use crate::language::c::language_object::declaration_object::declaration;
use crate::language::c::language_object::expression_object::{
    assignment_expression, binary_expression, call_expression, number_literal, reference,
    string_literal,
};
use crate::language::c::language_object::special_object::comment;
use crate::language::c::language_object::special_object::unknown::Unknown;
use crate::language::c::language_object::statement_object::{
    compound_statement, if_statement, return_statement,
};

#[derive(
    Debug,
    PartialEq,
    Clone,
    field_inspect_derive::VariantProvider,
    field_inspect_derive::FieldInspect,
)]
pub enum CompoundStatementObject {
    Declaration(declaration::Declaration),

    AssignmentExpression(assignment_expression::AssignmentExpression),
    BinaryExpression(binary_expression::BinaryExpression),
    CallExpression(call_expression::CallExpression),
    NumberLiteral(number_literal::NumberLiteral),
    Reference(reference::Reference),
    StringLiteral(string_literal::StringLiteral),

    CompoundStatement(compound_statement::CompoundStatement),
    IfStatement(if_statement::IfStatement),
    ReturnStatement(return_statement::ReturnStatement),

    Comment(comment::Comment), // TODO: This doesn't belong here. To avoid putting comments in every enum, I woud try to match them with their corresponding node
    Unknown(Unknown),
}

impl CompoundStatementObject {
    pub fn write(
        &self,
        w: &mut dyn crate::language::c::writers::Cursor,
    ) -> Result<(), crate::language::c::writers::writer_error::WriterError> {
        match self {
            CompoundStatementObject::Declaration(decl) => decl.write(w),

            CompoundStatementObject::AssignmentExpression(expr) => expr.write(w),
            CompoundStatementObject::BinaryExpression(expr) => expr.write(w),
            CompoundStatementObject::CallExpression(expr) => expr.write(w),
            CompoundStatementObject::NumberLiteral(expr) => expr.write(w),
            CompoundStatementObject::Reference(expr) => expr.write(w),
            CompoundStatementObject::StringLiteral(expr) => expr.write(w),

            CompoundStatementObject::CompoundStatement(compound_statement) => {
                compound_statement.write(w)
            }
            CompoundStatementObject::IfStatement(if_statement) => if_statement.write(w),
            CompoundStatementObject::ReturnStatement(return_statement) => return_statement.write(w),

            CompoundStatementObject::Unknown(unknown) => unknown.write(w),
            CompoundStatementObject::Comment(comment) => comment.write(w),
        }
    }
}

impl Default for CompoundStatementObject {
    fn default() -> Self {
        CompoundStatementObject::Unknown(Unknown::default())
    }
}

impl From<CompoundStatementObject> for LanguageObject {
    fn from(value: CompoundStatementObject) -> Self {
        match value {
            CompoundStatementObject::Declaration(decl) => LanguageObject::Declaration(decl.clone()),

            CompoundStatementObject::AssignmentExpression(expr) => {
                LanguageObject::AssignmentExpression(expr.clone())
            }
            CompoundStatementObject::BinaryExpression(expr) => {
                LanguageObject::BinaryExpression(expr.clone())
            }
            CompoundStatementObject::CallExpression(expr) => {
                LanguageObject::CallExpression(expr.clone())
            }
            CompoundStatementObject::NumberLiteral(expr) => {
                LanguageObject::NumberLiteral(expr.clone())
            }
            CompoundStatementObject::Reference(expr) => LanguageObject::Reference(expr.clone()),
            CompoundStatementObject::StringLiteral(expr) => {
                LanguageObject::StringLiteral(expr.clone())
            }

            CompoundStatementObject::CompoundStatement(compound_statement) => {
                LanguageObject::CompoundStatement(compound_statement.clone())
            }
            CompoundStatementObject::IfStatement(if_statement) => {
                LanguageObject::IfStatement(if_statement.clone())
            }
            CompoundStatementObject::ReturnStatement(return_statement) => {
                LanguageObject::ReturnStatement(return_statement.clone())
            }

            CompoundStatementObject::Unknown(unknown) => LanguageObject::Unknown(unknown.clone()),
            CompoundStatementObject::Comment(comment) => LanguageObject::Comment(comment.clone()),
        }
    }
}

impl TryFrom<LanguageObject> for CompoundStatementObject {
    type Error = crate::language::c::language_object::ConversionError;

    fn try_from(value: LanguageObject) -> Result<Self, Self::Error> {
        match value {
            LanguageObject::Declaration(decl) => Ok(CompoundStatementObject::Declaration(decl)),

            LanguageObject::AssignmentExpression(assignment_expression) => Ok(
                CompoundStatementObject::AssignmentExpression(assignment_expression),
            ),
            LanguageObject::BinaryExpression(binary_expression) => {
                Ok(CompoundStatementObject::BinaryExpression(binary_expression))
            }
            LanguageObject::CallExpression(call_expression) => {
                Ok(CompoundStatementObject::CallExpression(call_expression))
            }
            LanguageObject::NumberLiteral(number_literal) => {
                Ok(CompoundStatementObject::NumberLiteral(number_literal))
            }
            LanguageObject::Reference(reference) => {
                Ok(CompoundStatementObject::Reference(reference))
            }
            LanguageObject::StringLiteral(string_literal) => {
                Ok(CompoundStatementObject::StringLiteral(string_literal))
            }

            LanguageObject::CompoundStatement(compound_statement) => Ok(
                CompoundStatementObject::CompoundStatement(compound_statement),
            ),
            LanguageObject::IfStatement(if_statement) => {
                Ok(CompoundStatementObject::IfStatement(if_statement))
            }
            LanguageObject::ReturnStatement(return_statement) => {
                Ok(CompoundStatementObject::ReturnStatement(return_statement))
            }
            LanguageObject::Unknown(unknown) => Ok(CompoundStatementObject::Unknown(unknown)),
            LanguageObject::Comment(comment) => Ok(CompoundStatementObject::Comment(comment)),
            _ => Err(crate::language::c::language_object::ConversionError(
                "Cannot convert LanguageObject to CompoundStatementObject".into(),
            )),
        }
    }
}
