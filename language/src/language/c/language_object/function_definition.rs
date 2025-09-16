use uuid::Uuid;

use crate::{
    language::c::{
        language_object::{
            LanguageObject as CLanguageObject, compound_statement::CompoundStatement,
            function_parameter::FunctionParameter,
        },
        object_types::c_type::CType,
        writers::{Cursor, writer_error::WriterError},
    },
    node::Node,
};

#[derive(Debug, Clone)]
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
    pub fn parse(node: Node) -> Result<CLanguageObject, String> {
        let identifier_vec = node.tags.get("identifier").unwrap();
        assert!(!identifier_vec.len() == 1);
        let identifier = identifier_vec[0].content.clone();
        todo!();
        // let parameter_list = node
        //     .tags
        //     .get("parameters")
        //     .unwrap()
        //     .iter()
        //     .map(|param| FunctionParameter::parse(param).unwrap())
        //     .collect();
        // Ok(FunctionDefinition {
        //     id: node.id,
        //     return_type: CType::from_str(&node.tags.get("return_type").unwrap()[0].content)
        //         .unwrap(),
        //     declaration: FunctionDeclaration {
        //         identifier,
        //         parameter_list,
        //     },
        //     code_block: todo!("Implement code block parsing"),
        // })
    }
}

impl FunctionDefinition {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_function_definition(&self)
    }
}
