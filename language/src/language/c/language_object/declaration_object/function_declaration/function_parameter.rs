use uuid::Uuid;

use crate::language::c::{
    c_type::CType,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, field_inspect_derive::FieldInspect)]
pub struct FunctionParameter {
    pub id: Uuid,
    pub identifier: String,
    pub param_type: CType,
}

impl PartialEq for FunctionParameter {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.param_type == other.param_type
    }
}

impl FunctionParameter {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        todo!()
    }
}

impl Default for FunctionParameter {
    fn default() -> Self {
        FunctionParameter {
            id: Uuid::new_v4(),
            identifier: String::new(),
            param_type: CType::Void,
        }
    }
}
