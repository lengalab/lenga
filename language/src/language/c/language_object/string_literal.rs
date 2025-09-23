use uuid::Uuid;

use crate::language::c::{
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone)]
pub struct StringLiteral {
    pub id: Uuid,
    pub value: String,
}

impl StringLiteral {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_string_literal(&self)
    }
}

impl PartialEq for StringLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}