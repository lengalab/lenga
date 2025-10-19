#[derive(Debug)]
pub enum NodeType {
    SourceFile,
    AssignmentExpression,
    BinaryExpression,
    CallExpression,
    Comment,
    Declaration,
    ElseClause,
    FunctionDeclaration,
    FunctionDefinition,
    FunctionParameter,
    IfStatement,
    NumberLiteral,
    PreprocInclude,
    Reference,
    ReturnStatement,
    StringLiteral,
    TypeDescription,
    Symbol,
    CompoundStatement,
    Unknown,
}

impl NodeType {
    pub fn as_u64(&self) -> u64 {
        match self {
            NodeType::SourceFile => 0,
            NodeType::AssignmentExpression => 1,
            NodeType::BinaryExpression => 2,
            NodeType::CallExpression => 3,
            NodeType::Comment => 4,
            NodeType::Declaration => 5,
            NodeType::ElseClause => 6,
            NodeType::FunctionDeclaration => 8,
            NodeType::FunctionDefinition => 9,
            NodeType::FunctionParameter => 10,
            NodeType::IfStatement => 11,
            NodeType::NumberLiteral => 12,
            NodeType::PreprocInclude => 13,
            NodeType::Reference => 14,
            NodeType::ReturnStatement => 15,
            NodeType::StringLiteral => 16,
            NodeType::TypeDescription => 17,
            NodeType::Symbol => 18,
            NodeType::CompoundStatement => 19,
            NodeType::Unknown => 404,
        }
    }
}

impl From<NodeType> for u64 {
    fn from(value: NodeType) -> Self {
        match value {
            NodeType::SourceFile => 0,
            NodeType::AssignmentExpression => 1,
            NodeType::BinaryExpression => 2,
            NodeType::CallExpression => 3,
            NodeType::Comment => 4,
            NodeType::Declaration => 5,
            NodeType::ElseClause => 6,
            NodeType::FunctionDeclaration => 8,
            NodeType::FunctionDefinition => 9,
            NodeType::FunctionParameter => 10,
            NodeType::IfStatement => 11,
            NodeType::NumberLiteral => 12,
            NodeType::PreprocInclude => 13,
            NodeType::Reference => 14,
            NodeType::ReturnStatement => 15,
            NodeType::StringLiteral => 16,
            NodeType::TypeDescription => 17,
            NodeType::Symbol => 18,
            NodeType::CompoundStatement => 19,
            NodeType::Unknown => 404,
        }
    }
}

impl From<u64> for NodeType {
    fn from(value: u64) -> Self {
        match value {
            0 => NodeType::SourceFile,
            1 => NodeType::AssignmentExpression,
            2 => NodeType::BinaryExpression,
            3 => NodeType::CallExpression,
            4 => NodeType::Comment,
            5 => NodeType::Declaration,
            6 => NodeType::ElseClause,
            8 => NodeType::FunctionDeclaration,
            9 => NodeType::FunctionDefinition,
            10 => NodeType::FunctionParameter,
            11 => NodeType::IfStatement,
            12 => NodeType::NumberLiteral,
            13 => NodeType::PreprocInclude,
            14 => NodeType::Reference,
            15 => NodeType::ReturnStatement,
            16 => NodeType::StringLiteral,
            17 => NodeType::TypeDescription,
            18 => NodeType::Symbol,
            19 => NodeType::CompoundStatement,
            404 => NodeType::Unknown,
            _ => panic!("NodeType not supported"),
        }
    }
}
