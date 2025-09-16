use crate::language::c::TreeSitterNodeExt;

#[derive(Debug, PartialEq, Clone)]
pub enum CType {
    Int,
    Float,
    Double,
    Char,
    Void,
    Fn(FnType),
    // TODO add more types as needed
}

#[derive(Debug, PartialEq, Clone)]
struct FnType {
    pub return_type: Box<CType>,
    pub parameters: Vec<CType>,
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

    pub fn from_str(type_str: &str) -> Option<Self> {
        match type_str {
            "int" => Some(CType::Int),
            "float" => Some(CType::Float),
            "double" => Some(CType::Double),
            "char" => Some(CType::Char),
            "void" => Some(CType::Void),
            _ => todo!(),
        }
    }

    pub fn from_tree_sitter_node(node: tree_sitter::Node<'_>, source_code: &str) -> Self {
        assert_eq!(node.kind(), "primitive_type");
        let type_str = node.content(source_code);
        Self::from_str(&type_str).unwrap()
    }
}
