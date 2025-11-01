use uuid::Uuid;

use crate::language::c::{
    language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, field_inspect_derive::FieldInspect)]
pub struct ElseClause {
    pub id: Uuid,
    pub body: Box<CompoundStatementObject>,
}

impl ElseClause {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_else_clause(self)
    }
}

impl PartialEq for ElseClause {
    fn eq(&self, other: &Self) -> bool {
        // crate::language::PartialEqAny::eq_dyn(&self.condition, &other.condition)
        crate::language::PartialEqAny::eq_dyn(&self.body, &other.body)
    }
}

impl Default for ElseClause {
    fn default() -> Self {
        ElseClause {
            id: Uuid::new_v4(),
            body: Box::new(CompoundStatementObject::default()),
        }
    }
}
