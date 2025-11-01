use language::language::c::{self};
use uuid::Uuid;

use crate::lenga_service::clenga::proto;

pub fn proto_to_c_language_object(
    proto_msg: proto::LanguageObject,
) -> Result<c::language_object::LanguageObject, String> {
    let c_object = match proto_msg.language_object {
        Some(proto::language_object::LanguageObject::SourceFile(src_file)) => {
            let source_file_c_object = source_file_to_c_object(src_file)?;
            c::language_object::LanguageObject::SourceFile(source_file_c_object)
        }
        Some(proto::language_object::LanguageObject::AssignmentExpression(
            assignment_expression,
        )) => {
            let assignment_expression_c_object =
                assignment_expression_to_c_object(assignment_expression)?;
            c::language_object::LanguageObject::AssignmentExpression(assignment_expression_c_object)
        }
        Some(proto::language_object::LanguageObject::BinaryExpression(binary_expression)) => {
            let binary_expression_c_object = binary_expression_to_c_object(binary_expression)?;
            c::language_object::LanguageObject::BinaryExpression(binary_expression_c_object)
        }
        Some(proto::language_object::LanguageObject::CallExpression(call_expression)) => {
            let call_expression_c_object = call_expression_to_c_object(call_expression)?;
            c::language_object::LanguageObject::CallExpression(call_expression_c_object)
        }
        Some(proto::language_object::LanguageObject::Comment(comment)) => {
            let comment_c_object = comment_to_c_object(comment)?;
            c::language_object::LanguageObject::Comment(comment_c_object)
        }
        Some(proto::language_object::LanguageObject::Declaration(declaration)) => {
            let declaration_c_object = declaration_to_c_object(declaration)?;
            c::language_object::LanguageObject::Declaration(declaration_c_object)
        }
        Some(proto::language_object::LanguageObject::ElseClause(else_clause)) => {
            let else_clause_c_object = else_clause_to_c_object(else_clause)?;
            c::language_object::LanguageObject::ElseClause(else_clause_c_object)
        }
        Some(proto::language_object::LanguageObject::FunctionDeclaration(function_declaration)) => {
            let function_declaration_c_object =
                function_declaration_to_c_object(function_declaration)?;
            c::language_object::LanguageObject::FunctionDeclaration(function_declaration_c_object)
        }
        Some(proto::language_object::LanguageObject::FunctionDefinition(function_definition)) => {
            let function_definition_c_object =
                function_definition_to_c_object(function_definition)?;
            c::language_object::LanguageObject::FunctionDefinition(function_definition_c_object)
        }
        Some(proto::language_object::LanguageObject::FunctionParameter(function_parameter)) => {
            let function_parameter_c_object = function_parameter_to_c_object(function_parameter)?;
            c::language_object::LanguageObject::FunctionParameter(function_parameter_c_object)
        }
        Some(proto::language_object::LanguageObject::IfStatement(if_statement)) => {
            let if_statement_c_object = if_statement_to_c_object(if_statement)?;
            c::language_object::LanguageObject::IfStatement(if_statement_c_object)
        }
        Some(proto::language_object::LanguageObject::NumberLiteral(number_literal)) => {
            let number_literal_c_object = number_literal_to_c_object(number_literal)?;
            c::language_object::LanguageObject::NumberLiteral(number_literal_c_object)
        }
        Some(proto::language_object::LanguageObject::PreprocInclude(preproc_include)) => {
            let preproc_include_c_object = preproc_include_to_c_object(preproc_include)?;
            c::language_object::LanguageObject::PreprocInclude(preproc_include_c_object)
        }
        Some(proto::language_object::LanguageObject::Reference(reference)) => {
            let reference_c_object = reference_to_c_object(reference)?;
            c::language_object::LanguageObject::Reference(reference_c_object)
        }
        Some(proto::language_object::LanguageObject::ReturnStatement(return_statement)) => {
            let return_statement_c_object = return_statement_to_c_object(return_statement)?;
            c::language_object::LanguageObject::ReturnStatement(return_statement_c_object)
        }
        Some(proto::language_object::LanguageObject::StringLiteral(string_literal)) => {
            let string_literal_c_object = string_literal_to_c_object(string_literal)?;
            c::language_object::LanguageObject::StringLiteral(string_literal_c_object)
        }
        Some(proto::language_object::LanguageObject::CompoundStatement(compound_statement)) => {
            let compound_statement_c_object = compound_statement_to_c_object(compound_statement)?;
            c::language_object::LanguageObject::CompoundStatement(compound_statement_c_object)
        }
        Some(proto::language_object::LanguageObject::Unknown(unknown)) => {
            let unknown_c_object = unknown_to_c_object(unknown)?;
            c::language_object::LanguageObject::Unknown(unknown_c_object)
        }
        None => return Err("no node was set".to_string()),
    };
    Ok(c_object)
}

