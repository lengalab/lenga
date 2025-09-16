use uuid::Uuid;

use crate::language::c::writers::{Cursor, writer_error::WriterError};

#[derive(Debug, Clone, PartialEq)]
/// ```c
/// a
/// ```
pub struct Reference {
    pub id: Uuid,
    pub identifier: String,
}

impl Reference {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_reference(&self)
    }
}
