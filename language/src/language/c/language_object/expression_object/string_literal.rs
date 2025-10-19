use uuid::Uuid;

use crate::language::c::writers::{Cursor, writer_error::WriterError};

#[derive(Debug, Clone, field_inspect_derive::FieldInspect)]
pub struct StringLiteral {
    pub id: Uuid,
    pub value: String,
}

impl StringLiteral {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_string_literal(&self)
    }
}

impl Default for StringLiteral {
    fn default() -> Self {
        StringLiteral {
            id: Uuid::new_v4(),
            value: String::new(),
        }
    }
}

impl PartialEq for StringLiteral {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
