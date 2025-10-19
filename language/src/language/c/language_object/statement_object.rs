use crate::language::c::language_object::{
    ConversionError, LanguageObject, special_object::unknown::Unknown,
};

pub mod compound_statement;
pub mod if_statement;
pub mod return_statement;

#[derive(
    Debug,
    PartialEq,
    Clone,
    field_inspect_derive::VariantProvider,
    field_inspect_derive::FieldInspect,
)]
pub enum StatementObject {
    CompoundStatement(compound_statement::CompoundStatement),
    IfStatement(if_statement::IfStatement),
    ReturnStatement(return_statement::ReturnStatement),
    Unknown(Unknown),
}

impl StatementObject {
    pub fn write(
        &self,
        w: &mut dyn crate::language::c::writers::Cursor,
    ) -> Result<(), crate::language::c::writers::writer_error::WriterError> {
        match self {
            StatementObject::CompoundStatement(stmt) => stmt.write(w),
            StatementObject::IfStatement(stmt) => stmt.write(w),
            StatementObject::ReturnStatement(stmt) => stmt.write(w),
            StatementObject::Unknown(stmt) => stmt.write(w),
        }
    }
}

impl Default for StatementObject {
    fn default() -> Self {
        StatementObject::Unknown(Unknown::default())
    }
}

impl From<StatementObject> for LanguageObject {
    fn from(value: StatementObject) -> Self {
        match value {
            StatementObject::CompoundStatement(stmt) => LanguageObject::CompoundStatement(stmt),
            StatementObject::IfStatement(stmt) => LanguageObject::IfStatement(stmt),
            StatementObject::ReturnStatement(stmt) => LanguageObject::ReturnStatement(stmt),
            StatementObject::Unknown(stmt) => LanguageObject::Unknown(stmt),
        }
    }
}

impl TryFrom<compound_statement::CompoundStatement> for Box<StatementObject> {
    type Error = ConversionError;

    fn try_from(value: compound_statement::CompoundStatement) -> Result<Self, Self::Error> {
        Ok(Box::new(StatementObject::CompoundStatement(value)))
    }
}
