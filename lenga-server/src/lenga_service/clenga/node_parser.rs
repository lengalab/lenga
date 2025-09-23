use std::{fs::File, io::Read};

use language::language::{c::{self, C}, Language};
use uuid::Uuid;

use crate::lenga_service::clenga::proto;

pub fn parse_file(
    path: &str,
) -> proto::SourceFile {
    let file = File::open(&path).unwrap();

    let content: Vec<u8> = file.bytes().map(|b| b.unwrap()).collect();

    let c = C::new();
    let src_file = c.parse_nodes(content).unwrap();
    source_file_to_proto(src_file)
}

fn c_language_object_to_proto(
    c_object: c::language_object::LanguageObject,
) -> proto::LanguageObject {
    match c_object {
        c::language_object::LanguageObject::SourceFile(source_file) => {
            let source_file_msg = source_file_to_proto(source_file);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::SourceFile(source_file_msg))
            }
        }
        c::language_object::LanguageObject::AssignmentExpression(assignment_expression) => {
            let assignment_expression_msg = assignment_expression_to_proto(assignment_expression);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::AssignmentExpression(
                    Box::new(assignment_expression_msg)
                ))
            }
        }
        c::language_object::LanguageObject::BinaryExpression(binary_expression) => {
            let binary_expression_msg = binary_expression_to_proto(binary_expression);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::BinaryExpression(
                    Box::new(binary_expression_msg)
                ))
            }
        }
        c::language_object::LanguageObject::CallExpression(call_expression) => {
            let call_expression_msg = call_expression_to_proto(call_expression);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::CallExpression(
                    call_expression_msg
                ))
            }                
        }
        c::language_object::LanguageObject::Comment(comment) => {
            let comment_msg = comment_to_proto(comment);
            proto::LanguageObject {
                language_object: Some(
                    proto::language_object::LanguageObject::Comment(
                        comment_msg
                ))
            }                        
        }
        c::language_object::LanguageObject::Declaration(declaration) => {
            let declaration_msg = declaration_to_proto(declaration);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::Declaration(
                    Box::new(declaration_msg)
                ))
            }
        }
        c::language_object::LanguageObject::ElseClause(else_clause) => {
            let else_clause_msg = else_clause_to_proto(else_clause);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::ElseClause(
                    Box::new(else_clause_msg)
                ))
            }
        }
        c::language_object::LanguageObject::ExpressionStatement(expression_statement) => {
            let expression_statement_msg = expression_statement_to_proto(expression_statement);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::ExpressionStatement(
                    expression_statement_msg
                ))
            }
        }
        c::language_object::LanguageObject::FunctionDeclaration(function_declaration) => {
            let function_declaration_msg = function_declaration_to_proto(function_declaration);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::FunctionDeclaration(
                    function_declaration_msg
                ))
            }
        }
        c::language_object::LanguageObject::FunctionDefinition(function_definition) => {
            let function_definition_msg = function_definition_to_proto(function_definition);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::FunctionDefinition(
                    function_definition_msg
                ))
            }
        }
        c::language_object::LanguageObject::FunctionParameter(function_parameter) => {
            let function_parameter_msg = function_parameter_to_proto(function_parameter);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::FunctionParameter(
                    function_parameter_msg
                ))
            }
        }
        c::language_object::LanguageObject::IfStatement(if_statement) => {
            let if_statement_msg = if_statement_to_proto(if_statement);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::IfStatement(
                    Box::new(if_statement_msg),
                ))
            }
        }
        c::language_object::LanguageObject::NumberLiteral(number_literal) => {
            let number_literal_msg = number_literal_to_proto(number_literal);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::NumberLiteral(
                    number_literal_msg
                ))
            }
        }
        c::language_object::LanguageObject::PreprocInclude(preproc_include) => {
            let preproc_include_msg = preproc_include_to_proto(preproc_include);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::PreprocInclude(
                    preproc_include_msg
                ))
            }
        }
        c::language_object::LanguageObject::Reference(reference) => {
            let reference_msg = reference_to_proto(reference);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::Reference(
                    reference_msg
                ))
            }
        }
        c::language_object::LanguageObject::ReturnStatement(return_statement) => {
            let return_statement_msg = return_statement_to_proto(return_statement);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::ReturnStatement(
                    Box::new(return_statement_msg)
                ))
            }
        }
        c::language_object::LanguageObject::StringLiteral(string_literal) => {
            let string_literal_msg = string_literal_to_proto(string_literal);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::StringLiteral(
                    string_literal_msg
                ))
            }
        }
        c::language_object::LanguageObject::CompoundStatement(compound_statement) => {
            let compound_statement_msg = compound_statement_to_proto(compound_statement);
            proto::LanguageObject {
                language_object: Some(proto::language_object::LanguageObject::CompoundStatement(
                    compound_statement_msg
                ))
            }
        }
    }
}

