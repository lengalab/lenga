use core::panic;

use crate::language::c::language_object::{
    ConversionError, LanguageObject, special_object::unknown::Unknown,
};

pub mod assignment_expression;
pub mod binary_expression;
pub mod call_expression;
pub mod number_literal;
pub mod reference;
pub mod string_literal;

#[derive(
    Debug,
    PartialEq,
    Clone,
    field_inspect_derive::VariantProvider,
    field_inspect_derive::FieldInspect,
)]
pub enum ExpressionObject {
    AssignmentExpression(assignment_expression::AssignmentExpression),
    BinaryExpression(binary_expression::BinaryExpression),
    CallExpression(call_expression::CallExpression),
    NumberLiteral(number_literal::NumberLiteral),
    Reference(reference::Reference),
    StringLiteral(string_literal::StringLiteral),
    Unknown(Unknown),
}

impl ExpressionObject {
    pub fn write(
        &self,
        w: &mut dyn crate::language::c::writers::Cursor,
    ) -> Result<(), crate::language::c::writers::writer_error::WriterError> {
        match self {
            ExpressionObject::AssignmentExpression(expr) => expr.write(w),
            ExpressionObject::BinaryExpression(expr) => expr.write(w),
            ExpressionObject::CallExpression(expr) => expr.write(w),
            ExpressionObject::NumberLiteral(expr) => expr.write(w),
            ExpressionObject::Reference(expr) => expr.write(w),
            ExpressionObject::StringLiteral(expr) => expr.write(w),
            ExpressionObject::Unknown(expr) => expr.write(w),
        }
    }

    pub fn as_language_object(&self) -> LanguageObject {
        match self {
            ExpressionObject::AssignmentExpression(expr) => {
                LanguageObject::AssignmentExpression(expr.clone())
            }
            ExpressionObject::BinaryExpression(expr) => {
                LanguageObject::BinaryExpression(expr.clone())
            }
            ExpressionObject::CallExpression(expr) => LanguageObject::CallExpression(expr.clone()),
            ExpressionObject::NumberLiteral(expr) => LanguageObject::NumberLiteral(expr.clone()),
            ExpressionObject::Reference(expr) => LanguageObject::Reference(expr.clone()),
            ExpressionObject::StringLiteral(expr) => LanguageObject::StringLiteral(expr.clone()),
            ExpressionObject::Unknown(expr) => LanguageObject::Unknown(expr.clone()),
        }
    }

    pub fn as_language_objects(vec: &Vec<Self>) -> Vec<LanguageObject> {
        vec.iter().map(|e| e.as_language_object()).collect()
    }
}

impl Default for ExpressionObject {
    fn default() -> Self {
        ExpressionObject::Unknown(Unknown::default())
    }
}

impl From<ExpressionObject> for LanguageObject {
    fn from(value: ExpressionObject) -> Self {
        match value {
            ExpressionObject::AssignmentExpression(expr) => {
                LanguageObject::AssignmentExpression(expr)
            }
            ExpressionObject::BinaryExpression(expr) => LanguageObject::BinaryExpression(expr),
            ExpressionObject::CallExpression(expr) => LanguageObject::CallExpression(expr),
            ExpressionObject::NumberLiteral(expr) => LanguageObject::NumberLiteral(expr),
            ExpressionObject::Reference(expr) => LanguageObject::Reference(expr),
            ExpressionObject::StringLiteral(expr) => LanguageObject::StringLiteral(expr),
            ExpressionObject::Unknown(expr) => LanguageObject::Unknown(expr),
        }
    }
}

impl From<Box<ExpressionObject>> for Box<LanguageObject> {
    fn from(value: Box<ExpressionObject>) -> Self {
        match *value {
            ExpressionObject::AssignmentExpression(expr) => {
                Box::new(LanguageObject::AssignmentExpression(expr))
            }
            ExpressionObject::BinaryExpression(expr) => {
                Box::new(LanguageObject::BinaryExpression(expr))
            }
            ExpressionObject::CallExpression(expr) => {
                Box::new(LanguageObject::CallExpression(expr))
            }
            ExpressionObject::NumberLiteral(expr) => Box::new(LanguageObject::NumberLiteral(expr)),
            ExpressionObject::Reference(expr) => Box::new(LanguageObject::Reference(expr)),
            ExpressionObject::StringLiteral(expr) => Box::new(LanguageObject::StringLiteral(expr)),
            ExpressionObject::Unknown(expr) => Box::new(LanguageObject::Unknown(expr)),
        }
    }
}

impl TryFrom<LanguageObject> for ExpressionObject {
    type Error = ConversionError;
    fn try_from(value: LanguageObject) -> Result<Self, Self::Error> {
        match value {
            LanguageObject::AssignmentExpression(expr) => {
                Ok(ExpressionObject::AssignmentExpression(expr))
            }
            LanguageObject::BinaryExpression(expr) => Ok(ExpressionObject::BinaryExpression(expr)),
            LanguageObject::CallExpression(expr) => Ok(ExpressionObject::CallExpression(expr)),
            LanguageObject::NumberLiteral(expr) => Ok(ExpressionObject::NumberLiteral(expr)),
            LanguageObject::Reference(expr) => Ok(ExpressionObject::Reference(expr)),
            LanguageObject::StringLiteral(expr) => Ok(ExpressionObject::StringLiteral(expr)),
            LanguageObject::Unknown(expr) => Ok(ExpressionObject::Unknown(expr)),
            _ => Err(ConversionError(
                "Cannot convert LanguageObject to ExpressionObject".into(),
            )),
        }
    }
}

impl TryFrom<Box<LanguageObject>> for Box<ExpressionObject> {
    type Error = ConversionError;
    fn try_from(value: Box<LanguageObject>) -> Result<Self, Self::Error> {
        match *value {
            LanguageObject::AssignmentExpression(expr) => {
                Ok(ExpressionObject::AssignmentExpression(expr).into())
            }
            LanguageObject::BinaryExpression(expr) => {
                Ok(ExpressionObject::BinaryExpression(expr).into())
            }
            LanguageObject::CallExpression(expr) => {
                Ok(ExpressionObject::CallExpression(expr).into())
            }
            LanguageObject::NumberLiteral(expr) => Ok(ExpressionObject::NumberLiteral(expr).into()),
            LanguageObject::Reference(expr) => Ok(ExpressionObject::Reference(expr).into()),
            LanguageObject::StringLiteral(expr) => Ok(ExpressionObject::StringLiteral(expr).into()),
            LanguageObject::Unknown(expr) => Ok(ExpressionObject::Unknown(expr).into()),
            _ => Err(ConversionError(
                "Cannot convert LanguageObject to ExpressionObject".into(),
            )),
        }
    }
}
