use crate::language::c::{
    writers::Writer,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral {
    pub value: String,
}

impl NumberLiteral {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_number_literal(&self)
    }
}
