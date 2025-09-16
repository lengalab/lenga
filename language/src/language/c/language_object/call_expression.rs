use uuid::Uuid;

use crate::language::c::{
    language_object::LanguageObject as CLanguageObject,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, PartialEq)]
/// ```c
/// int result = first(1);
/// //           \______/
/// ```
pub struct CallExpression {
    pub id: Uuid,
    pub identifier: String,
    pub argument_list: Vec<CLanguageObject>,
}

impl CallExpression {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_call_expression(&self)
    }
}
