use uuid::Uuid;

use crate::language::c::{
    language_object::expression_object::ExpressionObject,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, field_inspect_derive::FieldInspect)]
/// ```c
/// int result = first(1);
/// //           \______/
/// ```
pub struct CallExpression {
    pub id: Uuid,
    pub id_declaration: Uuid,
    pub identifier: String,
    pub argument_list: Vec<ExpressionObject>,
}

impl Default for CallExpression {
    fn default() -> Self {
        CallExpression {
            id: Uuid::new_v4(),
            id_declaration: Uuid::new_v4(),
            identifier: String::new(),
            argument_list: vec![],
        }
    }
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
