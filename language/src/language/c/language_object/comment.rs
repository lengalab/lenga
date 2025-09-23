use uuid::Uuid;

use crate::language::c::{
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone)]
pub struct Comment {
    pub id: Uuid,
    pub content: String,
}

impl Comment {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_comment(&self)
    }
}

impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}
