use uuid::Uuid;

use crate::language::c::{
    language_object::LanguageObject as CLanguageObject,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    // TODO how do we differentiate Expressions from Functions?
    pub id: Uuid,
    pub identifier: String,
    pub argument_list: Vec<CLanguageObject>,
}

impl ExpressionStatement {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_expression_statement(&self)
    }
}
