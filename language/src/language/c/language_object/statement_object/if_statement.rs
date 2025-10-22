use uuid::Uuid;

pub mod else_clause;

use crate::language::c::{
    language_object::{
        LanguageObject,
        expression_object::ExpressionObject,
        statement_object::{StatementObject, if_statement::else_clause::ElseClause},
    },
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, field_inspect_derive::FieldInspect)]
pub enum ElseStatement {
    ElseIf(Box<IfStatement>),
    ElseClause(Box<ElseClause>),
}

impl ElseStatement {
    pub fn write(
        &self,
        w: &mut dyn crate::language::c::writers::Cursor,
    ) -> Result<(), crate::language::c::writers::writer_error::WriterError> {
        match self {
            ElseStatement::ElseIf(stmt) => stmt.write(w),
            ElseStatement::ElseClause(stmt) => stmt.write(w),
        }
    }
}

impl From<ElseStatement> for LanguageObject {
    fn from(else_statement: ElseStatement) -> Self {
        match else_statement {
            ElseStatement::ElseIf(stmt) => LanguageObject::IfStatement(*stmt),
            ElseStatement::ElseClause(stmt) => LanguageObject::ElseClause(*stmt),
        }
    }
}

impl TryFrom<LanguageObject> for ElseStatement {
    type Error = crate::language::c::language_object::ConversionError;

    fn try_from(else_statement: LanguageObject) -> Result<Self, Self::Error> {
        match else_statement {
            LanguageObject::IfStatement(stmt) => Ok(ElseStatement::ElseIf(Box::new(stmt))),
            LanguageObject::ElseClause(stmt) => Ok(ElseStatement::ElseClause(Box::new(stmt))),
            _ => Err(crate::language::c::language_object::ConversionError(
                "Cannot convert LanguageObject to ElseStatement".into(),
            )),
        }
    }
}

#[derive(Debug, Clone, field_inspect_derive::FieldInspect)]
pub struct IfStatement {
    pub id: Uuid,
    pub condition: Box<ExpressionObject>,
    pub compound_statement: Box<StatementObject>,
    pub else_statement: Option<ElseStatement>,
}

impl IfStatement {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_if_statement(&self)
    }
}

impl Default for IfStatement {
    fn default() -> Self {
        IfStatement {
            id: Uuid::new_v4(),
            condition: Box::new(ExpressionObject::default()),
            compound_statement: Box::new(StatementObject::default()),
            else_statement: None,
        }
    }
}

impl PartialEq for IfStatement {
    fn eq(&self, other: &Self) -> bool {
        crate::language::PartialEqAny::eq_dyn(&self.condition, &other.condition)
            && crate::language::PartialEqAny::eq_dyn(
                &self.compound_statement,
                &other.compound_statement,
            )
            && self.else_statement == other.else_statement
    }
}

impl PartialEq for ElseStatement {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ElseStatement::ElseIf(a), ElseStatement::ElseIf(b)) => a == b,
            (ElseStatement::ElseClause(a), ElseStatement::ElseClause(b)) => a == b,
            _ => false,
        }
    }
}
