use language::language::c;
use uuid::Uuid;

use crate::lenga_service::clenga::proto;

fn proto_to_c_language_object(
    proto_msg: proto::LanguageObject,
) -> Result<c::language_object::LanguageObject, String> {
    let c_object = match proto_msg.language_object {
        Some(proto::language_object::LanguageObject::SourceFile(src_file)) => {
            let source_file_c_object = source_file_to_c_object(src_file)?;
            c::language_object::LanguageObject::SourceFile(source_file_c_object)
        }
        Some(proto::language_object::LanguageObject::AssignmentExpression(assignment_expression)) => {
            let assignment_expression_c_object = assignment_expression_to_c_object(*assignment_expression)?;
            c::language_object::LanguageObject::AssignmentExpression(assignment_expression_c_object)
        }
        Some(proto::language_object::LanguageObject::BinaryExpression(binary_expression)) => {
            let binary_expression_c_object = binary_expression_to_c_object(*binary_expression)?;
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
            let declaration_c_object = declaration_to_c_object(*declaration)?;
            c::language_object::LanguageObject::Declaration(declaration_c_object)
        }
        Some(proto::language_object::LanguageObject::ElseClause(else_clause)) => {
            let else_clause_c_object = else_clause_to_c_object(*else_clause)?;
            c::language_object::LanguageObject::ElseClause(else_clause_c_object)
        }
        Some(proto::language_object::LanguageObject::ExpressionStatement(expression_statement)) => {
            let expression_statement_c_object = expression_statement_to_c_object(expression_statement)?;
            c::language_object::LanguageObject::ExpressionStatement(expression_statement_c_object)
        }
        Some(proto::language_object::LanguageObject::FunctionDeclaration(function_declaration)) => {
            let function_declaration_c_object = function_declaration_to_c_object(function_declaration)?;
            c::language_object::LanguageObject::FunctionDeclaration(function_declaration_c_object)
        }
        Some(proto::language_object::LanguageObject::FunctionDefinition(function_definition)) => {
            let function_definition_c_object = function_definition_to_c_object(function_definition)?;
            c::language_object::LanguageObject::FunctionDefinition(function_definition_c_object)
        }
        Some(proto::language_object::LanguageObject::FunctionParameter(function_parameter)) => {
            let function_parameter_c_object = function_parameter_to_c_object(function_parameter)?;
            c::language_object::LanguageObject::FunctionParameter(function_parameter_c_object)
        }
        Some(proto::language_object::LanguageObject::IfStatement(if_statement)) => {
            let if_statement_c_object = if_statement_to_c_object(*if_statement)?;
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
            let return_statement_c_object = return_statement_to_c_object(*return_statement)?;
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
        Some(proto::language_object::LanguageObject::UnknownNode(_)) => {
            return Err("unknown node was set".to_string())
        }
        None => return Err("no node was set".to_string()),
    };
    return Ok(c_object)
}

fn source_file_to_c_object(
    src_file: proto::SourceFile,
) -> Result<c::language_object::source_file::SourceFile, String> {
    let id = Uuid::parse_str(&src_file.id).map_err(|_| "object id could not be parsed".to_string())?;

    let mut code: Vec<c::language_object::LanguageObject> = Vec::new();
    for msg in src_file.code {
        code.push(proto_to_c_language_object(msg)?)
    }

    Ok(c::language_object::source_file::SourceFile {
        id,
        code,
    })
}

fn assignment_expression_to_c_object(
    assignment_expression: proto::AssignmentExpression,
) -> Result<c::language_object::assignment_expression::AssignmentExpression, String> {
    let id = Uuid::parse_str(&assignment_expression.id).map_err(|_| "object id could not be parsed".to_string())?;

    let proto_value = assignment_expression.value.ok_or("assignment expression with inexistent value attribute".to_string())?;

    let value_c_object = Box::new(
        proto_to_c_language_object(*proto_value)?,
    );

    Ok(c::language_object::assignment_expression::AssignmentExpression {
        id,
        identifier: assignment_expression.identifier,
        value: value_c_object,
    })
}

fn binary_expression_to_c_object(
    binary_expression: proto::BinaryExpression,
) -> Result<c::language_object::binary_expression::BinaryExpression, String> {
    let id = Uuid::parse_str(&binary_expression.id).map_err(|_| "object id could not be parsed".to_string())?;

    let proto_left = binary_expression.left.ok_or("binary expression with inexistent left attribute".to_string())?;
    let left_c_object = Box::new(proto_to_c_language_object(*proto_left)?);
    
    let proto_right = binary_expression.right.ok_or("binary expression with inexistent right attribute".to_string())?;
    let right_c_object = Box::new(proto_to_c_language_object(*proto_right)?);

    Ok(c::language_object::binary_expression::BinaryExpression {
        id,
        left: left_c_object,
        operator: binary_expression.operator,
        right: right_c_object,
    })
}

fn call_expression_to_c_object(
    call_expression: proto::CallExpression,
) -> Result<c::language_object::call_expression::CallExpression, String> {
    let id = Uuid::parse_str(&call_expression.id).map_err(|_| "object id could not be parsed".to_string())?;

    let id_declaration = Uuid::parse_str(&call_expression.id_declaration).map_err(|_| "call expression with unparsable id_declaration attribute".to_string())?;

    let mut argument_list: Vec<c::language_object::LanguageObject> = Vec::new();
    for argument in call_expression.argument_list {
        argument_list.push(proto_to_c_language_object(argument)?);
    }

    Ok(c::language_object::call_expression::CallExpression {
        id,
        id_declaration,
        identifier: call_expression.identifier,
        argument_list: argument_list,
    })
}

fn comment_to_c_object(
    comment: proto::Comment
) -> Result<c::language_object::comment::Comment, String> {
    let id = Uuid::parse_str(&comment.id).map_err(|_| "object id could not be parsed".to_string())?;

    Ok(c::language_object::comment::Comment {
        id,
        content: comment.content,
    })
}

fn declaration_to_c_object(
    declaration: proto::Declaration,
) -> Result<c::language_object::declaration::Declaration, String> {
    let id = Uuid::parse_str(&declaration.id).map_err(|_| "object id could not be parsed".to_string())?;
   
    let primitive_type = c::object_types::c_type::CType::from_str(&declaration.primitive_type).ok_or("declaration with unknown primitive_type attribute")?;

    let value = declaration.value.map(|v| proto_to_c_language_object(*v).map(Box::new)).transpose()?;
    
    Ok(c::language_object::declaration::Declaration {
        id,
        primitive_type,
        identifier: declaration.identifier,
        value: value
    })
}

fn else_clause_to_c_object(
    else_clause: proto::ElseClause,
) -> Result<c::language_object::else_clause::ElseClause, String> {
    let id = Uuid::parse_str(&else_clause.id).map_err(|_| "object id could not be parsed".to_string())?;

    let condition = else_clause.condition.map(|c| proto_to_c_language_object(*c).map(Box::new)).transpose()?;

    let compound_statement = compound_statement_to_c_object(else_clause.compound_statement.ok_or("else clause without compound_statement attribute")?)?;

    Ok(c::language_object::else_clause::ElseClause {
        id,
        condition,
        compound_statement,
    }    )
}

fn expression_statement_to_c_object(
    expression_statement: proto::ExpressionStatement,
) -> Result<c::language_object::expression_statement::ExpressionStatement, String> {
    let id = Uuid::parse_str(&expression_statement.id).map_err(|_| "object id could not be parsed".to_string())?;

    let mut argument_list: Vec<c::language_object::LanguageObject> = Vec::new();
    for argument in expression_statement.argument_list {
        argument_list.push(proto_to_c_language_object(argument)?);
    }

    Ok(c::language_object::expression_statement::ExpressionStatement {
        id,
        identifier: expression_statement.identifier,
        argument_list: argument_list,
    })
}

fn function_declaration_to_c_object(
    function_declaration: proto::FunctionDeclaration,
) -> Result<c::language_object::function_declaration::FunctionDeclaration, String> {
    let id = Uuid::parse_str(&function_declaration.id).map_err(|_| "object id could not be parsed".to_string())?;
    
    let return_type = c::object_types::c_type::CType::from_str(&function_declaration.return_type).ok_or("function declaration with unknown return_type attribute")?;

    let mut parameter_list: Vec<c::language_object::function_parameter::FunctionParameter> = Vec::new();
    for parameter in function_declaration.parameter_list {
        parameter_list.push(function_parameter_to_c_object(parameter)?);
    }

    Ok(c::language_object::function_declaration::FunctionDeclaration {
        id,
        return_type,
        identifier: function_declaration.identifier,
        parameter_list,
    })
}

fn function_definition_to_c_object(
    function_definition: proto::FunctionDefinition,
) -> Result<c::language_object::function_definition::FunctionDefinition, String> {
    let id = Uuid::parse_str(&function_definition.id).map_err(|_| "object id could not be parsed".to_string())?;
    
    let return_type = c::object_types::c_type::CType::from_str(&function_definition.return_type).ok_or("function definition with unknown return_type attribute")?;

    let mut parameter_list: Vec<c::language_object::function_parameter::FunctionParameter> = Vec::new();
    for parameter in function_definition.parameter_list {
        parameter_list.push(function_parameter_to_c_object(parameter)?);
    }

    let compound_statement = compound_statement_to_c_object(function_definition.compound_statement.ok_or("function definition without compound_statement attribute")?)?;

    Ok(c::language_object::function_definition::FunctionDefinition {
        id,
        return_type,
        identifier: function_definition.identifier,
        parameter_list,
        compound_statement,
    })
}

fn function_parameter_to_c_object(
    function_parameter: proto::FunctionParameter,
) -> Result<c::language_object::function_parameter::FunctionParameter, String> {
    let id = Uuid::parse_str(&function_parameter.id).map_err(|_| "object id could not be parsed".to_string())?;

    let param_type = c::object_types::c_type::CType::from_str(&function_parameter.param_type).ok_or("function parameter with unknown param_type attribute")?;

    Ok(c::language_object::function_parameter::FunctionParameter {
        id,
        identifier: function_parameter.identifier,
        param_type,
    })
}

fn if_statement_to_c_object(
    if_statement: proto::IfStatement,
) -> Result<c::language_object::if_statement::IfStatement, String> {
    let id = Uuid::parse_str(&if_statement.id).map_err(|_| "object id could not be parsed".to_string())?;

    let condition = proto_to_c_language_object(*if_statement.condition.ok_or("if statement  without condition attribute")?)?;
    
    let compound_statement = compound_statement_to_c_object(if_statement.compound_statement.ok_or("if statement without compound_statement attribute")?)?;

    let else_clause = if_statement.else_clause.map(|e| else_clause_to_c_object(*e)).transpose()?;
    

    Ok(c::language_object::if_statement::IfStatement {
        id,
        condition: Box::new(condition),
        compound_statement,
        else_clause,
    })
}

fn number_literal_to_c_object(
    number_literal: proto::NumberLiteral,
) -> Result<c::language_object::number_literal::NumberLiteral, String> {
    let id = Uuid::parse_str(&number_literal.id).map_err(|_| "object id could not be parsed".to_string())?;

    Ok(c::language_object::number_literal::NumberLiteral {
        id,
        value: number_literal.value,
    })
}

fn preproc_include_to_c_object(
    preproc_include: proto::PreprocInclude,
) -> Result<c::language_object::preproc_include::PreprocInclude, String> {
    let id = Uuid::parse_str(&preproc_include.id).map_err(|_| "object id could not be parsed".to_string())?;

    Ok(c::language_object::preproc_include::PreprocInclude {
        id,
        content: preproc_include.content,
    })
}

fn reference_to_c_object(
    reference: proto::Reference,
) -> Result<c::language_object::reference::Reference, String> {
    let id = Uuid::parse_str(&reference.id).map_err(|_| "object id could not be parsed".to_string())?;

    let declaration_id = Uuid::parse_str(&reference.declaration_id).map_err(|_| "reference with unparsable declaration_id attribute".to_string())?;

    Ok(c::language_object::reference::Reference {
        id,
        declaration_id, 
        identifier: reference.identifier,
    })
}

fn return_statement_to_c_object(
    return_statement: proto::ReturnStatement,
) -> Result<c::language_object::return_statement::ReturnStatement, String> {
    let id = Uuid::parse_str(&return_statement.id).map_err(|_| "object id could not be parsed".to_string())?;

    let value = proto_to_c_language_object(*return_statement.value.ok_or("return statement without value attribute")?)?; //TODO: This should be able to be an Option

    Ok(c::language_object::return_statement::ReturnStatement {
        id,
        value: Box::new(value),
    })
}

fn string_literal_to_c_object(
    string_literal: proto::StringLiteral,
) -> Result<c::language_object::string_literal::StringLiteral, String> {
    let id = Uuid::parse_str(&string_literal.id).map_err(|_| "object id could not be parsed".to_string())?;

    Ok(c::language_object::string_literal::StringLiteral {
        id,
        value: string_literal.value,
    })
}

fn compound_statement_to_c_object(
    compound_statement: proto::CompoundStatement,
) -> Result<c::language_object::compound_statement::CompoundStatement, String> {
    let id = Uuid::parse_str(&compound_statement.id).map_err(|_| "object id could not be parsed".to_string())?;

    let mut code_block: Vec<c::language_object::LanguageObject> = Vec::new();
    for statement in compound_statement.code_block {
        code_block.push(proto_to_c_language_object(statement)?);
    }

    Ok(c::language_object::compound_statement::CompoundStatement {
        id,
        code_block: code_block,
    })
}
