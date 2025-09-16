pub mod c_type;

use crate::{
    language::{
        LanguageObject,
        c::{
            C,
            language_object::{
                LanguageObject as CLanguageObject, function_definition::FunctionDefinition,
            },
        },
    },
    node::Node,
};

pub enum ObjectConstructors {
    FunctionDeclaration,
    FunctionSignature,
    FunctionCall,
    VariableDeclaration,
    VariableReference,
    VariableAssignment,
    ReturnStatement,
    CType,
    FunctionParameter,
}