fn source_file_to_proto(
    src_file: c::language_object::source_file::SourceFile,
) -> proto::SourceFile {
    let mut code: Vec<proto::LanguageObject> = Vec::new();
    for o in src_file.code {
        code.push(c_language_object_to_proto(o))
    }

    proto::SourceFile {
        id: Uuid::new_v4().to_string(),
        code,
    }
}

fn assignment_expression_to_proto(
    assignment_expression: c::language_object::assignment_expression::AssignmentExpression,
) -> proto::AssignmentExpression {
    let value_proto = Some(Box::new(
        c_language_object_to_proto(*assignment_expression.value),
    ));

    proto::AssignmentExpression {
        id: assignment_expression.id.to_string(),
        identifier: assignment_expression.identifier,
        value: value_proto,
    }
}

fn binary_expression_to_proto(
    binary_expression: c::language_object::binary_expression::BinaryExpression,
) -> proto::BinaryExpression {
    let left_proto = Some(Box::new(
        c_language_object_to_proto(*binary_expression.left)
    ));

    let right_proto = Some(Box::new(
        c_language_object_to_proto(*binary_expression.right)
    ));

    proto::BinaryExpression {
        id: Uuid::new_v4().to_string(),
        left: left_proto,
        operator: binary_expression.operator,
        right: right_proto,
    }
}

fn call_expression_to_proto(
    call_expression: c::language_object::call_expression::CallExpression,
) -> proto::CallExpression {
    let mut argument_list: Vec<proto::LanguageObject> = Vec::new();
    for argument in call_expression.argument_list {
        argument_list.push(c_language_object_to_proto(argument));
    }

    proto::CallExpression {
        id: call_expression.id.to_string(),
        id_declaration: "".to_string(), //TODO: Get the calle id instead of the identifier
        identifier: call_expression.identifier,
        argument_list: argument_list,
    }
}

fn comment_to_proto(
    comment: c::language_object::comment::Comment
) -> proto::Comment {
    proto::Comment {
        id: Uuid::new_v4().to_string(),
        content: comment.content,
    }
}

fn declaration_to_proto(
    declaration: c::language_object::declaration::Declaration,
) -> proto::Declaration {
    let value = declaration.value.map(|value| Box::new(c_language_object_to_proto(*value)));
    
    proto::Declaration {
        id: declaration.id.to_string(),
        primitive_type: declaration.primitive_type.as_str().to_string(),
        identifier: declaration.identifier,
        value: value
    }
}

fn else_clause_to_proto(
    else_clause: c::language_object::else_clause::ElseClause,
) -> proto::ElseClause {
    let condition = else_clause.condition.map(|condition| Box::new(c_language_object_to_proto(*condition)));
    let compound_statement = compound_statement_to_proto(else_clause.compound_statement);

    proto::ElseClause {
        id: Uuid::new_v4().to_string(),
        condition: condition,
        compound_statement: Some(compound_statement),
    }    
}

fn expression_statement_to_proto(
    expression_statement: c::language_object::expression_statement::ExpressionStatement,
) -> proto::ExpressionStatement {
    let mut argument_list: Vec<proto::LanguageObject> = Vec::new();
    for argument in expression_statement.argument_list {
        argument_list.push(c_language_object_to_proto(argument));
    }

    proto::ExpressionStatement {
        id: expression_statement.id.to_string(),
        identifier: expression_statement.identifier,
        argument_list: argument_list,
    }
}

