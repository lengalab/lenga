use uuid::Uuid;

use crate::language::c::{
    language_object::{expression_object::ExpressionObject, statement_object::StatementObject},
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, field_inspect_derive::FieldInspect)]
pub struct ElseClause {
    pub id: Uuid,
    pub condition: Option<Box<ExpressionObject>>,
    pub compound_statement: Box<StatementObject>,
}

impl ElseClause {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_else_clause(&self)
    }
}

impl PartialEq for ElseClause {
    fn eq(&self, other: &Self) -> bool {
        // crate::language::PartialEqAny::eq_dyn(&self.condition, &other.condition)
        crate::language::PartialEqAny::eq_dyn(&self.compound_statement, &other.compound_statement)
    }
}

impl Default for ElseClause {
    fn default() -> Self {
        ElseClause {
            id: Uuid::new_v4(),
            condition: None,
            compound_statement: Box::new(StatementObject::default()),
        }
    }
}
