use uuid::Uuid;

use crate::language::c::{
    c_type::CType,
    language_object::{
        declaration_object::function_declaration::function_parameter::FunctionParameter,
        statement_object::compound_statement::CompoundStatement,
    },
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone, field_inspect_derive::FieldInspect)]
/// ```c
/// int main() {
/// }
/// ```
pub struct FunctionDefinition {
    pub id: Uuid,
    pub return_type: CType,
    pub identifier: String,
    pub parameter_list: Vec<FunctionParameter>,
    pub compound_statement: CompoundStatement,
}

impl PartialEq for FunctionDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.return_type == other.return_type
            && self.identifier == other.identifier
            && self.parameter_list == other.parameter_list
            && self.compound_statement == other.compound_statement
    }
}

impl FunctionDefinition {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_function_definition(self)
    }
}

impl Default for FunctionDefinition {
    fn default() -> Self {
        FunctionDefinition {
            id: Uuid::new_v4(),
            return_type: CType::default(),
            identifier: String::new(),
            parameter_list: vec![],
            compound_statement: CompoundStatement::default(),
        }
    }
}