fn function_declaration_to_proto(
    function_declaration: c::language_object::function_declaration::FunctionDeclaration,
) -> proto::FunctionDeclaration {
    let mut parameter_list: Vec<proto::FunctionParameter> = Vec::new();
    for parameter in function_declaration.parameter_list {
        parameter_list.push(function_parameter_to_proto(parameter));
    }

    proto::FunctionDeclaration {
        id: function_declaration.id.to_string(),
        return_type: function_declaration.return_type.as_str().to_string(),
        identifier: function_declaration.identifier,
        parameter_list: parameter_list,
    }
}

fn function_definition_to_proto(
    function_definition: c::language_object::function_definition::FunctionDefinition,
) -> proto::FunctionDefinition {
    let mut parameter_list: Vec<proto::FunctionParameter> = Vec::new();
    for parameter in function_definition.parameter_list {
        parameter_list.push(function_parameter_to_proto(parameter));
    }

    let compound_statement = compound_statement_to_proto(function_definition.compound_statement);

    proto::FunctionDefinition {
        id: function_definition.id.to_string(),
        return_type: function_definition.return_type.as_str().to_string(),
        identifier: function_definition.identifier,
        parameter_list: parameter_list,
        compound_statement: Some(compound_statement),
    }
}

fn function_parameter_to_proto(
    function_parameter: c::language_object::function_parameter::FunctionParameter,
) -> proto::FunctionParameter {
    proto::FunctionParameter {
        id: function_parameter.id.to_string(),
        identifier: function_parameter.identifier,
        param_type: function_parameter.param_type.as_str().to_string(),
    }
}

fn if_statement_to_proto(
    if_statement: c::language_object::if_statement::IfStatement,
) -> proto::IfStatement {
    let condition = c_language_object_to_proto(*if_statement.condition);
    let compound_statement = compound_statement_to_proto(if_statement.compound_statement);
    let else_clause = if_statement.else_clause.map(|else_clause| Box::new(else_clause_to_proto(else_clause)));

    proto::IfStatement {
        id: Uuid::new_v4().to_string(),
        condition: Some(Box::new(condition)),
        compound_statement: Some(compound_statement),
        else_clause: else_clause,
    }
}

fn number_literal_to_proto(
    number_literal: c::language_object::number_literal::NumberLiteral,
) -> proto::NumberLiteral {
    proto::NumberLiteral {
        id: Uuid::new_v4().to_string(),
        value: number_literal.value,
    }
}

fn preproc_include_to_proto(
    preproc_include: c::language_object::preproc_include::PreprocInclude,
) -> proto::PreprocInclude {
    proto::PreprocInclude {
        id: Uuid::new_v4().to_string(),
        content: preproc_include.content,
    }
}

fn reference_to_proto(
    reference: c::language_object::reference::Reference,
) -> proto::Reference {
    proto::Reference {
        id: reference.id.to_string(),
        declaration_id: "".to_string(), //TODO: Get the declaration id instead of the identifier
        identifier: reference.identifier,
    }
}

fn return_statement_to_proto(
    return_statement: c::language_object::return_statement::ReturnStatement,
) -> proto::ReturnStatement {
    let value = c_language_object_to_proto(*return_statement.value);

    proto::ReturnStatement {
        id: Uuid::new_v4().to_string(),
        value: Some(Box::new(value)),
    }
}

fn string_literal_to_proto(
    string_literal: c::language_object::string_literal::StringLiteral,
) -> proto::StringLiteral {
    proto::StringLiteral {
        id: Uuid::new_v4().to_string(),
        value: string_literal.value,
    }
}

fn compound_statement_to_proto(
    compound_statement: c::language_object::compound_statement::CompoundStatement,
) -> proto::CompoundStatement {
    let mut code_block: Vec<proto::LanguageObject> = Vec::new();
    for statement in compound_statement.code_block {
        code_block.push(c_language_object_to_proto(statement));
    }

    proto::CompoundStatement {
        id: Uuid::new_v4().to_string(),
        code_block: code_block,
    }
}
