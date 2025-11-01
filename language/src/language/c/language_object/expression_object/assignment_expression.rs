use uuid::Uuid;

use crate::language::c::{
    language_object::expression_object::ExpressionObject,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, field_inspect_derive::FieldInspect)]
/// ```c
/// a = 5;
/// ```
pub struct AssignmentExpression {
    pub id: Uuid,
    pub id_declaration: Uuid,
    pub identifier: String,
    pub value: Box<ExpressionObject>,
}

impl AssignmentExpression {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_assignment_expression(self)
    }
}

impl Default for AssignmentExpression {
    fn default() -> Self {
        AssignmentExpression {
            id: Uuid::new_v4(),
            id_declaration: Uuid::new_v4(),
            identifier: String::new(),
            value: Box::new(ExpressionObject::default()),
        }
    }
}

impl PartialEq for AssignmentExpression {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
            && crate::language::PartialEqAny::eq_dyn(&self.value, &other.value)
    }
}
