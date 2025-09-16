use std::any::Any;

use crate::language::{
    LanguageObject,
    c::{
        C, TreeSitterNodeExt,
        language_object::LanguageObject as CLanguageObject,
        writers::Writer,
        writers::{Cursor, writer_error::WriterError},
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub content: String,
}

impl Comment {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_comment(&self)
    }
}
