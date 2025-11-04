use uuid::Uuid;

use crate::language::c::writers::{Cursor, writer_error::WriterError};

#[derive(Debug, Clone, lenga_field_inspect_derive::FieldInspect)]
pub struct PreprocInclude {
    pub id: Uuid,
    pub content: String,
}

impl PreprocInclude {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_preproc_include(self)
    }
}

impl PartialEq for PreprocInclude {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl Default for PreprocInclude {
    fn default() -> Self {
        PreprocInclude {
            id: Uuid::new_v4(),
            content: String::new(),
        }
    }
}
