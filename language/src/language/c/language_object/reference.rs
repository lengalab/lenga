use uuid::Uuid;

use crate::language::c::writers::{Cursor, writer_error::WriterError};

#[derive(Debug, Clone)]
/// ```c
/// a
/// ```
pub struct Reference {
    pub id: Uuid,
    pub declaration_id: Uuid,
    pub identifier: String,
}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        self.declaration_id == other.declaration_id
            && self.identifier == other.identifier
    }
}

impl Reference {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_reference(&self)
    }
}
