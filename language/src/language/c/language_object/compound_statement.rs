use uuid::Uuid;

use crate::language::c::{
    language_object::LanguageObject as CLanguageObject,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone)]
pub struct CompoundStatement {
    pub id: Uuid,
    pub code_block: Vec<CLanguageObject>,
}

impl CompoundStatement {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_compound_statement(&self)
    }
}

impl PartialEq for CompoundStatement {
    fn eq(&self, other: &Self) -> bool {
        crate::language::PartialEqAny::eq_dyn(&self.code_block, &other.code_block)
    }
}
