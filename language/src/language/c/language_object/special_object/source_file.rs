use uuid::Uuid;

use crate::language::c::{
    language_object::declaration_object::DeclarationObject,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, field_inspect_derive::FieldInspect)]
pub struct SourceFile {
    pub id: Uuid,
    pub code: Vec<DeclarationObject>,
}

impl SourceFile {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_source_file(self)
    }
}

impl PartialEq for SourceFile {
    fn eq(&self, other: &Self) -> bool {
        crate::language::PartialEqAny::eq_dyn(&self.code, &other.code)
    }
}
