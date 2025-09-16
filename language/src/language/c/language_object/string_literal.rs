use crate::language::c::{
    TreeSitterNodeExt,
    writers::Writer,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub value: String,
}

impl StringLiteral {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_string_literal(&self)
    }
}
