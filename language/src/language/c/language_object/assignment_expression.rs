use uuid::Uuid;

use crate::language::c::{
    language_object::LanguageObject as CLanguageObject,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone)]
/// ```c
/// a = 5;
/// ```
pub struct AssignmentExpression {
    pub id: Uuid,
    pub identifier: String,
    pub value: Box<CLanguageObject>,
}

impl AssignmentExpression {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_assignment_expression(&self)
    }
}

impl PartialEq for AssignmentExpression {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
            && crate::language::PartialEqAny::eq_dyn(&self.value, &other.value)
    }
}
