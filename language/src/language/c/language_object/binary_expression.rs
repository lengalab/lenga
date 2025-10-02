use uuid::Uuid;

use crate::language::c::{
    language_object::LanguageObject as CLanguageObject,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub id: Uuid,
    pub left: Box<CLanguageObject>,
    pub operator: String,
    pub right: Box<CLanguageObject>,
}

impl BinaryExpression {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_binary_expression(&self)
    }
}

impl PartialEq for BinaryExpression {
    fn eq(&self, other: &Self) -> bool {
        self.operator == other.operator
            && crate::language::PartialEqAny::eq_dyn(&self.left, &other.left)
            && crate::language::PartialEqAny::eq_dyn(&self.right, &other.right)
    }
}
