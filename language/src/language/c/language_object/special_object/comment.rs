use uuid::Uuid;

use crate::language::c::writers::{Cursor, writer_error::WriterError};

#[derive(Debug, Clone, lenga_field_inspect_derive::FieldInspect)]
pub struct Comment {
    pub id: Uuid,
    pub content: String,
}

impl Comment {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_comment(self)
    }
}

impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl Default for Comment {
    fn default() -> Self {
        Comment {
            id: Uuid::new_v4(),
            content: String::new(),
        }
    }
}