pub fn proto_to_c_expression_object(
    proto_msg: proto::ExpressionObject,
) -> Result<c::language_object::expression_object::ExpressionObject, String> {
    let c_object = match proto_msg
        .expression_object
        .ok_or("empty expression object")?
    {
        proto::expression_object::ExpressionObject::AssignmentExpression(assignment_expression) => {
            let assignment_expression_c_object =
                assignment_expression_to_c_object(*assignment_expression)?;
            c::language_object::expression_object::ExpressionObject::AssignmentExpression(
                assignment_expression_c_object,
            )
        }
        proto::expression_object::ExpressionObject::BinaryExpression(binary_expression) => {
            let binary_expression_c_object = binary_expression_to_c_object(*binary_expression)?;
            c::language_object::expression_object::ExpressionObject::BinaryExpression(
                binary_expression_c_object,
            )
        }
        proto::expression_object::ExpressionObject::CallExpression(call_expression) => {
            let call_expression_c_object = call_expression_to_c_object(call_expression)?;
            c::language_object::expression_object::ExpressionObject::CallExpression(
                call_expression_c_object,
            )
        }
        proto::expression_object::ExpressionObject::NumberLiteral(number_literal) => {
            let number_literal_c_object = number_literal_to_c_object(number_literal)?;
            c::language_object::expression_object::ExpressionObject::NumberLiteral(
                number_literal_c_object,
            )
        }
        proto::expression_object::ExpressionObject::Reference(reference) => {
            let reference_c_object = reference_to_c_object(reference)?;
            c::language_object::expression_object::ExpressionObject::Reference(reference_c_object)
        }
        proto::expression_object::ExpressionObject::StringLiteral(string_literal) => {
            let string_literal_c_object = string_literal_to_c_object(string_literal)?;
            c::language_object::expression_object::ExpressionObject::StringLiteral(
                string_literal_c_object,
            )
        }
        proto::expression_object::ExpressionObject::Unknown(unknown) => {
            let unknown_c_object = unknown_to_c_object(unknown)?;
            c::language_object::expression_object::ExpressionObject::Unknown(unknown_c_object)
        }
    };
    Ok(c_object)
}

fn source_file_to_c_object(
    src_file: proto::SourceFile,
) -> Result<c::language_object::special_object::source_file::SourceFile, String> {
    let id =
        Uuid::parse_str(&src_file.id).map_err(|_| "object id could not be parsed".to_string())?;

    let mut code: Vec<c::language_object::declaration_object::DeclarationObject> = Vec::new();
    for msg in src_file.code {
        code.push(proto_to_c_declaration_object(msg)?)
    }

    Ok(c::language_object::special_object::source_file::SourceFile { id, code })
}

fn proto_to_c_declaration_object(
    msg: proto::DeclarationObject,
) -> Result<c::language_object::declaration_object::DeclarationObject, String> {
    Ok(
        match msg.declaration_object.ok_or("empty declaration object")? {
            proto::declaration_object::DeclarationObject::Comment(comment) => {
                let comment_c_object = comment_to_c_object(comment)?;

                c::language_object::declaration_object::DeclarationObject::Comment(comment_c_object)
            }
            proto::declaration_object::DeclarationObject::Declaration(declaration) => {
                let declaration_c_object = declaration_to_c_object(declaration)?;

                c::language_object::declaration_object::DeclarationObject::Declaration(
                    declaration_c_object,
                )
            }
            proto::declaration_object::DeclarationObject::FunctionDeclaration(
                function_declaration,
            ) => {
                let function_declaration_c_object =
                    function_declaration_to_c_object(function_declaration)?;

                c::language_object::declaration_object::DeclarationObject::FunctionDeclaration(
                    function_declaration_c_object,
                )
            }
            proto::declaration_object::DeclarationObject::FunctionDefinition(
                function_definition,
            ) => {
                let function_definition_c_object =
                    function_definition_to_c_object(function_definition)?;

                c::language_object::declaration_object::DeclarationObject::FunctionDefinition(
                    function_definition_c_object,
                )
            }
            proto::declaration_object::DeclarationObject::PreprocInclude(preproc_include) => {
                let preproc_include_c_object = preproc_include_to_c_object(preproc_include)?;

                c::language_object::declaration_object::DeclarationObject::PreprocInclude(
                    preproc_include_c_object,
                )
            }
            proto::declaration_object::DeclarationObject::Unknown(unknown) => {
                let unknown_c_object = unknown_to_c_object(unknown)?;

                c::language_object::declaration_object::DeclarationObject::Unknown(unknown_c_object)
            }
        },
    )
}

fn assignment_expression_to_c_object(
    assignment_expression: proto::AssignmentExpression,
) -> Result<
    c::language_object::expression_object::assignment_expression::AssignmentExpression,
    String,
> {
    let id = Uuid::parse_str(&assignment_expression.id)
        .map_err(|_| "object id could not be parsed".to_string())?;

    let id_declaration = Uuid::parse_str(&assignment_expression.id_declaration)
        .map_err(|_| "assignment expression with unparsable id_declaration attribute")?;

    let proto_value = assignment_expression
        .value
        .ok_or("assignment expression with inexistent value attribute".to_string())?;

    let value_c_object = Box::new(proto_to_c_expression_object(*proto_value)?);

    Ok(
        c::language_object::expression_object::assignment_expression::AssignmentExpression {
            id,
            id_declaration,
            identifier: assignment_expression.identifier,
            value: value_c_object,
        },
    )
}

fn binary_expression_to_c_object(
    binary_expression: proto::BinaryExpression,
) -> Result<c::language_object::expression_object::binary_expression::BinaryExpression, String> {
    let id = Uuid::parse_str(&binary_expression.id)
        .map_err(|_| "object id could not be parsed".to_string())?;

    let proto_left = binary_expression
        .left
        .ok_or("binary expression with inexistent left attribute".to_string())?;
    let left_c_object = Box::new(proto_to_c_expression_object(*proto_left)?);

    let proto_right = binary_expression
        .right
        .ok_or("binary expression with inexistent right attribute".to_string())?;
    let right_c_object = Box::new(proto_to_c_expression_object(*proto_right)?);

    Ok(
        c::language_object::expression_object::binary_expression::BinaryExpression {
            id,
            left: left_c_object,
            operator: binary_expression.operator,
            right: right_c_object,
        },
    )
}

