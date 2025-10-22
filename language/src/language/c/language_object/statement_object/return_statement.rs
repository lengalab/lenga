use uuid::Uuid;

use crate::language::c::{
    language_object::expression_object::ExpressionObject,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, field_inspect_derive::FieldInspect)]
pub struct ReturnStatement {
    pub id: Uuid,
    pub value: Option<ExpressionObject>,
}

impl PartialEq for ReturnStatement {
    fn eq(&self, other: &Self) -> bool {
        crate::language::PartialEqAny::eq_dyn(&self.value, &other.value)
    }
}

impl Default for ReturnStatement {
    fn default() -> Self {
        ReturnStatement {
            id: Uuid::new_v4(),
            value: None,
        }
    }
}

impl ReturnStatement {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_return_statement(&self)
    }
}
