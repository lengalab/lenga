use uuid::Uuid;

use crate::language::c::{
    language_object::expression_object::ExpressionObject,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, field_inspect_derive::FieldInspect)]
pub struct BinaryExpression {
    pub id: Uuid,
    pub left: Box<ExpressionObject>,
    pub operator: String,
    pub right: Box<ExpressionObject>,
}

impl BinaryExpression {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_binary_expression(&self)
    }
}

impl Default for BinaryExpression {
    fn default() -> Self {
        BinaryExpression {
            id: Uuid::new_v4(),
            left: Box::new(ExpressionObject::default()),
            operator: String::new(),
            right: Box::new(ExpressionObject::default()),
        }
    }
}

impl PartialEq for BinaryExpression {
    fn eq(&self, other: &Self) -> bool {
        self.operator == other.operator
            && crate::language::PartialEqAny::eq_dyn(&self.left, &other.left)
            && crate::language::PartialEqAny::eq_dyn(&self.right, &other.right)
    }
}