fn call_expression_to_c_object(
    call_expression: proto::CallExpression,
) -> Result<c::language_object::expression_object::call_expression::CallExpression, String> {
    let id = Uuid::parse_str(&call_expression.id)
        .map_err(|_| "object id could not be parsed".to_string())?;

    let id_declaration = Uuid::parse_str(&call_expression.id_declaration)
        .map_err(|_| "call expression with unparsable id_declaration attribute".to_string())?;

    let mut argument_list: Vec<c::language_object::expression_object::ExpressionObject> =
        Vec::new();
    for argument in call_expression.argument_list {
        argument_list.push(proto_to_c_expression_object(argument)?);
    }

    Ok(
        c::language_object::expression_object::call_expression::CallExpression {
            id,
            id_declaration,
            identifier: call_expression.identifier,
            argument_list,
        },
    )
}

fn comment_to_c_object(
    comment: proto::Comment,
) -> Result<c::language_object::special_object::comment::Comment, String> {
    let id =
        Uuid::parse_str(&comment.id).map_err(|_| "object id could not be parsed".to_string())?;

    Ok(c::language_object::special_object::comment::Comment {
        id,
        content: comment.content,
    })
}

fn declaration_to_c_object(
    declaration: proto::Declaration,
) -> Result<c::language_object::declaration_object::declaration::Declaration, String> {
    let id = Uuid::parse_str(&declaration.id)
        .map_err(|_| "object id could not be parsed".to_string())?;

    let primitive_type = c::c_type::CType::from_str(&declaration.primitive_type)
        .ok_or("declaration with unknown primitive_type attribute")?;

    let value = declaration
        .value
        .map(|v| proto_to_c_expression_object(v).map(Box::new))
        .transpose()?;

    Ok(
        c::language_object::declaration_object::declaration::Declaration {
            id,
            primitive_type,
            identifier: declaration.identifier,
            value,
        },
    )
}

fn else_clause_to_c_object(
    else_clause: proto::ElseClause,
) -> Result<c::language_object::statement_object::if_statement::else_clause::ElseClause, String> {
    let id = Uuid::parse_str(&else_clause.id)
        .map_err(|_| "object id could not be parsed".to_string())?;

    let body = else_clause
        .body
        .ok_or("else clause without compound_statement attribute")?;
    let compound_statement = compound_statement_object_to_c_object(*body)?;

    Ok(
        c::language_object::statement_object::if_statement::else_clause::ElseClause {
            id,
            body: Box::new(compound_statement),
        },
    )
}

#[allow(dead_code)]
fn statement_object_to_c_object(
    statement_object: proto::StatementObject,
) -> Result<c::language_object::statement_object::StatementObject, String> {
    match statement_object
        .statement_object
        .ok_or("Empty Statement Object")?
    {
        proto::statement_object::StatementObject::CompoundStatement(compound_statement) => Ok(
            c::language_object::statement_object::StatementObject::CompoundStatement(
                compound_statement_to_c_object(compound_statement)?,
            ),
        ),
        proto::statement_object::StatementObject::IfStatement(if_statement) => Ok(
            c::language_object::statement_object::StatementObject::IfStatement(
                if_statement_to_c_object(if_statement)?,
            ),
        ),
        proto::statement_object::StatementObject::ReturnStatement(return_statement) => Ok(
            c::language_object::statement_object::StatementObject::ReturnStatement(
                return_statement_to_c_object(return_statement)?,
            ),
        ),
        proto::statement_object::StatementObject::Unknown(unknown) => Ok(
            c::language_object::statement_object::StatementObject::Unknown(unknown_to_c_object(
                unknown,
            )?),
        ),
    }
}

fn unknown_to_c_object(
    unknown: proto::Unknown,
) -> Result<c::language_object::special_object::unknown::Unknown, String> {
    let id = Uuid::parse_str(&unknown.id)
        .map_err(|_| "object id could not be parsed".to_string())
        .map_err(|e| e.as_str().to_string())?;

    Ok(c::language_object::special_object::unknown::Unknown {
        id,
        content: unknown.content,
    })
}

fn function_declaration_to_c_object(
    function_declaration: proto::FunctionDeclaration,
) -> Result<c::language_object::declaration_object::function_declaration::FunctionDeclaration, String>
{
    let id = Uuid::parse_str(&function_declaration.id)
        .map_err(|_| "object id could not be parsed".to_string())?;

    let return_type = c::c_type::CType::from_str(&function_declaration.return_type)
        .ok_or("function declaration with unknown return_type attribute")?;

    let mut parameter_list: Vec<
        c::language_object::declaration_object::function_declaration::function_parameter::FunctionParameter,
    > = Vec::new();
    for parameter in function_declaration.parameter_list {
        parameter_list.push(function_parameter_to_c_object(parameter)?);
    }

    Ok(
        c::language_object::declaration_object::function_declaration::FunctionDeclaration {
            id,
            return_type,
            identifier: function_declaration.identifier,
            parameter_list,
        },
    )
}

fn function_definition_to_c_object(
    function_definition: proto::FunctionDefinition,
) -> Result<c::language_object::declaration_object::function_definition::FunctionDefinition, String>
{
    let id = Uuid::parse_str(&function_definition.id)
        .map_err(|_| "object id could not be parsed".to_string())?;

    let return_type = c::c_type::CType::from_str(&function_definition.return_type)
        .ok_or("function definition with unknown return_type attribute")?;

    let mut parameter_list: Vec<
        c::language_object::declaration_object::function_declaration::function_parameter::FunctionParameter,
    > = Vec::new();
    for parameter in function_definition.parameter_list {
        parameter_list.push(function_parameter_to_c_object(parameter)?);
    }

    let compound_statement = compound_statement_to_c_object(
        function_definition
            .compound_statement
            .ok_or("function definition without compound_statement attribute")?,
    )?;

    Ok(
        c::language_object::declaration_object::function_definition::FunctionDefinition {
            id,
            return_type,
            identifier: function_definition.identifier,
            parameter_list,
            compound_statement,
        },
    )
}

