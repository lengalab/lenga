use uuid::Uuid;

use crate::language::c::{
    language_object::LanguageObject as CLanguageObject,
    object_types::c_type::CType,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, PartialEq)]
/// ```c
/// int a; // value == None
/// int a = 5; value == Some(_)
/// ```
pub struct Declaration {
    pub id: Uuid,
    pub primitive_type: CType,
    pub identifier: String,
    pub value: Option<Box<CLanguageObject>>,
}

impl Declaration {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_declaration(&self)
    }
}
