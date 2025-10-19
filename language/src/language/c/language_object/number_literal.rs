use uuid::Uuid;

use crate::language::c::{
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone)]
pub struct NumberLiteral {
    pub id: Uuid,
    pub value: String,
}

impl NumberLiteral {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_number_literal(&self)
    }
}

impl PartialEq for NumberLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