fn function_parameter_to_c_object(
    function_parameter: proto::FunctionParameter,
) -> Result<c::language_object::declaration_object::function_declaration::function_parameter::FunctionParameter, String>{
    let id = Uuid::parse_str(&function_parameter.id)
        .map_err(|_| "object id could not be parsed".to_string())?;

    let param_type = c::c_type::CType::from_str(&function_parameter.param_type)
        .ok_or("function parameter with unknown param_type attribute")?;

    Ok(
        c::language_object::declaration_object::function_declaration::function_parameter::FunctionParameter {
            id,
            identifier: function_parameter.identifier,
            param_type,
        },
    )
}

fn if_statement_to_c_object(
    if_statement: proto::IfStatement,
) -> Result<c::language_object::statement_object::if_statement::IfStatement, String> {
    let id = Uuid::parse_str(&if_statement.id)
        .map_err(|_| "object id could not be parsed".to_string())?;

    let condition = expression_object_to_c_language_object(
        if_statement
            .condition
            .ok_or("if statement  without condition attribute")?,
    )?;

    let body = compound_statement_object_to_c_object(
        *if_statement
            .body
            .ok_or("if statement without compound_statement attribute")?,
    )?;

    let else_statement = match if_statement.else_statement {
        Some(proto::if_statement::ElseStatement::ElseClause(else_clause)) => Some(
            c::language_object::statement_object::if_statement::ElseStatement::ElseClause(
                Box::new(else_clause_to_c_object(*else_clause)?),
            ),
        ),
        Some(proto::if_statement::ElseStatement::ElseIf(else_if)) => Some(
            c::language_object::statement_object::if_statement::ElseStatement::ElseIf(Box::new(
                if_statement_to_c_object(*else_if)?,
            )),
        ),
        None => None,
    };

    Ok(
        c::language_object::statement_object::if_statement::IfStatement {
            id,
            condition: Box::new(condition),
            body: Box::new(body),
            else_statement,
        },
    )
}

fn number_literal_to_c_object(
    number_literal: proto::NumberLiteral,
) -> Result<c::language_object::expression_object::number_literal::NumberLiteral, String> {
    let id = Uuid::parse_str(&number_literal.id)
        .map_err(|_| "object id could not be parsed".to_string())?;

    Ok(
        c::language_object::expression_object::number_literal::NumberLiteral {
            id,
            value: number_literal.value,
        },
    )
}

fn preproc_include_to_c_object(
    preproc_include: proto::PreprocInclude,
) -> Result<c::language_object::declaration_object::preproc_include::PreprocInclude, String> {
    let id = Uuid::parse_str(&preproc_include.id)
        .map_err(|_| "object id could not be parsed".to_string())?;

    Ok(
        c::language_object::declaration_object::preproc_include::PreprocInclude {
            id,
            content: preproc_include.content,
        },
    )
}

fn reference_to_c_object(
    reference: proto::Reference,
) -> Result<c::language_object::expression_object::reference::Reference, String> {
    let id =
        Uuid::parse_str(&reference.id).map_err(|_| "object id could not be parsed".to_string())?;

    let declaration_id = Uuid::parse_str(&reference.declaration_id)
        .map_err(|_| "reference with unparsable declaration_id attribute".to_string())?;

    Ok(
        c::language_object::expression_object::reference::Reference {
            id,
            declaration_id,
            identifier: reference.identifier,
        },
    )
}

fn return_statement_to_c_object(
    return_statement: proto::ReturnStatement,
) -> Result<c::language_object::statement_object::return_statement::ReturnStatement, String> {
    let id = Uuid::parse_str(&return_statement.id)
        .map_err(|_| "object id could not be parsed".to_string())?;
    let value = match return_statement.value {
        Some(value) => Some(expression_object_to_c_language_object(value)?),
        None => None,
    };

    Ok(c::language_object::statement_object::return_statement::ReturnStatement { id, value })
}

fn expression_object_to_c_language_object(
    expression_object: proto::ExpressionObject,
) -> Result<c::language_object::expression_object::ExpressionObject, String> {
    match expression_object
        .expression_object
        .ok_or("expression_object is missing")?
    {
        proto::expression_object::ExpressionObject::AssignmentExpression(assignment_expression) => {
            Ok(
                c::language_object::expression_object::ExpressionObject::AssignmentExpression(
                    assignment_expression_to_c_object(*assignment_expression)?,
                ),
            )
        }
        proto::expression_object::ExpressionObject::BinaryExpression(binary_expression) => Ok(
            c::language_object::expression_object::ExpressionObject::BinaryExpression(
                binary_expression_to_c_object(*binary_expression)?,
            ),
        ),
        proto::expression_object::ExpressionObject::CallExpression(call_expression) => Ok(
            c::language_object::expression_object::ExpressionObject::CallExpression(
                call_expression_to_c_object(call_expression)?,
            ),
        ),
        proto::expression_object::ExpressionObject::NumberLiteral(number_literal) => Ok(
            c::language_object::expression_object::ExpressionObject::NumberLiteral(
                number_literal_to_c_object(number_literal)?,
            ),
        ),
        proto::expression_object::ExpressionObject::Reference(reference) => Ok(
            c::language_object::expression_object::ExpressionObject::Reference(
                reference_to_c_object(reference)?,
            ),
        ),
        proto::expression_object::ExpressionObject::StringLiteral(string_literal) => Ok(
            c::language_object::expression_object::ExpressionObject::StringLiteral(
                string_literal_to_c_object(string_literal)?,
            ),
        ),
        proto::expression_object::ExpressionObject::Unknown(unknown) => Ok(
            c::language_object::expression_object::ExpressionObject::Unknown(unknown_to_c_object(
                unknown,
            )?),
        ),
    }
}

