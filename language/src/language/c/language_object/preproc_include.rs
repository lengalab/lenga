use uuid::Uuid;

use crate::language::c::{
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone)]
pub struct PreprocInclude {
    pub id: Uuid,
    pub content: String,
}

impl PreprocInclude {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_preproc_include(&self)
    }
}

impl PartialEq for PreprocInclude {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}
