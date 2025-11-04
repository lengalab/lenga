use uuid::Uuid;

use crate::language::c::writers::{Cursor, writer_error::WriterError};

#[derive(Debug, Clone, lenga_field_inspect_derive::FieldInspect)]
pub struct Unknown {
    pub id: Uuid,
    pub content: String,
}

impl Unknown {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_unknown(self)
    }
}

impl PartialEq for Unknown {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl Default for Unknown {
    fn default() -> Self {
        Unknown {
            id: Uuid::new_v4(),
            content: String::new(),
        }
    }
}
