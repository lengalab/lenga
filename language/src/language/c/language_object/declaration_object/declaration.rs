use uuid::Uuid;

use crate::language::c::{
    c_type::CType,
    language_object::expression_object::ExpressionObject,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, PartialEq, field_inspect_derive::FieldInspect)]
/// ```c
/// int a; // value == None
/// int a = 5; value == Some(_)
/// ```
pub struct Declaration {
    pub id: Uuid,
    pub primitive_type: CType,
    pub identifier: String,
    pub value: Option<Box<ExpressionObject>>,
}

impl Declaration {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_declaration(self)
    }
}

impl Default for Declaration {
    fn default() -> Self {
        Declaration {
            id: Uuid::new_v4(),
            primitive_type: CType::default(),
            identifier: String::new(),
            value: None,
        }
    }
}
