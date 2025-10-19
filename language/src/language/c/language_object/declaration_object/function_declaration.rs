pub mod function_parameter;

use uuid::Uuid;

use crate::language::c::{
    c_type::CType,
    language_object::declaration_object::function_declaration::function_parameter::FunctionParameter,
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, field_inspect_derive::FieldInspect)]
/// ```c
/// int main();
/// ```
pub struct FunctionDeclaration {
    pub id: Uuid,
    pub return_type: CType,
    pub identifier: String,
    pub parameter_list: Vec<FunctionParameter>,
}

impl PartialEq for FunctionDeclaration {
    fn eq(&self, other: &Self) -> bool {
        self.return_type == other.return_type
            && self.identifier == other.identifier
            && self.parameter_list == other.parameter_list
    }
}

impl FunctionDeclaration {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_function_declaration(&self)
    }
}

impl Default for FunctionDeclaration {
    fn default() -> Self {
        FunctionDeclaration {
            id: Uuid::new_v4(),
            return_type: CType::default(),
            identifier: String::new(),
            parameter_list: vec![],
        }
    }
}
