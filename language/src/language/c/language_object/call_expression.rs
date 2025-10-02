use uuid::Uuid;

use crate::language::c::{
    language_object::LanguageObject as CLanguageObject,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone)]
/// ```c
/// int result = first(1);
/// //           \______/
/// ```
pub struct CallExpression {
    pub id: Uuid,
    pub id_declaration: Uuid,
    pub identifier: String,
    pub argument_list: Vec<CLanguageObject>,
}

impl PartialEq for CallExpression {
    fn eq(&self, other: &Self) -> bool {
        self.id_declaration == other.id_declaration
            && self.identifier == other.identifier
            && self.argument_list == other.argument_list
    }
}

impl CallExpression {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_call_expression(&self)
    }
}
