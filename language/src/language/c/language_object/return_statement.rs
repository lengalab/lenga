use std::any::Any;

use crate::language::{
    LanguageObject,
    c::{
        C,
        language_object::LanguageObject as CLanguageObject,
        writers::{Cursor, writer_error::WriterError},
    },
};

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Box<CLanguageObject>,
}

impl PartialEq for ReturnStatement {
    fn eq(&self, other: &Self) -> bool {
        crate::language::PartialEqAny::eq_dyn(&self.value, &other.value)
    }
}

impl ReturnStatement {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_return_statement(&self)
    }
}
