use crate::language::c::{
    language_object::LanguageObject as CLanguageObject,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub code: Vec<CLanguageObject>,
}

impl SourceFile {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_source_file(&self)
    }
}

impl PartialEq for SourceFile {
    fn eq(&self, other: &Self) -> bool {
        crate::language::PartialEqAny::eq_dyn(&self.code, &other.code)
    }
}
