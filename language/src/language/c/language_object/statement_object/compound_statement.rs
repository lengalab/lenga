use uuid::Uuid;

use crate::language::c::writers::{Cursor, writer_error::WriterError};

pub mod compound_statement_object;

#[derive(Debug, Clone, field_inspect_derive::FieldInspect)]
pub struct CompoundStatement {
    pub id: Uuid,
    pub code_block: Vec<compound_statement_object::CompoundStatementObject>,
}

impl CompoundStatement {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_compound_statement(self)
    }
}

impl Default for CompoundStatement {
    fn default() -> Self {
        CompoundStatement {
            id: Uuid::new_v4(),
            code_block: vec![],
        }
    }
}

impl PartialEq for CompoundStatement {
    fn eq(&self, other: &Self) -> bool {
        crate::language::PartialEqAny::eq_dyn(&self.code_block, &other.code_block)
    }
}
