use crate::language::c::TreeSitterNodeExt;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum CType {
    Int,
    Float,
    Double,
    Char,
    #[default]
    Void,
    Fn(FnType),
    // TODO add more types as needed
}

#[derive(Debug, PartialEq, Clone)]
pub struct FnType {
    pub return_type: Box<CType>,
    pub parameters: Vec<CType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCTypeError;

impl fmt::Display for ParseCTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid C primitive type")
    }
}

impl FromStr for CType {
    type Err = ParseCTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int" => Ok(CType::Int),
            "float" => Ok(CType::Float),
            "double" => Ok(CType::Double),
            "char" => Ok(CType::Char),
            "void" => Ok(CType::Void),
            _ => Err(ParseCTypeError),
        }
    }
}

impl CType {
    pub fn as_str(&self) -> &str {
        match self {
            CType::Int => "int",
            CType::Float => "float",
            CType::Double => "double",
            CType::Char => "char",
            CType::Void => "void",
            CType::Fn(_) => todo!(),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        s.parse::<CType>().ok()
    }

    pub fn from_tree_sitter_node(node: tree_sitter::Node<'_>, source_code: &str) -> Self {
        assert_eq!(node.kind(), "primitive_type");
        let type_str = node.content(source_code);
        type_str
            .parse::<CType>()
            .expect("invalid primitive_type text in AST")
    }
}
