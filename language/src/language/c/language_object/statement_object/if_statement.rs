use uuid::Uuid;

pub mod else_clause;

use crate::language::c::{
    language_object::{
        expression_object::ExpressionObject,
        statement_object::{StatementObject, if_statement::else_clause::ElseClause},
    },
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, field_inspect_derive::FieldInspect)]
pub struct IfStatement {
    pub id: Uuid,
    pub condition: Box<ExpressionObject>,
    pub compound_statement: Box<StatementObject>,
    pub else_clause: Option<ElseClause>,
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
            else_clause: None,
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
            && self.else_clause == other.else_clause
    }
}