fn string_literal_to_c_object(
    string_literal: proto::StringLiteral,
) -> Result<c::language_object::expression_object::string_literal::StringLiteral, String> {
    let id = Uuid::parse_str(&string_literal.id)
        .map_err(|_| "object id could not be parsed".to_string())?;

    Ok(
        c::language_object::expression_object::string_literal::StringLiteral {
            id,
            value: string_literal.value,
        },
    )
}

fn compound_statement_to_c_object(
    compound_statement: proto::CompoundStatement,
) -> Result<c::language_object::statement_object::compound_statement::CompoundStatement, String> {
    let id = Uuid::parse_str(&compound_statement.id)
        .map_err(|_| "object id could not be parsed".to_string())?;

    let mut code_block: Vec<c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject> = Vec::new();
    for statement in compound_statement.code_block {
        code_block.push(compound_statement_object_to_c_object(statement)?);
    }

    Ok(
        c::language_object::statement_object::compound_statement::CompoundStatement {
            id,
            code_block,
        },
    )
}

fn compound_statement_object_to_c_object(statement: proto::CompoundStatementObject) -> Result<c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject, String>{
    match statement
        .compound_statement_object
        .ok_or("empty statement object")? {
            proto::compound_statement_object::CompoundStatementObject::Declaration(declaration) => Ok(
                c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_to_c_object(declaration)?,
                ),
            ),
            proto::compound_statement_object::CompoundStatementObject::AssignmentExpression(assignment_expression) => Ok(
                c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::AssignmentExpression(
                    assignment_expression_to_c_object(assignment_expression)?,
                ),
            ),
            proto::compound_statement_object::CompoundStatementObject::BinaryExpression(binary_expression) => Ok(
                c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::BinaryExpression(
                    binary_expression_to_c_object(binary_expression)?,
                ),
            ),
            proto::compound_statement_object::CompoundStatementObject::CallExpression(call_expression) => Ok(
                c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::CallExpression(
                    call_expression_to_c_object(call_expression)?,
                ),
            ),
            proto::compound_statement_object::CompoundStatementObject::NumberLiteral(number_literal) => Ok(
                c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::NumberLiteral(
                    number_literal_to_c_object(number_literal)?,
                ),
            ),
            proto::compound_statement_object::CompoundStatementObject::Reference(reference) => Ok(
                c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::Reference(
                    reference_to_c_object(reference)?,
                ),
            ),
            proto::compound_statement_object::CompoundStatementObject::StringLiteral(string_literal) => Ok(
                c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::StringLiteral(
                    string_literal_to_c_object(string_literal)?,
                ),
            ),
            proto::compound_statement_object::CompoundStatementObject::CompoundStatement(compound_statement) => Ok(
                c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::CompoundStatement(
                    compound_statement_to_c_object(compound_statement)?,
                ),
            ),
            proto::compound_statement_object::CompoundStatementObject::IfStatement(if_statement) => Ok(
                c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::IfStatement(
                    if_statement_to_c_object(*if_statement)?,
                ),
            ),
            proto::compound_statement_object::CompoundStatementObject::ReturnStatement(return_statement) => Ok(
                c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::ReturnStatement(
                    return_statement_to_c_object(return_statement)?,
                ),
            ),
            proto::compound_statement_object::CompoundStatementObject::Comment(comment) => Ok(
                c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::Comment(
                    comment_to_c_object(comment)?,
                ),
            ),
            proto::compound_statement_object::CompoundStatementObject::Unknown(unknown) => Ok(
                c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::Unknown(
                    unknown_to_c_object(unknown)?,
                ),
            ),
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lenga_service::clenga::proto;
    use language::language::c::{self, language_object::statement_object};
    use uuid::Uuid;

    #[test]
    fn test_01_source_file() {
        let comment_id = Uuid::new_v4();
        let content = "test";
        let comment = proto::DeclarationObject {
            declaration_object: Some(proto::declaration_object::DeclarationObject::Comment(
                proto::Comment {
                    id: comment_id.to_string(),
                    content: content.to_string(),
                },
            )),
        };

        let id = Uuid::new_v4();
        let src = proto::SourceFile {
            id: id.to_string(),
            code: vec![comment],
        };
        let c_src = source_file_to_c_object(src).unwrap();

        assert_eq!(c_src.id, id);
        match &c_src.code[0] {
            c::language_object::declaration_object::DeclarationObject::Comment(c_comment) => {
                assert_eq!(c_comment.id, comment_id);
                assert_eq!(c_comment.content, content);
            }
            _ => panic!("expected Comment"),
        }
    }

    #[test]
    fn test_02_assignment_expression() {
        let number_id = Uuid::new_v4();
        let number_value = "42";
        let number = proto::ExpressionObject {
            expression_object: Some(proto::expression_object::ExpressionObject::NumberLiteral(
                proto::NumberLiteral {
                    id: number_id.to_string(),
                    value: number_value.to_string(),
                },
            )),
        };

        let id = Uuid::new_v4();
        let id_declaration = Uuid::new_v4();
        let identifier = "test";
        let assignment = proto::AssignmentExpression {
            id: id.to_string(),
            id_declaration: id_declaration.to_string(),
            identifier: identifier.to_string(),
            value: Some(Box::new(number)),
        };
        let c_assign = assignment_expression_to_c_object(assignment).unwrap();

        assert_eq!(c_assign.id, id);
        assert_eq!(c_assign.id_declaration, id_declaration);
        assert_eq!(c_assign.identifier, identifier);
        match *c_assign.value {
            c::language_object::expression_object::ExpressionObject::NumberLiteral(ref c_num) => {
                assert_eq!(c_num.id, number_id);
                assert_eq!(c_num.value, number_value);
            }
            _ => panic!("expected NumberLiteral"),
        }
    }

    #[test]
    fn test_03_binary_expression() {
        let left_id = Uuid::new_v4();
        let left_value = "1";
        let left = proto::ExpressionObject {
            expression_object: Some(proto::expression_object::ExpressionObject::NumberLiteral(
                proto::NumberLiteral {
                    id: left_id.to_string(),
                    value: left_value.to_string(),
                },
            )),
        };

        let right_id = Uuid::new_v4();
        let right_value = "2";
        let right = proto::ExpressionObject {
            expression_object: Some(proto::expression_object::ExpressionObject::NumberLiteral(
                proto::NumberLiteral {
                    id: right_id.to_string(),
                    value: right_value.to_string(),
                },
            )),
        };

        let id = Uuid::new_v4();
        let operator = "+";
        let binary = proto::BinaryExpression {
            id: id.to_string(),
            operator: operator.to_string(),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };
        let c_bin = binary_expression_to_c_object(binary).unwrap();

        assert_eq!(c_bin.id, id);
        assert_eq!(c_bin.operator, operator);

        match *c_bin.left {
            c::language_object::expression_object::ExpressionObject::NumberLiteral(ref c_num) => {
                assert_eq!(c_num.id, left_id);
                assert_eq!(c_num.value, left_value);
            }
            _ => panic!("expected NumberLiteral"),
        }

        match *c_bin.right {
            c::language_object::expression_object::ExpressionObject::NumberLiteral(ref c_num) => {
                assert_eq!(c_num.id, right_id);
                assert_eq!(c_num.value, right_value);
            }
            _ => panic!("expected NumberLiteral"),
        }
    }

    #[test]
    fn test_04_call_expression() {
        let arg_id = Uuid::new_v4();
        let declaration_id = Uuid::new_v4();
        let arg_identifier = "bar";
        let arg = proto::ExpressionObject {
            expression_object: Some(proto::expression_object::ExpressionObject::Reference(
                proto::Reference {
                    id: arg_id.to_string(),
                    declaration_id: declaration_id.to_string(),
                    identifier: arg_identifier.to_string(),
                },
            )),
        };

        let id = Uuid::new_v4();
        let id_declaration = Uuid::new_v4();
        let identifier = "foo";
        let call = proto::CallExpression {
            id: id.to_string(),
            id_declaration: id_declaration.to_string(),
            identifier: identifier.to_string(),
            argument_list: vec![arg],
        };
        let c_call = call_expression_to_c_object(call).unwrap();

        assert_eq!(c_call.id, id);
        assert_eq!(c_call.identifier, identifier);
        assert_eq!(c_call.id_declaration, id_declaration);

        match &c_call.argument_list[0] {
            c::language_object::expression_object::ExpressionObject::Reference(c_ref) => {
                assert_eq!(c_ref.id, arg_id);
                assert_eq!(c_ref.declaration_id, declaration_id);
                assert_eq!(c_ref.identifier, arg_identifier);
            }
            _ => panic!("expected NumberLiteral"),
        }
    }

    #[test]
    fn test_05_comment() {
        let id = Uuid::new_v4();
        let content = "test";
        let proto_comment = proto::Comment {
            id: id.to_string(),
            content: content.to_string(),
        };
        let c_comment = comment_to_c_object(proto_comment).unwrap();

        assert_eq!(c_comment.id, id);
        assert_eq!(c_comment.content, content);
    }

    #[test]
    fn test_06_declaration() {
        let number_id = Uuid::new_v4();
        let number_value = "42";
        let number = proto::NumberLiteral {
            id: number_id.to_string(),
            value: number_value.to_string(),
        };

        let id = Uuid::new_v4();
        let primitive_type = c::c_type::CType::Int;
        let identifier = "test";
        let decl = proto::Declaration {
            id: id.to_string(),
            primitive_type: primitive_type.as_str().to_string(),
            identifier: identifier.to_string(),
            value: Some(proto::ExpressionObject {
                expression_object: Some(proto::expression_object::ExpressionObject::NumberLiteral(
                    number,
                )),
            }),
        };
        let c_decl = declaration_to_c_object(decl).unwrap();

        assert_eq!(c_decl.id, id);
        assert_eq!(c_decl.primitive_type, primitive_type);

        if let Some(inner) = &c_decl.value {
            match inner.as_ref() {
                c::language_object::expression_object::ExpressionObject::NumberLiteral(number) => {
                    assert_eq!(number.id, number_id);
                    assert_eq!(number.value, number_value);
                }
                _ => panic!("expected NumberLiteral"),
            }
        } else {
            panic!("expected Some(Box<NumberLiteral>)");
        }
    }

    #[test]
    fn test_07_else_clause() {
        let comment_id = Uuid::new_v4();
        let content = "test";
        let comment = proto::Comment {
            id: comment_id.to_string(),
            content: content.to_string(),
        };

        let stmt_id = Uuid::new_v4();
        let stmt_comment = proto::CompoundStatementObject {
            compound_statement_object: Some(
                proto::compound_statement_object::CompoundStatementObject::Comment(comment),
            ),
        };
        let comp_stmt = proto::CompoundStatementObject {
            compound_statement_object: Some(
                proto::compound_statement_object::CompoundStatementObject::CompoundStatement(
                    proto::CompoundStatement {
                        id: stmt_id.to_string(),
                        code_block: vec![stmt_comment],
                    },
                ),
            ),
        };

        let id = Uuid::new_v4();
        let else_clause = proto::ElseClause {
            id: id.to_string(),
            body: Some(Box::new(comp_stmt)),
        };
        let c_else = else_clause_to_c_object(else_clause).unwrap();

        assert_eq!(c_else.id, id);
        match &*c_else.body {
            statement_object::compound_statement::compound_statement_object::CompoundStatementObject::CompoundStatement(c_else_compound_statement) => {
                assert_eq!(c_else_compound_statement.id, stmt_id);
                match &c_else_compound_statement.code_block[0] {
                    c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::Comment(c_comment) => {
                        assert_eq!(c_comment.id, comment_id);
                        assert_eq!(c_comment.content, content);
                    }
                    _ => panic!("expected Comment"),
                }
            }
            _ => panic!("expected CompoundStatement"),
        }
    }

    #[test]
    fn test_08_function_declaration() {
        let param_id = Uuid::new_v4();
        let param_type = c::c_type::CType::Int;
        let param_identifier = "test";
        let param = proto::FunctionParameter {
            id: param_id.to_string(),
            param_type: param_type.as_str().to_string(),
            identifier: param_identifier.to_string(),
        };

        let id = Uuid::new_v4();
        let return_type = c::c_type::CType::Int;
        let identifier = "foo";
        let decl = proto::FunctionDeclaration {
            id: id.to_string(),
            return_type: return_type.as_str().to_string(),
            identifier: identifier.to_string(),
            parameter_list: vec![param],
        };
        let c_decl = function_declaration_to_c_object(decl).unwrap();

        assert_eq!(c_decl.id, id);
        assert_eq!(c_decl.return_type, return_type);
        assert_eq!(c_decl.identifier, identifier);
        assert_eq!(c_decl.parameter_list[0].id, param_id);
        assert_eq!(c_decl.parameter_list[0].param_type, param_type);
        assert_eq!(c_decl.parameter_list[0].identifier, param_identifier);
    }

    #[test]
    fn test_09_function_definition() {
        let body_id = Uuid::new_v4();
        let comment_id = Uuid::new_v4();
        let content = "test";
        let comment = proto::CompoundStatementObject {
            compound_statement_object: Some(
                proto::compound_statement_object::CompoundStatementObject::Comment(
                    proto::Comment {
                        id: comment_id.to_string(),
                        content: content.to_string(),
                    },
                ),
            ),
        };
        let compound = proto::CompoundStatement {
            id: body_id.to_string(),
            code_block: vec![comment],
        };

        let param_id = Uuid::new_v4();
        let param_type = c::c_type::CType::Int;
        let param_identifier = "test";
        let param = proto::FunctionParameter {
            id: param_id.to_string(),
            param_type: param_type.as_str().to_string(),
            identifier: param_identifier.to_string(),
        };

        let id = Uuid::new_v4();
        let return_type = c::c_type::CType::Int;
        let identifier = "foo";
        let def = proto::FunctionDefinition {
            id: id.to_string(),
            return_type: return_type.as_str().to_string(),
            identifier: identifier.to_string(),
            parameter_list: vec![param],
            compound_statement: Some(compound),
        };
        let c_def = function_definition_to_c_object(def).unwrap();

        assert_eq!(c_def.id, id);
        assert_eq!(c_def.return_type, return_type);
        assert_eq!(c_def.identifier, identifier);
        assert_eq!(c_def.compound_statement.id, body_id);
        match &c_def.compound_statement.code_block[0] {
            c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::Comment(c_comment) => {
                assert_eq!(c_comment.id, comment_id);
                assert_eq!(c_comment.content, content);
            }
            _ => panic!("expected Comment"),
        }
    }

    #[test]
    fn test_10_function_parameter() {
        let id = Uuid::new_v4();
        let param_type = c::c_type::CType::Int;
        let identifier = "bar";
        let param = proto::FunctionParameter {
            id: id.to_string(),
            param_type: param_type.as_str().to_string(),
            identifier: identifier.to_string(),
        };
        let c_param = function_parameter_to_c_object(param).unwrap();

        assert_eq!(c_param.id, id);
        assert_eq!(c_param.param_type, param_type);
        assert_eq!(c_param.identifier, identifier);
    }

    #[test]
    fn test_11_if_statement() {
        let cond_id = Uuid::new_v4();
        let cond_val = "1";
        let condition = proto::ExpressionObject {
            expression_object: Some(proto::expression_object::ExpressionObject::NumberLiteral(
                proto::NumberLiteral {
                    id: cond_id.to_string(),
                    value: cond_val.to_string(),
                },
            )),
        };

        let then_id = Uuid::new_v4();
        let then_comment_id = Uuid::new_v4();
        let then_comment_content = "then branch";
        let then_comment = proto::CompoundStatementObject {
            compound_statement_object: Some(
                proto::compound_statement_object::CompoundStatementObject::Comment(
                    proto::Comment {
                        id: then_comment_id.to_string(),
                        content: then_comment_content.to_string(),
                    },
                ),
            ),
        };
        let then_stmt = proto::CompoundStatementObject {
            compound_statement_object: Some(
                proto::compound_statement_object::CompoundStatementObject::CompoundStatement(
                    proto::CompoundStatement {
                        id: then_id.to_string(),
                        code_block: vec![then_comment],
                    },
                ),
            ),
        };

        let else_id = Uuid::new_v4();
        let else_comment_id = Uuid::new_v4();
        let else_comment_content = "else branch";
        let else_comment = proto::CompoundStatementObject {
            compound_statement_object: Some(
                proto::compound_statement_object::CompoundStatementObject::Comment(
                    proto::Comment {
                        id: else_comment_id.to_string(),
                        content: else_comment_content.to_string(),
                    },
                ),
            ),
        };
        let else_compound = proto::CompoundStatementObject {
            compound_statement_object: Some(
                proto::compound_statement_object::CompoundStatementObject::CompoundStatement(
                    proto::CompoundStatement {
                        id: else_id.to_string(),
                        code_block: vec![else_comment],
                    },
                ),
            ),
        };
        let else_clause = proto::ElseClause {
            id: Uuid::new_v4().to_string(),
            body: Some(Box::new(else_compound)),
        };

        let id = Uuid::new_v4();
        let if_stmt = proto::IfStatement {
            id: id.to_string(),
            condition: Some(condition),
            body: Some(Box::new(then_stmt)),
            else_statement: Some(proto::if_statement::ElseStatement::ElseClause(Box::new(
                else_clause,
            ))),
        };
        let c_if = if_statement_to_c_object(if_stmt).unwrap();

        assert_eq!(c_if.id, id);
        match *c_if.condition {
            c::language_object::expression_object::ExpressionObject::NumberLiteral(ref c_num) => {
                assert_eq!(c_num.id, cond_id);
                assert_eq!(c_num.value, cond_val);
            }
            _ => panic!("expected NumberLiteral"),
        }
        match *c_if.body {
            c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::CompoundStatement(
                ref c_compound,
            ) => {
                assert_eq!(c_compound.id, then_id);
                match &c_compound.code_block[0] {
                    c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::Comment(c_comment) => {
                        assert_eq!(c_comment.id, then_comment_id);
                        assert_eq!(c_comment.content, then_comment_content);
                    }
                    _ => panic!("expected Comment"),
                }
            }
            _ => panic!("expected compound statement"),
        }
        let language::language::c::language_object::statement_object::if_statement::ElseStatement::ElseClause(
            else_statement) = c_if.else_statement.unwrap() else{
                        panic!("expected ElseClause")
            };
        match *else_statement.body {
            statement_object::compound_statement::compound_statement_object::CompoundStatementObject::CompoundStatement(else_clause_compound_statement) => {
                assert_eq!(else_clause_compound_statement.id, else_id);
                match &else_clause_compound_statement.code_block[0] {
                    c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::Comment(c_comment) => {
                        assert_eq!(c_comment.id, else_comment_id);
                        assert_eq!(c_comment.content, else_comment_content);
                    }
                    _ => panic!("expected Comment"),
                }
            }
            _ => panic!("expected CompoundStatement"),
        }
    }

    #[test]
    fn test_12_number_literal() {
        let id = Uuid::new_v4();
        let value = "123";
        let num = proto::NumberLiteral {
            id: id.to_string(),
            value: value.to_string(),
        };
        let c_num = number_literal_to_c_object(num).unwrap();

        assert_eq!(c_num.id, id);
        assert_eq!(c_num.value, value);
    }

    #[test]
    fn test_13_preproc_include() {
        let id = Uuid::new_v4();
        let directive = "stdio.h";
        let incl = proto::PreprocInclude {
            id: id.to_string(),
            content: directive.to_string(),
        };
        let c_incl = preproc_include_to_c_object(incl).unwrap();

        assert_eq!(c_incl.id, id);
        assert_eq!(c_incl.content, directive);
    }

    #[test]
    fn test_14_reference() {
        let id = Uuid::new_v4();
        let decl_id = Uuid::new_v4();
        let identifier = "foo";
        let proto_ref = proto::Reference {
            id: id.to_string(),
            declaration_id: decl_id.to_string(),
            identifier: identifier.to_string(),
        };
        let c_ref = reference_to_c_object(proto_ref).unwrap();

        assert_eq!(c_ref.id, id);
        assert_eq!(c_ref.declaration_id, decl_id);
        assert_eq!(c_ref.identifier, identifier);
    }

    #[test]
    fn test_15_return_statement() {
        let num_id = Uuid::new_v4();
        let val = "7";
        let ret_expr = proto::ExpressionObject {
            expression_object: Some(proto::expression_object::ExpressionObject::NumberLiteral(
                proto::NumberLiteral {
                    id: num_id.to_string(),
                    value: val.to_string(),
                },
            )),
        };

        let id = Uuid::new_v4();
        let proto_ret = proto::ReturnStatement {
            id: id.to_string(),
            value: Some(ret_expr),
        };
        let c_ret = return_statement_to_c_object(proto_ret).unwrap();

        assert_eq!(c_ret.id, id);
        match c_ret.value.unwrap() {
            c::language_object::expression_object::ExpressionObject::NumberLiteral(ref c_num) => {
                assert_eq!(c_num.id, num_id);
                assert_eq!(c_num.value, val);
            }
            _ => panic!("expected NumberLiteral"),
        }
    }

    #[test]
    fn test_16_string_literal() {
        let id = Uuid::new_v4();
        let val = "hello";
        let proto_str = proto::StringLiteral {
            id: id.to_string(),
            value: val.to_string(),
        };
        let c_str = string_literal_to_c_object(proto_str).unwrap();

        assert_eq!(c_str.id, id);
        assert_eq!(c_str.value, val);
    }

    #[test]
    fn test_17_compound_statement() {
        let comment_id = Uuid::new_v4();
        let content = "test";
        let comment = proto::CompoundStatementObject {
            compound_statement_object: Some(
                proto::compound_statement_object::CompoundStatementObject::Comment(
                    proto::Comment {
                        id: comment_id.to_string(),
                        content: content.to_string(),
                    },
                ),
            ),
        };

        let id = Uuid::new_v4();
        let comp = proto::CompoundStatement {
            id: id.to_string(),
            code_block: vec![comment],
        };
        let c_comp = compound_statement_to_c_object(comp).unwrap();

        assert_eq!(c_comp.id, id);
        match &c_comp.code_block[0] {
            c::language_object::statement_object::compound_statement::compound_statement_object::CompoundStatementObject::Comment(
                c_comment,
            ) => {
                assert_eq!(c_comment.id, comment_id);
                assert_eq!(c_comment.content, content);
            }
            _ => panic!("expected Comment"),
        }
    }
}
