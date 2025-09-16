use uuid::Uuid;

use crate::language::c::{
    object_types::c_type::CType,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone)]
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
