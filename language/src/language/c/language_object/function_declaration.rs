use std::any::Any;

use uuid::Uuid;

use crate::{
    language::{
        LanguageObject,
        c::{
            C,
            language_object::{
                LanguageObject as CLanguageObject, function_parameter::FunctionParameter,
            },
            object_types::c_type::CType,
            writers::{Cursor, Writer, writer_error::WriterError},
        },
    },
    node::Node,
};

#[derive(Debug, Clone)]
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
        // Ok(FunctionDeclaration {
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

impl FunctionDeclaration {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_function_declaration(&self)
    }
}
