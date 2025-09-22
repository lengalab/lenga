use std::{fs::File, io::Read};

use language::language::{Language, c, c::C};
use uuid::Uuid;

use crate::lenga_service::clenga::proto;

pub fn parse_file(path: &str) -> proto::SourceFile {
    let file = File::open(&path).unwrap();

    let content: Vec<u8> = file.bytes().map(|b| b.unwrap()).collect();

    let c = C::new();
    let src_file = c.parse_nodes(content).unwrap();
    source_file_to_proto(src_file)
}

fn source_file_to_proto(
    src_file: c::language_object::source_file::SourceFile,
) -> proto::SourceFile {
    let mut code: Vec<proto::LanguageObject> = Vec::new();
    for o in src_file.code {
        let proto_obj = match o {
            c::language_object::LanguageObject::SourceFile(source_file) => {
                source_file_to_proto(source_file)
            }
            c::language_object::LanguageObject::AssignmentExpression(assignment_expression) => {
                assignment_expression_to_proto(assignment_expression)
            }
            c::language_object::LanguageObject::BinaryExpression(binary_expression) => {
                binary_expression_to_proto(binary_expression)
            }
            c::language_object::LanguageObject::CallExpression(call_expression) => {
                call_expression_to_proto(call_expression)
            }
            c::language_object::LanguageObject::Comment(comment) => comment_to_proto(comment),
            c::language_object::LanguageObject::Declaration(declaration) => {
                declaration_to_proto(declaration)
            }
            c::language_object::LanguageObject::ElseClause(else_clause) => {
                else_clause_to_proto(else_clause)
            }
            c::language_object::LanguageObject::ExpressionStatement(expression_statement) => {
                expression_statement_to_proto(expression_statement)
            }
            c::language_object::LanguageObject::FunctionDeclaration(function_declaration) => {
                function_declaration_to_proto(function_declaration)
            }
            c::language_object::LanguageObject::FunctionDefinition(function_definition) => {
                function_definition_to_proto(function_definition)
            }
            c::language_object::LanguageObject::FunctionParameter(function_parameter) => {
                function_parameter_to_proto(function_parameter)
            }
            c::language_object::LanguageObject::IfStatement(if_statement) => {
                if_statement_to_proto(if_statement)
            }
            c::language_object::LanguageObject::NumberLiteral(number_literal) => {
                number_literal_to_proto(number_literal)
            }
            c::language_object::LanguageObject::PreprocInclude(preproc_include) => {
                preproc_include_to_proto(preproc_include)
            }
            c::language_object::LanguageObject::Reference(reference) => {
                reference_to_proto(reference)
            }
            c::language_object::LanguageObject::ReturnStatement(return_statement) => {
                return_statement_to_proto(return_statement)
            }
            c::language_object::LanguageObject::StringLiteral(string_literal) => {
                string_literal_to_proto(string_literal)
            }
            c::language_object::LanguageObject::CompoundStatement(compound_statement) => {
                compound_statement_to_proto(compound_statement)
            }
        };
        code.push(proto_obj);
    }
    proto::SourceFile {
        id: Uuid::new_v4().to_string(),
        code,
    }
}

fn assignment_expression_to_proto(
    assignment_expression: c::language_object::assignment_expression::AssignmentExpression,
) -> proto::LanguageObject {
    todo!()
}

fn binary_expression_to_proto(
    binary_expression: c::language_object::binary_expression::BinaryExpression,
) -> proto::LanguageObject {
    todo!()
}

fn call_expression_to_proto(
    call_expression: c::language_object::call_expression::CallExpression,
) -> proto::LanguageObject {
    todo!()
}

fn comment_to_proto(comment: c::language_object::comment::Comment) -> proto::LanguageObject {
    todo!()
}

fn declaration_to_proto(
    declaration: c::language_object::declaration::Declaration,
) -> proto::LanguageObject {
    todo!()
}

fn else_clause_to_proto(
    else_clause: c::language_object::else_clause::ElseClause,
) -> proto::LanguageObject {
    todo!()
}

fn expression_statement_to_proto(
    expression_statement: c::language_object::expression_statement::ExpressionStatement,
) -> proto::LanguageObject {
    todo!()
}

fn function_declaration_to_proto(
    function_declaration: c::language_object::function_declaration::FunctionDeclaration,
) -> proto::LanguageObject {
    todo!()
}

fn function_definition_to_proto(
    function_definition: c::language_object::function_definition::FunctionDefinition,
) -> proto::LanguageObject {
    todo!()
}

fn function_parameter_to_proto(
    function_parameter: c::language_object::function_parameter::FunctionParameter,
) -> proto::LanguageObject {
    todo!()
}

fn if_statement_to_proto(
    if_statement: c::language_object::if_statement::IfStatement,
) -> proto::LanguageObject {
    todo!()
}

fn number_literal_to_proto(
    number_literal: c::language_object::number_literal::NumberLiteral,
) -> proto::LanguageObject {
    todo!()
}

fn preproc_include_to_proto(
    preproc_include: c::language_object::preproc_include::PreprocInclude,
) -> proto::LanguageObject {
    todo!()
}

fn reference_to_proto(
    reference: c::language_object::reference::Reference,
) -> proto::LanguageObject {
    todo!()
}

fn return_statement_to_proto(
    return_statement: c::language_object::return_statement::ReturnStatement,
) -> proto::LanguageObject {
    todo!()
}

fn string_literal_to_proto(
    string_literal: c::language_object::string_literal::StringLiteral,
) -> proto::LanguageObject {
    todo!()
}

fn compound_statement_to_proto(
    compound_statement: c::language_object::compound_statement::CompoundStatement,
) -> proto::LanguageObject {
    todo!()
}
