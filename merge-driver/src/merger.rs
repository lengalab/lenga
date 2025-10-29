use language::language::c::language_object::{
    declaration_object, expression_object, special_object,
    statement_object::{self, compound_statement::compound_statement_object},
};

pub struct Merger {}

impl Merger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn merge(
        &self,
        origin: special_object::source_file::SourceFile,
        ours: special_object::source_file::SourceFile,
        theirs: special_object::source_file::SourceFile,
    ) -> Result<special_object::source_file::SourceFile, String> {
        let mut merge = special_object::source_file::SourceFile {
            id: origin.id,
            code: vec![],
        };

        for (i, object) in origin.code.iter().enumerate() {
            merge.code.push(merge_declaration_objects(
                &object,
                &ours.code[i],
                &theirs.code[i],
            )?);
        }

        Ok(merge)
    }
}

fn merge_declaration_objects(
    origin: &declaration_object::DeclarationObject,
    ours: &declaration_object::DeclarationObject,
    theirs: &declaration_object::DeclarationObject,
) -> Result<declaration_object::DeclarationObject, String> {
    let merge = match (origin, ours, theirs) {
        (
            declaration_object::DeclarationObject::Comment(origin_comment),
            declaration_object::DeclarationObject::Comment(ours_comment),
            declaration_object::DeclarationObject::Comment(theirs_comment),
        ) => declaration_object::DeclarationObject::Comment(merge_comment(
            origin_comment,
            ours_comment,
            theirs_comment,
        )?),
        (
            declaration_object::DeclarationObject::Declaration(origin_declaration),
            declaration_object::DeclarationObject::Declaration(ours_declaration),
            declaration_object::DeclarationObject::Declaration(theirs_declaration),
        ) => declaration_object::DeclarationObject::Declaration(merge_declaration(
            origin_declaration,
            ours_declaration,
            theirs_declaration,
        )?),
        (
            declaration_object::DeclarationObject::FunctionDeclaration(origin_function_declaration),
            declaration_object::DeclarationObject::FunctionDeclaration(ours_function_declaration),
            declaration_object::DeclarationObject::FunctionDeclaration(theirs_function_declaration),
        ) => {
            declaration_object::DeclarationObject::FunctionDeclaration(merge_function_declaration(
                origin_function_declaration,
                ours_function_declaration,
                theirs_function_declaration,
            )?)
        }
        (
            declaration_object::DeclarationObject::FunctionDefinition(origin_function_definition),
            declaration_object::DeclarationObject::FunctionDefinition(ours_function_definition),
            declaration_object::DeclarationObject::FunctionDefinition(theirs_function_definition),
        ) => declaration_object::DeclarationObject::FunctionDefinition(merge_function_definition(
            origin_function_definition,
            ours_function_definition,
            theirs_function_definition,
        )?),
        (
            declaration_object::DeclarationObject::PreprocInclude(origin_preproc_include),
            declaration_object::DeclarationObject::PreprocInclude(ours_preproc_include),
            declaration_object::DeclarationObject::PreprocInclude(theirs_preproc_include),
        ) => declaration_object::DeclarationObject::PreprocInclude(merge_preproc_include(
            origin_preproc_include,
            ours_preproc_include,
            theirs_preproc_include,
        )?),
        _ => return Err("Objects are of different type".to_string()),
    };

    Ok(merge)
}

fn expression_object_changes(
    origin: &expression_object::ExpressionObject,
    alt: &expression_object::ExpressionObject,
) -> Result<Option<expression_object::ExpressionObject>, String> {
    let result = match (origin, alt) {
        (
            expression_object::ExpressionObject::AssignmentExpression(origin_assignment),
            expression_object::ExpressionObject::AssignmentExpression(alt_assignment),
        ) => assignment_expression_changes(origin_assignment, alt_assignment)
            .map(expression_object::ExpressionObject::AssignmentExpression),
        (
            expression_object::ExpressionObject::BinaryExpression(origin_binary),
            expression_object::ExpressionObject::BinaryExpression(alt_binary),
        ) => binary_expression_changes(origin_binary, alt_binary)
            .map(expression_object::ExpressionObject::BinaryExpression),
        (
            expression_object::ExpressionObject::CallExpression(origin_call),
            expression_object::ExpressionObject::CallExpression(alt_call),
        ) => call_expression_changes(origin_call, alt_call)
            .map(expression_object::ExpressionObject::CallExpression),
        (
            expression_object::ExpressionObject::NumberLiteral(origin_number_literal),
            expression_object::ExpressionObject::NumberLiteral(alt_number_literal),
        ) => number_literal_changes(origin_number_literal, alt_number_literal)
            .map(expression_object::ExpressionObject::NumberLiteral),
        (
            expression_object::ExpressionObject::Reference(origin_reference),
            expression_object::ExpressionObject::Reference(alt_reference),
        ) => reference_changes(origin_reference, alt_reference)
            .map(expression_object::ExpressionObject::Reference),
        (
            expression_object::ExpressionObject::StringLiteral(origin_string_literal),
            expression_object::ExpressionObject::StringLiteral(alt_string_literal),
        ) => string_literal_changes(origin_string_literal, alt_string_literal)
            .map(expression_object::ExpressionObject::StringLiteral),
        _ => return Err("Objects are of different type".to_string()),
    };

    Ok(result)
}

fn merge_expression_objects(
    origin: &expression_object::ExpressionObject,
    ours: &expression_object::ExpressionObject,
    theirs: &expression_object::ExpressionObject,
) -> Result<expression_object::ExpressionObject, String> {
    let merge = match (origin, ours, theirs) {
        (
            expression_object::ExpressionObject::AssignmentExpression(origin_assignment),
            expression_object::ExpressionObject::AssignmentExpression(ours_assignment),
            expression_object::ExpressionObject::AssignmentExpression(theirs_assignment),
        ) => expression_object::ExpressionObject::AssignmentExpression(
            merge_assignment_expression(origin_assignment, ours_assignment, theirs_assignment)?,
        ),
        (
            expression_object::ExpressionObject::BinaryExpression(origin_binary),
            expression_object::ExpressionObject::BinaryExpression(ours_binary),
            expression_object::ExpressionObject::BinaryExpression(theirs_binary),
        ) => expression_object::ExpressionObject::BinaryExpression(merge_binary_expression(
            origin_binary,
            ours_binary,
            theirs_binary,
        )?),
        (
            expression_object::ExpressionObject::CallExpression(origin_call),
            expression_object::ExpressionObject::CallExpression(ours_call),
            expression_object::ExpressionObject::CallExpression(theirs_call),
        ) => expression_object::ExpressionObject::CallExpression(merge_call_expression(
            origin_call,
            ours_call,
            theirs_call,
        )?),
        (
            expression_object::ExpressionObject::NumberLiteral(origin_number_literal),
            expression_object::ExpressionObject::NumberLiteral(ours_number_literal),
            expression_object::ExpressionObject::NumberLiteral(theirs_number_literal),
        ) => expression_object::ExpressionObject::NumberLiteral(merge_number_literal(
            origin_number_literal,
            ours_number_literal,
            theirs_number_literal,
        )?),
        (
            expression_object::ExpressionObject::Reference(origin_reference),
            expression_object::ExpressionObject::Reference(ours_reference),
            expression_object::ExpressionObject::Reference(theirs_reference),
        ) => expression_object::ExpressionObject::Reference(merge_reference(
            origin_reference,
            ours_reference,
            theirs_reference,
        )?),
        (
            expression_object::ExpressionObject::StringLiteral(origin_string_literal),
            expression_object::ExpressionObject::StringLiteral(ours_string_literal),
            expression_object::ExpressionObject::StringLiteral(theirs_string_literal),
        ) => expression_object::ExpressionObject::StringLiteral(merge_string_literal(
            origin_string_literal,
            ours_string_literal,
            theirs_string_literal,
        )?),
        _ => return Err("Objects are of different type".to_string()),
    };

    Ok(merge)
}

fn merge_statement_objects(
    origin: &statement_object::StatementObject,
    ours: &statement_object::StatementObject,
    theirs: &statement_object::StatementObject,
) -> Result<statement_object::StatementObject, String> {
    let merge = match (origin, ours, theirs) {
        (
            statement_object::StatementObject::CompoundStatement(origin_compound_statement),
            statement_object::StatementObject::CompoundStatement(ours_compound_statement),
            statement_object::StatementObject::CompoundStatement(theirs_compound_statement),
        ) => statement_object::StatementObject::CompoundStatement(merge_compound_statement(
            origin_compound_statement,
            ours_compound_statement,
            theirs_compound_statement,
        )?),
        (
            statement_object::StatementObject::IfStatement(origin_if_statement),
            statement_object::StatementObject::IfStatement(ours_if_statement),
            statement_object::StatementObject::IfStatement(theirs_if_statement),
        ) => statement_object::StatementObject::IfStatement(merge_if_statement(
            origin_if_statement,
            ours_if_statement,
            theirs_if_statement,
        )?),
        (
            statement_object::StatementObject::ReturnStatement(origin_return_statement),
            statement_object::StatementObject::ReturnStatement(ours_return_statement),
            statement_object::StatementObject::ReturnStatement(theirs_return_statement),
        ) => statement_object::StatementObject::ReturnStatement(merge_return_statement(
            origin_return_statement,
            ours_return_statement,
            theirs_return_statement,
        )?),
        _ => return Err("Objects are of different type".to_string()),
    };

    Ok(merge)
}

fn merge_compound_statement_object(
    origin: &compound_statement_object::CompoundStatementObject,
    ours: &compound_statement_object::CompoundStatementObject,
    theirs: &compound_statement_object::CompoundStatementObject,
) -> Result<compound_statement_object::CompoundStatementObject, String> {
    let merge = match (origin, ours, theirs) {
        (
            compound_statement_object::CompoundStatementObject::Declaration(origin_declaration),
            compound_statement_object::CompoundStatementObject::Declaration(ours_declaration),
            compound_statement_object::CompoundStatementObject::Declaration(theirs_declaration),
        ) => compound_statement_object::CompoundStatementObject::Declaration(merge_declaration(
            origin_declaration,
            ours_declaration,
            theirs_declaration,
        )?),
        (
            compound_statement_object::CompoundStatementObject::AssignmentExpression(origin_assign),
            compound_statement_object::CompoundStatementObject::AssignmentExpression(ours_assign),
            compound_statement_object::CompoundStatementObject::AssignmentExpression(theirs_assign),
        ) => compound_statement_object::CompoundStatementObject::AssignmentExpression(
            merge_assignment_expression(origin_assign, ours_assign, theirs_assign)?,
        ),
        (
            compound_statement_object::CompoundStatementObject::BinaryExpression(origin_binary),
            compound_statement_object::CompoundStatementObject::BinaryExpression(ours_binary),
            compound_statement_object::CompoundStatementObject::BinaryExpression(theirs_binary),
        ) => compound_statement_object::CompoundStatementObject::BinaryExpression(
            merge_binary_expression(origin_binary, ours_binary, theirs_binary)?,
        ),
        (
            compound_statement_object::CompoundStatementObject::CallExpression(origin_call),
            compound_statement_object::CompoundStatementObject::CallExpression(ours_call),
            compound_statement_object::CompoundStatementObject::CallExpression(theirs_call),
        ) => compound_statement_object::CompoundStatementObject::CallExpression(
            merge_call_expression(origin_call, ours_call, theirs_call)?,
        ),
        (
            compound_statement_object::CompoundStatementObject::NumberLiteral(origin_number),
            compound_statement_object::CompoundStatementObject::NumberLiteral(ours_number),
            compound_statement_object::CompoundStatementObject::NumberLiteral(theirs_number),
        ) => compound_statement_object::CompoundStatementObject::NumberLiteral(
            merge_number_literal(origin_number, ours_number, theirs_number)?,
        ),
        (
            compound_statement_object::CompoundStatementObject::Reference(origin_reference),
            compound_statement_object::CompoundStatementObject::Reference(ours_reference),
            compound_statement_object::CompoundStatementObject::Reference(theirs_reference),
        ) => compound_statement_object::CompoundStatementObject::Reference(merge_reference(
            origin_reference,
            ours_reference,
            theirs_reference,
        )?),
        (
            compound_statement_object::CompoundStatementObject::StringLiteral(origin_string),
            compound_statement_object::CompoundStatementObject::StringLiteral(ours_string),
            compound_statement_object::CompoundStatementObject::StringLiteral(theirs_string),
        ) => compound_statement_object::CompoundStatementObject::StringLiteral(
            merge_string_literal(origin_string, ours_string, theirs_string)?,
        ),
        (
            compound_statement_object::CompoundStatementObject::CompoundStatement(origin_comp_stmt),
            compound_statement_object::CompoundStatementObject::CompoundStatement(ours_comp_stmt),
            compound_statement_object::CompoundStatementObject::CompoundStatement(theirs_comp_stmt),
        ) => compound_statement_object::CompoundStatementObject::CompoundStatement(
            merge_compound_statement(origin_comp_stmt, ours_comp_stmt, theirs_comp_stmt)?,
        ),
        (
            compound_statement_object::CompoundStatementObject::IfStatement(origin_if_statement),
            compound_statement_object::CompoundStatementObject::IfStatement(ours_if_statement),
            compound_statement_object::CompoundStatementObject::IfStatement(theirs_if_statement),
        ) => compound_statement_object::CompoundStatementObject::IfStatement(merge_if_statement(
            origin_if_statement,
            ours_if_statement,
            theirs_if_statement,
        )?),
        (
            compound_statement_object::CompoundStatementObject::ReturnStatement(origin_return),
            compound_statement_object::CompoundStatementObject::ReturnStatement(ours_return),
            compound_statement_object::CompoundStatementObject::ReturnStatement(theirs_return),
        ) => compound_statement_object::CompoundStatementObject::ReturnStatement(
            merge_return_statement(origin_return, ours_return, theirs_return)?,
        ),
        (
            compound_statement_object::CompoundStatementObject::Comment(origin_comment),
            compound_statement_object::CompoundStatementObject::Comment(ours_comment),
            compound_statement_object::CompoundStatementObject::Comment(theirs_comment),
        ) => compound_statement_object::CompoundStatementObject::Comment(merge_comment(
            origin_comment,
            ours_comment,
            theirs_comment,
        )?),
        _ => return Err("Objects are of different type".to_string()),
    };

    Ok(merge)
}

fn comment_changes(
    origin: &special_object::comment::Comment,
    alt: &special_object::comment::Comment,
) -> Option<special_object::comment::Comment> {
    if alt.content != origin.content {
        Some(alt.clone())
    } else {
        None
    }
}

fn merge_comment(
    origin: &special_object::comment::Comment,
    ours: &special_object::comment::Comment,
    theirs: &special_object::comment::Comment,
) -> Result<special_object::comment::Comment, String> {
    let m_comment = match (
        comment_changes(origin, ours),
        comment_changes(origin, theirs),
    ) {
        (None, None) => origin.clone(),
        (None, Some(m_comment)) => m_comment,
        (Some(m_comment), None) => m_comment,
        _ => return Err(format!("merge conflict in object {}", origin.id)),
    };

    Ok(m_comment)
}

fn merge_value(
    origin: Option<&expression_object::ExpressionObject>,
    ours: Option<&expression_object::ExpressionObject>,
    theirs: Option<&expression_object::ExpressionObject>,
) -> Result<Option<expression_object::ExpressionObject>, String> {
    match (origin, ours, theirs) {
        (Some(origin_expression), Some(ours_expression), Some(theirs_expression)) => {
            let value =
                merge_expression_objects(origin_expression, ours_expression, theirs_expression)?;
            Ok(Some(value))
        }
        (None, None, None) => Ok(None),
        (Some(origin_expression), None, Some(theirs_expression)) => {
            if expression_object_changes(origin_expression, theirs_expression)?.is_none() {
                Ok(None)
            } else {
                Err("merge conflict in declaration value".to_string())
            }
        }
        (Some(origin_expression), Some(ours_expression), None) => {
            if expression_object_changes(origin_expression, ours_expression)?.is_none() {
                Ok(None)
            } else {
                Err("merge conflict in declaration value".to_string())
            }
        }
        (None, Some(ours_expression), None) => Ok(Some(ours_expression.clone())),
        (None, None, Some(theirs_expression)) => Ok(Some(theirs_expression.clone())),
        _ => Err("merge conflict in declaration value".to_string()),
    }
}

fn declaration_changes(
    origin: &declaration_object::declaration::Declaration,
    alt: &declaration_object::declaration::Declaration,
) -> Option<declaration_object::declaration::Declaration> {
    if alt.primitive_type != origin.primitive_type
        || alt.identifier != origin.identifier
        || alt.id != origin.id
    //Because references are dependant of id, the check must be stricter
    {
        Some(alt.clone())
    } else {
        None
    }
}

fn merge_declaration(
    origin: &declaration_object::declaration::Declaration,
    ours: &declaration_object::declaration::Declaration,
    theirs: &declaration_object::declaration::Declaration,
) -> Result<declaration_object::declaration::Declaration, String> {
    let m_value = merge_value(
        origin.value.as_deref(),
        ours.value.as_deref(),
        theirs.value.as_deref(),
    )?;

    let mut m_declaration = match (
        declaration_changes(origin, ours),
        declaration_changes(origin, theirs),
    ) {
        (None, None) => origin.clone(),
        (None, Some(m_declaration)) => m_declaration,
        (Some(m_declaration), None) => m_declaration,
        _ => return Err(format!("merge conflict in object {}", origin.id)),
    };

    m_declaration.value = m_value.map(Box::new);

    Ok(m_declaration)
}

fn function_parameter_changes(
    origin: &declaration_object::function_declaration::function_parameter::FunctionParameter,
    alt: &declaration_object::function_declaration::function_parameter::FunctionParameter,
) -> Option<declaration_object::function_declaration::function_parameter::FunctionParameter> {
    if alt.param_type != origin.param_type || alt.identifier != origin.identifier {
        Some(alt.clone())
    } else {
        None
    }
}

fn merge_function_parameter(
    origin: &declaration_object::function_declaration::function_parameter::FunctionParameter,
    ours: &declaration_object::function_declaration::function_parameter::FunctionParameter,
    theirs: &declaration_object::function_declaration::function_parameter::FunctionParameter,
) -> Result<declaration_object::function_declaration::function_parameter::FunctionParameter, String>
{
    match (
        function_parameter_changes(origin, ours),
        function_parameter_changes(origin, theirs),
    ) {
        (None, None) => Ok(origin.clone()),
        (None, Some(m_function_parameter)) => Ok(m_function_parameter),
        (Some(m_function_parameter), None) => Ok(m_function_parameter),
        _ => Err(format!("merge conflict in object {}", origin.id)),
    }
}

fn function_declaration_changes(
    origin: &declaration_object::function_declaration::FunctionDeclaration,
    alt: &declaration_object::function_declaration::FunctionDeclaration,
) -> Option<declaration_object::function_declaration::FunctionDeclaration> {
    if alt.return_type != origin.return_type
        || alt.identifier != origin.identifier
        || alt.id != origin.id
    //Because references are dependant of id, the check must be stricter
    {
        Some(alt.clone())
    } else {
        None
    }
}

fn merge_function_declaration(
    origin: &declaration_object::function_declaration::FunctionDeclaration,
    ours: &declaration_object::function_declaration::FunctionDeclaration,
    theirs: &declaration_object::function_declaration::FunctionDeclaration,
) -> Result<declaration_object::function_declaration::FunctionDeclaration, String> {
    let mut m_parameter_list = vec![];
    for (i, param) in origin.parameter_list.iter().enumerate() {
        m_parameter_list.push(merge_function_parameter(
            &param,
            &ours.parameter_list[i],
            &theirs.parameter_list[i],
        )?);
    }

    let mut m_function_declaration = match (
        function_declaration_changes(origin, ours),
        function_declaration_changes(origin, theirs),
    ) {
        (None, None) => origin.clone(),
        (None, Some(m_function_declaration)) => m_function_declaration,
        (Some(m_function_declaration), None) => m_function_declaration,
        _ => return Err(format!("merge conflict in object {}", origin.id)),
    };

    m_function_declaration.parameter_list = m_parameter_list;

    Ok(m_function_declaration)
}

fn function_definition_changes(
    origin: &declaration_object::function_definition::FunctionDefinition,
    alt: &declaration_object::function_definition::FunctionDefinition,
) -> Option<declaration_object::function_definition::FunctionDefinition> {
    if alt.return_type != origin.return_type
        || alt.identifier != origin.identifier
        || alt.id != origin.id
    //Because references are dependant of id, the check must be stricter
    {
        Some(alt.clone())
    } else {
        None
    }
}

fn merge_function_definition(
    origin: &declaration_object::function_definition::FunctionDefinition,
    ours: &declaration_object::function_definition::FunctionDefinition,
    theirs: &declaration_object::function_definition::FunctionDefinition,
) -> Result<declaration_object::function_definition::FunctionDefinition, String> {
    let mut m_parameter_list = vec![];
    for (i, param) in origin.parameter_list.iter().enumerate() {
        m_parameter_list.push(merge_function_parameter(
            &param,
            &ours.parameter_list[i],
            &theirs.parameter_list[i],
        )?);
    }

    let m_compound_statement = merge_compound_statement(
        &origin.compound_statement,
        &ours.compound_statement,
        &theirs.compound_statement,
    )?;

    let mut m_function_definition = match (
        function_definition_changes(origin, ours),
        function_definition_changes(origin, theirs),
    ) {
        (None, None) => origin.clone(),
        (None, Some(m_function_definition)) => m_function_definition,
        (Some(m_function_definition), None) => m_function_definition,
        _ => return Err(format!("merge conflict in object {}", origin.id)),
    };

    m_function_definition.parameter_list = m_parameter_list;
    m_function_definition.compound_statement = m_compound_statement;

    Ok(m_function_definition)
}

fn preproc_include_changes(
    origin: &declaration_object::preproc_include::PreprocInclude,
    alt: &declaration_object::preproc_include::PreprocInclude,
) -> Option<declaration_object::preproc_include::PreprocInclude> {
    if alt.content != origin.content {
        return Some(alt.clone());
    } else {
        None
    }
}

fn merge_preproc_include(
    origin: &declaration_object::preproc_include::PreprocInclude,
    ours: &declaration_object::preproc_include::PreprocInclude,
    theirs: &declaration_object::preproc_include::PreprocInclude,
) -> Result<declaration_object::preproc_include::PreprocInclude, String> {
    let m_preproc_include = match (
        preproc_include_changes(origin, ours),
        preproc_include_changes(origin, theirs),
    ) {
        (None, None) => origin.clone(),
        (None, Some(m_preproc_include)) => m_preproc_include,
        (Some(m_preproc_include), None) => m_preproc_include,
        _ => return Err(format!("merge conflict in object {}", origin.id)),
    };

    Ok(m_preproc_include)
}

fn assignment_expression_changes(
    origin: &expression_object::assignment_expression::AssignmentExpression,
    alt: &expression_object::assignment_expression::AssignmentExpression,
) -> Option<expression_object::assignment_expression::AssignmentExpression> {
    if alt.id_declaration != origin.id_declaration || alt.identifier != origin.identifier {
        return Some(alt.clone());
    } else {
        None
    }
}

fn merge_assignment_expression(
    origin: &expression_object::assignment_expression::AssignmentExpression,
    ours: &expression_object::assignment_expression::AssignmentExpression,
    theirs: &expression_object::assignment_expression::AssignmentExpression,
) -> Result<expression_object::assignment_expression::AssignmentExpression, String> {
    let m_value = merge_expression_objects(&origin.value, &ours.value, &theirs.value)?;
    let mut m_assignment_expression = match (
        assignment_expression_changes(origin, ours),
        assignment_expression_changes(origin, theirs),
    ) {
        (None, None) => origin.clone(),
        (None, Some(m_assignment_expression)) => m_assignment_expression,
        (Some(m_assignment_expression), None) => m_assignment_expression,
        _ => return Err(format!("merge conflict in object {}", origin.id)),
    };

    m_assignment_expression.value = Box::new(m_value);
    Ok(m_assignment_expression)
}

fn binary_expression_changes(
    origin: &expression_object::binary_expression::BinaryExpression,
    alt: &expression_object::binary_expression::BinaryExpression,
) -> Option<expression_object::binary_expression::BinaryExpression> {
    if alt.operator != origin.operator {
        return Some(alt.clone());
    } else {
        None
    }
}

fn merge_binary_expression(
    origin: &expression_object::binary_expression::BinaryExpression,
    ours: &expression_object::binary_expression::BinaryExpression,
    theirs: &expression_object::binary_expression::BinaryExpression,
) -> Result<expression_object::binary_expression::BinaryExpression, String> {
    let m_left = merge_expression_objects(&origin.left, &ours.left, &theirs.left)?;
    let m_right = merge_expression_objects(&origin.right, &ours.right, &theirs.right)?;
    let mut m_binary_expression = match (
        binary_expression_changes(origin, ours),
        binary_expression_changes(origin, theirs),
    ) {
        (None, None) => origin.clone(),
        (None, Some(m_binary_expression)) => m_binary_expression,
        (Some(m_binary_expression), None) => m_binary_expression,
        _ => return Err(format!("merge conflict in object {}", origin.id)),
    };

    m_binary_expression.left = Box::new(m_left);
    m_binary_expression.right = Box::new(m_right);
    Ok(m_binary_expression)
}

fn call_expression_changes(
    origin: &expression_object::call_expression::CallExpression,
    alt: &expression_object::call_expression::CallExpression,
) -> Option<expression_object::call_expression::CallExpression> {
    if alt.id_declaration != origin.id_declaration || alt.identifier != origin.identifier {
        return Some(alt.clone());
    } else {
        None
    }
}

fn merge_call_expression(
    origin: &expression_object::call_expression::CallExpression,
    ours: &expression_object::call_expression::CallExpression,
    theirs: &expression_object::call_expression::CallExpression,
) -> Result<expression_object::call_expression::CallExpression, String> {
    let mut m_argument_list = vec![];
    for (i, arg) in origin.argument_list.iter().enumerate() {
        m_argument_list.push(merge_expression_objects(
            &arg,
            &ours.argument_list[i],
            &theirs.argument_list[i],
        )?);
    }

    let mut m_call_expression = match (
        call_expression_changes(origin, ours),
        call_expression_changes(origin, theirs),
    ) {
        (None, None) => origin.clone(),
        (None, Some(m_call_expression)) => m_call_expression,
        (Some(m_call_expression), None) => m_call_expression,
        _ => return Err(format!("merge conflict in object {}", origin.id)),
    };

    m_call_expression.argument_list = m_argument_list;
    Ok(m_call_expression)
}

fn number_literal_changes(
    origin: &expression_object::number_literal::NumberLiteral,
    alt: &expression_object::number_literal::NumberLiteral,
) -> Option<expression_object::number_literal::NumberLiteral> {
    if alt.value != origin.value {
        return Some(alt.clone());
    } else {
        None
    }
}

fn merge_number_literal(
    origin: &expression_object::number_literal::NumberLiteral,
    ours: &expression_object::number_literal::NumberLiteral,
    theirs: &expression_object::number_literal::NumberLiteral,
) -> Result<expression_object::number_literal::NumberLiteral, String> {
    let m_number_literal = match (
        number_literal_changes(origin, ours),
        number_literal_changes(origin, theirs),
    ) {
        (None, None) => origin.clone(),
        (None, Some(m_number_literal)) => m_number_literal,
        (Some(m_number_literal), None) => m_number_literal,
        _ => return Err(format!("merge conflict in object {}", origin.id)),
    };

    Ok(m_number_literal)
}

fn reference_changes(
    origin: &expression_object::reference::Reference,
    alt: &expression_object::reference::Reference,
) -> Option<expression_object::reference::Reference> {
    if alt.declaration_id != origin.declaration_id || alt.identifier != origin.identifier {
        return Some(alt.clone());
    } else {
        None
    }
}

fn merge_reference(
    origin: &expression_object::reference::Reference,
    ours: &expression_object::reference::Reference,
    theirs: &expression_object::reference::Reference,
) -> Result<expression_object::reference::Reference, String> {
    let m_reference = match (
        reference_changes(origin, ours),
        reference_changes(origin, theirs),
    ) {
        (None, None) => origin.clone(),
        (None, Some(m_reference)) => m_reference,
        (Some(m_reference), None) => m_reference,
        _ => return Err(format!("merge conflict in object {}", origin.id)),
    };

    Ok(m_reference)
}

fn string_literal_changes(
    origin: &expression_object::string_literal::StringLiteral,
    alt: &expression_object::string_literal::StringLiteral,
) -> Option<expression_object::string_literal::StringLiteral> {
    if alt.value != origin.value {
        return Some(alt.clone());
    } else {
        None
    }
}

fn merge_string_literal(
    origin: &expression_object::string_literal::StringLiteral,
    ours: &expression_object::string_literal::StringLiteral,
    theirs: &expression_object::string_literal::StringLiteral,
) -> Result<expression_object::string_literal::StringLiteral, String> {
    let m_string_literal = match (
        string_literal_changes(origin, ours),
        string_literal_changes(origin, theirs),
    ) {
        (None, None) => origin.clone(),
        (None, Some(m_string_literal)) => m_string_literal,
        (Some(m_string_literal), None) => m_string_literal,
        _ => return Err(format!("merge conflict in object {}", origin.id)),
    };

    Ok(m_string_literal)
}

fn merge_compound_statement(
    origin: &statement_object::compound_statement::CompoundStatement,
    ours: &statement_object::compound_statement::CompoundStatement,
    theirs: &statement_object::compound_statement::CompoundStatement,
) -> Result<statement_object::compound_statement::CompoundStatement, String> {
    let mut m_code_block = vec![];
    for (i, element) in origin.code_block.iter().enumerate() {
        m_code_block.push(merge_compound_statement_object(
            &element,
            &ours.code_block[i],
            &theirs.code_block[i],
        )?);
    }

    let mut m_compound_statement = origin.clone();
    m_compound_statement.code_block = m_code_block;

    Ok(m_compound_statement)
}

fn else_clause_changes(
    origin: &statement_object::if_statement::else_clause::ElseClause,
    alt: &statement_object::if_statement::else_clause::ElseClause,
) -> Option<statement_object::if_statement::else_clause::ElseClause> {
    if alt.id != origin.id {
        Some(alt.clone())
    } else {
        None
    }
}

fn merge_else_clause(
    origin: &statement_object::if_statement::else_clause::ElseClause,
    ours: &statement_object::if_statement::else_clause::ElseClause,
    theirs: &statement_object::if_statement::else_clause::ElseClause,
) -> Result<statement_object::if_statement::else_clause::ElseClause, String> {
    let m_body = merge_compound_statement_object(
        origin.body.as_ref(),
        ours.body.as_ref(),
        theirs.body.as_ref(),
    )?;

    let mut m_else_clause = origin.clone();
    m_else_clause.body = Box::new(m_body);
    Ok(m_else_clause)
}

fn else_statement_changes(
    origin: &statement_object::if_statement::ElseStatement,
    alt: &statement_object::if_statement::ElseStatement,
) -> Result<Option<statement_object::if_statement::ElseStatement>, String> {
    match (origin, alt) {
        (
            statement_object::if_statement::ElseStatement::ElseClause(origin_else_clause),
            statement_object::if_statement::ElseStatement::ElseClause(alt_else_clause),
        ) => Ok(else_clause_changes(origin_else_clause, alt_else_clause)
            .map(|opt| statement_object::if_statement::ElseStatement::ElseClause(Box::new(opt)))),
        (
            statement_object::if_statement::ElseStatement::ElseIf(origin_else_if),
            statement_object::if_statement::ElseStatement::ElseIf(alt_else_if),
        ) => Ok(if_statement_changes(origin_else_if, alt_else_if)
            .map(|opt| statement_object::if_statement::ElseStatement::ElseIf(Box::new(opt)))),
        _ => Err("".to_string()),
    }
}

fn merge_else_statement(
    origin: &statement_object::if_statement::ElseStatement,
    ours: &statement_object::if_statement::ElseStatement,
    theirs: &statement_object::if_statement::ElseStatement,
) -> Result<statement_object::if_statement::ElseStatement, String> {
    let m_else_statement =
        match (origin, ours, theirs) {
            (
                statement_object::if_statement::ElseStatement::ElseClause(origin_else_clause),
                statement_object::if_statement::ElseStatement::ElseClause(ours_else_clause),
                statement_object::if_statement::ElseStatement::ElseClause(theirs_else_clause),
            ) => statement_object::if_statement::ElseStatement::ElseClause(Box::new(
                merge_else_clause(origin_else_clause, ours_else_clause, theirs_else_clause)?,
            )),
            (
                statement_object::if_statement::ElseStatement::ElseIf(origin_else_if),
                statement_object::if_statement::ElseStatement::ElseIf(ours_else_if),
                statement_object::if_statement::ElseStatement::ElseIf(theirs_else_if),
            ) => statement_object::if_statement::ElseStatement::ElseIf(Box::new(
                merge_if_statement(origin_else_if, ours_else_if, theirs_else_if)?,
            )),
            (
                statement_object::if_statement::ElseStatement::ElseClause(origin_else_clause),
                statement_object::if_statement::ElseStatement::ElseClause(ours_else_clause),
                statement_object::if_statement::ElseStatement::ElseIf(_),
            ) => {
                if else_clause_changes(origin_else_clause, ours_else_clause).is_none() {
                    theirs.clone()
                } else {
                    return Err("".to_string());
                }
            }
            (
                statement_object::if_statement::ElseStatement::ElseClause(origin_else_clause),
                statement_object::if_statement::ElseStatement::ElseIf(_),
                statement_object::if_statement::ElseStatement::ElseClause(theirs_else_clause),
            ) => {
                if else_clause_changes(origin_else_clause, theirs_else_clause).is_none() {
                    ours.clone()
                } else {
                    return Err("".to_string());
                }
            }
            (
                statement_object::if_statement::ElseStatement::ElseIf(origin_else_if),
                statement_object::if_statement::ElseStatement::ElseIf(ours_else_if),
                statement_object::if_statement::ElseStatement::ElseClause(_),
            ) => {
                if if_statement_changes(origin_else_if, ours_else_if).is_none() {
                    theirs.clone()
                } else {
                    return Err("".to_string());
                }
            }
            (
                statement_object::if_statement::ElseStatement::ElseIf(origin_else_if),
                statement_object::if_statement::ElseStatement::ElseClause(_),
                statement_object::if_statement::ElseStatement::ElseIf(theirs_else_if),
            ) => {
                if if_statement_changes(origin_else_if, theirs_else_if).is_none() {
                    ours.clone()
                } else {
                    return Err("".to_string());
                }
            }
            _ => return Err("".to_string()),
        };

    Ok(m_else_statement)
}

fn merge_ifs_else_statement(
    origin: &Option<statement_object::if_statement::ElseStatement>,
    ours: &Option<statement_object::if_statement::ElseStatement>,
    theirs: &Option<statement_object::if_statement::ElseStatement>,
) -> Result<Option<statement_object::if_statement::ElseStatement>, String> {
    match (origin, ours, theirs) {
        (Some(origin_statement), Some(ours_statement), Some(theirs_statement)) => {
            let value = merge_else_statement(origin_statement, ours_statement, theirs_statement)?;
            Ok(Some(value))
        }
        (None, None, None) => Ok(None),
        (Some(origin_expression), None, Some(theirs_expression)) => {
            if else_statement_changes(origin_expression, theirs_expression)?.is_none() {
                Ok(None)
            } else {
                Err("merge conflict in declaration value".to_string())
            }
        }
        (Some(origin_expression), Some(ours_expression), None) => {
            if else_statement_changes(origin_expression, ours_expression)?.is_none() {
                Ok(None)
            } else {
                Err("merge conflict in declaration value".to_string())
            }
        }
        (None, Some(ours_expression), None) => Ok(Some(ours_expression.clone())),
        (None, None, Some(theirs_expression)) => Ok(Some(theirs_expression.clone())),
        _ => Err("merge conflict in declaration value".to_string()),
    }
}

fn if_statement_changes(
    origin: &statement_object::if_statement::IfStatement,
    alt: &statement_object::if_statement::IfStatement,
) -> Option<statement_object::if_statement::IfStatement> {
    if alt.id != origin.id {
        Some(alt.clone())
    } else {
        None
    }
}

fn merge_if_statement(
    origin: &statement_object::if_statement::IfStatement,
    ours: &statement_object::if_statement::IfStatement,
    theirs: &statement_object::if_statement::IfStatement,
) -> Result<statement_object::if_statement::IfStatement, String> {
    let m_condition =
        merge_expression_objects(&origin.condition, &ours.condition, &theirs.condition)?;

    let m_body = merge_compound_statement_object(&origin.body, &ours.body, &theirs.body)?;

    let m_else_statement = merge_ifs_else_statement(
        &origin.else_statement,
        &ours.else_statement,
        &theirs.else_statement,
    )?;

    let mut m_if_statement = origin.clone();
    m_if_statement.condition = Box::new(m_condition);
    m_if_statement.body = Box::new(m_body);
    m_if_statement.else_statement = m_else_statement;

    Ok(m_if_statement)
}

fn merge_return_statement(
    origin: &statement_object::return_statement::ReturnStatement,
    ours: &statement_object::return_statement::ReturnStatement,
    theirs: &statement_object::return_statement::ReturnStatement,
) -> Result<statement_object::return_statement::ReturnStatement, String> {
    let m_value = merge_value(
        origin.value.as_ref(),
        ours.value.as_ref(),
        theirs.value.as_ref(),
    )?;

    let mut m_return_statement = origin.clone();
    m_return_statement.value = m_value;
    Ok(m_return_statement)
}

#[cfg(test)]

mod tests {
    use core::num;

    use super::*;
    use language::language::c::{
        c_type::CType,
        language_object::{
            expression_object::binary_expression,
            statement_object::compound_statement::compound_statement_object,
        },
    };
    use uuid::Uuid;

    #[test]
    fn test_01_merge_empty_files() {
        let origin = special_object::source_file::SourceFile {
            id: Uuid::new_v4(),
            code: vec![],
        };

        let merger = Merger::new();

        let merge = merger
            .merge(origin.clone(), origin.clone(), origin.clone())
            .unwrap();

        assert_eq!(merge.id, origin.id);
        assert_eq!(merge.code.len(), origin.code.len());
    }

    #[test]
    fn test_02_merge_comment() {
        let id = Uuid::new_v4();
        let content_comment_origin = "origin".to_string();
        let comment_origin = special_object::comment::Comment {
            id,
            content: content_comment_origin.clone(),
        };

        let content_comment_ours = "ours".to_string();
        let comment_ours = special_object::comment::Comment {
            id,
            content: content_comment_ours.clone(),
        };

        let comment_theirs = special_object::comment::Comment {
            id,
            content: content_comment_origin,
        };

        let comment = merge_comment(&comment_origin, &comment_ours, &comment_theirs).unwrap();

        assert_eq!(comment.id, id);
        assert_eq!(comment.content, content_comment_ours);
    }

    #[test]
    fn test_03_merge_conflict_with_comment() {
        let id = Uuid::new_v4();
        let content_comment_origin = "origin".to_string();
        let comment_origin = special_object::comment::Comment {
            id,
            content: content_comment_origin.clone(),
        };

        let content_comment_ours = "ours".to_string();
        let comment_ours = special_object::comment::Comment {
            id,
            content: content_comment_ours.clone(),
        };

        let content_comment_theirs = "theirs".to_string();
        let comment_theirs = special_object::comment::Comment {
            id,
            content: content_comment_theirs,
        };

        let merge = merge_comment(&comment_origin, &comment_ours, &comment_theirs);

        assert!(merge.is_err());
    }

    #[test]
    fn test_04_merge_declaration_without_value() {
        let id = Uuid::new_v4();
        let primitive_type = CType::Int;
        let declaration_identifier_origin = "origin".to_string();
        let declaration_origin = declaration_object::declaration::Declaration {
            id,
            primitive_type: primitive_type.clone(),
            identifier: declaration_identifier_origin.clone(),
            value: None,
        };

        let declaration_ours = declaration_object::declaration::Declaration {
            id,
            primitive_type: primitive_type.clone(),
            identifier: declaration_identifier_origin,
            value: None,
        };

        let declaration_identifier_theirs = "theirs".to_string();
        let declaration_theirs = declaration_object::declaration::Declaration {
            id,
            primitive_type: primitive_type.clone(),
            identifier: declaration_identifier_theirs.clone(),
            value: None,
        };

        let declaration =
            merge_declaration(&declaration_origin, &declaration_ours, &declaration_theirs).unwrap();

        assert_eq!(declaration.id, id);
        assert_eq!(declaration.primitive_type, primitive_type);
        assert_eq!(declaration.identifier, declaration_identifier_theirs);
        assert!(declaration.value.is_none());
    }

    #[test]
    fn test_05_merge_declaration_values() {
        let id = Uuid::new_v4();
        let primitive_type = CType::Int;
        let declaration_identifier_origin = "origin".to_string();
        let declaration_origin = declaration_object::declaration::Declaration {
            id,
            primitive_type: primitive_type.clone(),
            identifier: declaration_identifier_origin.clone(),
            value: None,
        };

        let id_number_literal = Uuid::new_v4();
        let value = "0".to_string();
        let number_literal = expression_object::number_literal::NumberLiteral {
            id: id_number_literal,
            value: value.clone(),
        };

        let declaration_identifier_ours = "ours".to_string();
        let declaration_ours = declaration_object::declaration::Declaration {
            id,
            primitive_type: primitive_type.clone(),
            identifier: declaration_identifier_ours.clone(),
            value: Some(Box::new(
                expression_object::ExpressionObject::NumberLiteral(number_literal),
            )),
        };

        let declaration = merge_declaration(
            &declaration_origin,
            &declaration_ours,
            &declaration_origin.clone(),
        )
        .unwrap();

        assert_eq!(declaration.id, id);
        assert_eq!(declaration.primitive_type, primitive_type);
        assert_eq!(declaration.identifier, declaration_identifier_ours);
        if let Some(expression_object::ExpressionObject::NumberLiteral(number_literal)) =
            declaration.value.as_deref()
        {
            assert_eq!(number_literal.id, id_number_literal);
            assert_eq!(number_literal.value, value);
        }
    }

    #[test]
    fn test_06_merge_function_declaration() {
        let id_param = Uuid::new_v4();
        let identifier_param_origin = "param_origin".to_string();
        let param_type_origin = CType::Int;
        let param_origin =
            declaration_object::function_declaration::function_parameter::FunctionParameter {
                id: id_param,
                identifier: identifier_param_origin,
                param_type: param_type_origin,
            };

        let id = Uuid::new_v4();
        let return_type_origin = CType::Void;
        let identifier_origin = "origin".to_string();
        let function_declaration_origin =
            declaration_object::function_declaration::FunctionDeclaration {
                id,
                return_type: return_type_origin,
                identifier: identifier_origin,
                parameter_list: vec![param_origin],
            };

        let identifier_param_ours = "param_ours".to_string();
        let param_type_ours = CType::Float;
        let param_ours =
            declaration_object::function_declaration::function_parameter::FunctionParameter {
                id: id_param,
                identifier: identifier_param_ours.clone(),
                param_type: param_type_ours.clone(),
            };

        let return_type_ours = CType::Void;
        let identifier_ours = "ours".to_string();
        let function_declaration_ours =
            declaration_object::function_declaration::FunctionDeclaration {
                id,
                return_type: return_type_ours.clone(),
                identifier: identifier_ours.clone(),
                parameter_list: vec![param_ours],
            };

        let function_declaration = merge_function_declaration(
            &function_declaration_origin,
            &function_declaration_ours,
            &function_declaration_origin.clone(),
        )
        .unwrap();

        assert_eq!(function_declaration.id, id);
        assert_eq!(function_declaration.return_type, return_type_ours);
        assert_eq!(function_declaration.identifier, identifier_ours);
        assert_eq!(
            function_declaration.parameter_list.len(),
            function_declaration_origin.parameter_list.len()
        );
        assert_eq!(function_declaration.parameter_list[0].id, id_param);
        assert_eq!(
            function_declaration.parameter_list[0].identifier,
            identifier_param_ours
        );
        assert_eq!(
            function_declaration.parameter_list[0].param_type,
            param_type_ours
        );
    }

    #[test]
    fn test_07_merge_function_declaration_conflict_in_parameters() {
        let id_param = Uuid::new_v4();
        let identifier_param_origin = "param_origin".to_string();
        let param_type_origin = CType::Int;
        let param_origin =
            declaration_object::function_declaration::function_parameter::FunctionParameter {
                id: id_param,
                identifier: identifier_param_origin,
                param_type: param_type_origin,
            };

        let id = Uuid::new_v4();
        let return_type_origin = CType::Void;
        let identifier_origin = "origin".to_string();
        let function_declaration_origin =
            declaration_object::function_declaration::FunctionDeclaration {
                id,
                return_type: return_type_origin.clone(),
                identifier: identifier_origin.clone(),
                parameter_list: vec![param_origin],
            };

        let identifier_param_ours = "param_ours".to_string();
        let param_type_ours = CType::Float;
        let param_ours =
            declaration_object::function_declaration::function_parameter::FunctionParameter {
                id: id_param,
                identifier: identifier_param_ours.clone(),
                param_type: param_type_ours.clone(),
            };

        let return_type_ours = CType::Void;
        let identifier_ours = "ours".to_string();
        let function_declaration_ours =
            declaration_object::function_declaration::FunctionDeclaration {
                id,
                return_type: return_type_ours.clone(),
                identifier: identifier_ours.clone(),
                parameter_list: vec![param_ours],
            };

        let identifier_param_theirs = "param_theirs".to_string();
        let param_type_theirs = CType::Char;
        let param_theirs =
            declaration_object::function_declaration::function_parameter::FunctionParameter {
                id: id_param,
                identifier: identifier_param_theirs.clone(),
                param_type: param_type_theirs.clone(),
            };

        let function_declaration_theirs =
            declaration_object::function_declaration::FunctionDeclaration {
                id,
                return_type: return_type_origin,
                identifier: identifier_origin,
                parameter_list: vec![param_theirs],
            };

        assert!(
            merge_function_declaration(
                &function_declaration_origin,
                &function_declaration_ours,
                &function_declaration_theirs,
            )
            .is_err()
        );
    }

    #[test]
    fn test_08_crossed_name_changes() {
        // Creating origin

        let id_decl_one = Uuid::new_v4();
        let primitive_type_one = CType::Int;
        let identifier_one = "a".to_string();
        let mut declaration_one = declaration_object::declaration::Declaration {
            id: id_decl_one,
            primitive_type: primitive_type_one,
            identifier: identifier_one.clone(),
            value: None,
        };

        let id_decl_two = Uuid::new_v4();
        let primitive_type_two = CType::Int;
        let identifier_two = "b".to_string();
        let mut declaration_two = declaration_object::declaration::Declaration {
            id: id_decl_two,
            primitive_type: primitive_type_two,
            identifier: identifier_two.clone(),
            value: None,
        };

        let id_ref_one = Uuid::new_v4();
        let mut reference_one = expression_object::reference::Reference {
            id: id_ref_one,
            declaration_id: id_decl_one,
            identifier: identifier_one.clone(),
        };

        let id_ref_two = Uuid::new_v4();
        let mut reference_two = expression_object::reference::Reference {
            id: id_ref_two,
            declaration_id: id_decl_two,
            identifier: identifier_two.clone(),
        };

        let id_operation = Uuid::new_v4();
        let operator = "+".to_string();
        let mut operation = expression_object::binary_expression::BinaryExpression {
            id: id_operation,
            left: Box::new(expression_object::ExpressionObject::Reference(
                reference_one.clone(),
            )),
            operator: operator.clone(),
            right: Box::new(expression_object::ExpressionObject::Reference(
                reference_two.clone(),
            )),
        };

        let id_comp_stmt = Uuid::new_v4();
        let comp_stmt = statement_object::compound_statement::CompoundStatement {
            id: id_comp_stmt,
            code_block: vec![
                compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_one.clone(),
                ),
                compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_two.clone(),
                ),
                compound_statement_object::CompoundStatementObject::BinaryExpression(
                    operation.clone(),
                ),
            ],
        };

        let id_function = Uuid::new_v4();
        let return_type = CType::Void;
        let identifier_function = "main".to_string();
        let function = declaration_object::function_definition::FunctionDefinition {
            id: id_function,
            return_type: return_type.clone(),
            identifier: identifier_function.clone(),
            parameter_list: vec![],
            compound_statement: comp_stmt.clone(),
        };

        let origin = special_object::source_file::SourceFile {
            id: Uuid::new_v4(),
            code: vec![declaration_object::DeclarationObject::FunctionDefinition(
                function.clone(),
            )],
        };

        //Creating ours

        let new_identifier_one = "A".to_string();
        declaration_one.identifier = new_identifier_one.clone();
        reference_one.identifier = new_identifier_one.clone();

        operation.left = Box::new(expression_object::ExpressionObject::Reference(
            reference_one.clone(),
        ));

        let comp_stmt_ours = statement_object::compound_statement::CompoundStatement {
            id: id_comp_stmt,
            code_block: vec![
                compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_one.clone(),
                ),
                compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_two.clone(),
                ),
                compound_statement_object::CompoundStatementObject::BinaryExpression(
                    operation.clone(),
                ),
            ],
        };

        let function_ours = declaration_object::function_definition::FunctionDefinition {
            id: id_function,
            return_type: return_type.clone(),
            identifier: identifier_function.clone(),
            parameter_list: vec![],
            compound_statement: comp_stmt_ours,
        };

        let ours = special_object::source_file::SourceFile {
            id: Uuid::new_v4(),
            code: vec![declaration_object::DeclarationObject::FunctionDefinition(
                function_ours,
            )],
        };

        //Creating theirs

        declaration_one.identifier = identifier_one.clone();
        reference_one.identifier = identifier_one;

        let new_identifier_two = "B".to_string();
        declaration_two.identifier = new_identifier_two.clone();
        reference_two.identifier = new_identifier_two.clone();

        operation.left = Box::new(expression_object::ExpressionObject::Reference(
            reference_one.clone(),
        ));

        operation.right = Box::new(expression_object::ExpressionObject::Reference(
            reference_two.clone(),
        ));

        let comp_stmt_theirs = statement_object::compound_statement::CompoundStatement {
            id: id_comp_stmt,
            code_block: vec![
                compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_one.clone(),
                ),
                compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_two.clone(),
                ),
                compound_statement_object::CompoundStatementObject::BinaryExpression(
                    operation.clone(),
                ),
            ],
        };

        let function_theirs = declaration_object::function_definition::FunctionDefinition {
            id: id_function,
            return_type,
            identifier: identifier_function,
            parameter_list: vec![],
            compound_statement: comp_stmt_theirs,
        };

        let theirs = special_object::source_file::SourceFile {
            id: Uuid::new_v4(),
            code: vec![declaration_object::DeclarationObject::FunctionDefinition(
                function_theirs,
            )],
        };

        //Testing

        let merger = Merger::new();

        let merge = merger.merge(origin.clone(), ours, theirs).unwrap();

        assert_eq!(merge.id, origin.id);
        assert_eq!(merge.code.len(), origin.code.len());

        if let declaration_object::DeclarationObject::FunctionDefinition(func) = &merge.code[0] {
            assert_eq!(func.id, function.id);
            assert_eq!(func.return_type, function.return_type);
            assert_eq!(func.identifier, function.identifier);
            assert_eq!(&func.parameter_list.len(), &function.parameter_list.len());

            let merge_comp_stmt = func.compound_statement.clone();
            assert_eq!(merge_comp_stmt.code_block.len(), comp_stmt.code_block.len());

            if let compound_statement_object::CompoundStatementObject::Declaration(decl) =
                &merge_comp_stmt.code_block[0]
            {
                assert_eq!(decl.id, declaration_one.id);
                assert_eq!(decl.primitive_type, declaration_one.primitive_type);
                assert_eq!(decl.identifier, new_identifier_one);
                assert!(decl.value.is_none());
            } else {
                panic!("expected Declaration");
            }

            if let compound_statement_object::CompoundStatementObject::Declaration(decl) =
                &merge_comp_stmt.code_block[1]
            {
                assert_eq!(decl.id, declaration_two.id);
                assert_eq!(decl.primitive_type, declaration_two.primitive_type);
                assert_eq!(decl.identifier, new_identifier_two);
                assert!(decl.value.is_none());
            } else {
                panic!("expected Declaration");
            }

            if let compound_statement_object::CompoundStatementObject::BinaryExpression(binary) =
                &merge_comp_stmt.code_block[2]
            {
                assert_eq!(binary.id, operation.id);
                assert_eq!(binary.operator, operator);

                if let expression_object::ExpressionObject::Reference(reference) = &*binary.left {
                    assert_eq!(reference.id, reference_one.id);
                    assert_eq!(reference.declaration_id, reference_one.declaration_id);
                    assert_eq!(reference.identifier, new_identifier_one);
                } else {
                    panic!("expected Reference");
                }

                if let expression_object::ExpressionObject::Reference(reference) = &*binary.right {
                    assert_eq!(reference.id, reference_two.id);
                    assert_eq!(reference.declaration_id, reference_two.declaration_id);
                    assert_eq!(reference.identifier, new_identifier_two);
                } else {
                    panic!("expected Reference");
                }
            } else {
                panic!("expected BinaryExpression");
            }
        } else {
            panic!("expected FunctionDefinition");
        }
    }

    #[test]
    fn test_09() {
        // Creating origin
        let id_number_one = Uuid::new_v4();
        let value_one = "1".to_string();
        let number_one = expression_object::number_literal::NumberLiteral {
            id: id_number_one,
            value: value_one,
        };

        let id_number_two = Uuid::new_v4();
        let value_two = "2".to_string();
        let number_two = expression_object::number_literal::NumberLiteral {
            id: id_number_two,
            value: value_two,
        };

        let id_decl_one = Uuid::new_v4();
        let primitive_type_one = CType::Int;
        let mut identifier_one = "a".to_string();
        let declaration_one = declaration_object::declaration::Declaration {
            id: id_decl_one,
            primitive_type: primitive_type_one.clone(),
            identifier: identifier_one.clone(),
            value: Some(Box::new(
                expression_object::ExpressionObject::NumberLiteral(number_one.clone()),
            )),
        };

        let id_decl_two = Uuid::new_v4();
        let primitive_type_two = CType::Int;
        let mut identifier_two = "b".to_string();
        let declaration_two = declaration_object::declaration::Declaration {
            id: id_decl_two,
            primitive_type: primitive_type_two.clone(),
            identifier: identifier_two.clone(),
            value: Some(Box::new(
                expression_object::ExpressionObject::NumberLiteral(number_two.clone()),
            )),
        };

        let id_ref_one = Uuid::new_v4();
        let reference_one = expression_object::reference::Reference {
            id: id_ref_one,
            declaration_id: id_decl_one,
            identifier: identifier_one.clone(),
        };

        let id_ref_two = Uuid::new_v4();
        let reference_two = expression_object::reference::Reference {
            id: id_ref_two,
            declaration_id: id_decl_two,
            identifier: identifier_two.clone(),
        };

        let id_operation = Uuid::new_v4();
        let operator = "+".to_string();
        let operation = expression_object::binary_expression::BinaryExpression {
            id: id_operation,
            left: Box::new(expression_object::ExpressionObject::Reference(
                reference_one.clone(),
            )),
            operator: operator.clone(),
            right: Box::new(expression_object::ExpressionObject::Reference(
                reference_two.clone(),
            )),
        };

        let id_decl_three = Uuid::new_v4();
        let primitive_type_three = CType::Int;
        let identifier_three = "c".to_string();
        let declaration_three = declaration_object::declaration::Declaration {
            id: id_decl_three,
            primitive_type: primitive_type_three.clone(),
            identifier: identifier_three.clone(),
            value: Some(Box::new(
                expression_object::ExpressionObject::BinaryExpression(operation.clone()),
            )),
        };

        let id_comp_stmt = Uuid::new_v4();
        let comp_stmt = statement_object::compound_statement::CompoundStatement {
            id: id_comp_stmt,
            code_block: vec![
                compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_one.clone(),
                ),
                compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_two.clone(),
                ),
                compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_three.clone(),
                ),
            ],
        };

        let id_function = Uuid::new_v4();
        let return_type = CType::Void;
        let identifier_function = "main".to_string();
        let function = declaration_object::function_definition::FunctionDefinition {
            id: id_function,
            return_type: return_type.clone(),
            identifier: identifier_function.clone(),
            parameter_list: vec![],
            compound_statement: comp_stmt.clone(),
        };

        let id = Uuid::new_v4();
        let origin = special_object::source_file::SourceFile {
            id,
            code: vec![declaration_object::DeclarationObject::FunctionDefinition(
                function.clone(),
            )],
        };

        // Creating ours

        let new_identifier_one = "A".to_string();
        let declaration_one = declaration_object::declaration::Declaration {
            id: id_decl_one,
            primitive_type: primitive_type_one.clone(),
            identifier: new_identifier_one.clone(),
            value: Some(Box::new(
                expression_object::ExpressionObject::NumberLiteral(number_one.clone()),
            )),
        };

        let new_identifier_two = "B".to_string();
        let declaration_two = declaration_object::declaration::Declaration {
            id: id_decl_two,
            primitive_type: primitive_type_two.clone(),
            identifier: new_identifier_two.clone(),
            value: Some(Box::new(
                expression_object::ExpressionObject::NumberLiteral(number_two.clone()),
            )),
        };

        let reference_one = expression_object::reference::Reference {
            id: id_ref_one,
            declaration_id: id_decl_one,
            identifier: identifier_one.clone(),
        };

        let reference_two = expression_object::reference::Reference {
            id: id_ref_two,
            declaration_id: id_decl_two,
            identifier: identifier_two.clone(),
        };

        let operator = "+".to_string();
        let operation = expression_object::binary_expression::BinaryExpression {
            id: id_operation,
            left: Box::new(expression_object::ExpressionObject::Reference(
                reference_one.clone(),
            )),
            operator: operator.clone(),
            right: Box::new(expression_object::ExpressionObject::Reference(
                reference_two.clone(),
            )),
        };

        let new_identifier_three = "C".to_string();
        let declaration_three = declaration_object::declaration::Declaration {
            id: id_decl_three,
            primitive_type: primitive_type_three.clone(),
            identifier: new_identifier_three.clone(),
            value: Some(Box::new(
                expression_object::ExpressionObject::BinaryExpression(operation.clone()),
            )),
        };

        let comp_stmt = statement_object::compound_statement::CompoundStatement {
            id: id_comp_stmt,
            code_block: vec![
                compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_one.clone(),
                ),
                compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_two.clone(),
                ),
                compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_three.clone(),
                ),
            ],
        };

        let identifier_function = "main".to_string();
        let function = declaration_object::function_definition::FunctionDefinition {
            id: id_function,
            return_type: return_type.clone(),
            identifier: identifier_function.clone(),
            parameter_list: vec![],
            compound_statement: comp_stmt.clone(),
        };

        let ours = special_object::source_file::SourceFile {
            id: Uuid::new_v4(),
            code: vec![declaration_object::DeclarationObject::FunctionDefinition(
                function.clone(),
            )],
        };

        // Theirs

        identifier_one = "a".to_string();
        let declaration_one = declaration_object::declaration::Declaration {
            id: id_decl_one,
            primitive_type: primitive_type_one.clone(),
            identifier: identifier_one.clone(),
            value: Some(Box::new(
                expression_object::ExpressionObject::NumberLiteral(number_one.clone()),
            )),
        };

        identifier_two = "b".to_string();
        let declaration_two = declaration_object::declaration::Declaration {
            id: id_decl_two,
            primitive_type: primitive_type_two.clone(),
            identifier: identifier_two.clone(),
            value: Some(Box::new(
                expression_object::ExpressionObject::NumberLiteral(number_two.clone()),
            )),
        };

        let reference_one = expression_object::reference::Reference {
            id: id_ref_one,
            declaration_id: id_decl_one,
            identifier: identifier_one.clone(),
        };

        let reference_two = expression_object::reference::Reference {
            id: id_ref_two,
            declaration_id: id_decl_two,
            identifier: identifier_two.clone(),
        };

        let operator = "+".to_string();
        let operation = expression_object::binary_expression::BinaryExpression {
            id: id_operation,
            left: Box::new(expression_object::ExpressionObject::Reference(
                reference_two.clone(),
            )),
            operator: operator.clone(),
            right: Box::new(expression_object::ExpressionObject::Reference(
                reference_one.clone(),
            )),
        };

        let identifier_three = "c".to_string();
        let declaration_three = declaration_object::declaration::Declaration {
            id: id_decl_three,
            primitive_type: primitive_type_three.clone(),
            identifier: identifier_three.clone(),
            value: Some(Box::new(
                expression_object::ExpressionObject::BinaryExpression(operation.clone()),
            )),
        };

        let comp_stmt = statement_object::compound_statement::CompoundStatement {
            id: id_comp_stmt,
            code_block: vec![
                compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_one.clone(),
                ),
                compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_two.clone(),
                ),
                compound_statement_object::CompoundStatementObject::Declaration(
                    declaration_three.clone(),
                ),
            ],
        };

        let identifier_function = "main".to_string();
        let function = declaration_object::function_definition::FunctionDefinition {
            id: id_function,
            return_type: return_type.clone(),
            identifier: identifier_function.clone(),
            parameter_list: vec![],
            compound_statement: comp_stmt.clone(),
        };

        let theirs = special_object::source_file::SourceFile {
            id: Uuid::new_v4(),
            code: vec![declaration_object::DeclarationObject::FunctionDefinition(
                function.clone(),
            )],
        };

        // Testing

        let merger = Merger::new();

        let merge = merger.merge(origin.clone(), ours, theirs).unwrap();

        assert_eq!(merge.id, origin.id);
        assert_eq!(merge.code.len(), origin.code.len());

        if let declaration_object::DeclarationObject::FunctionDefinition(func) = &merge.code[0] {
            assert_eq!(func.id, function.id);
            assert_eq!(func.return_type, function.return_type);
            assert_eq!(func.identifier, function.identifier);
            assert_eq!(&func.parameter_list.len(), &function.parameter_list.len());

            let merge_comp_stmt = func.compound_statement.clone();
            assert_eq!(merge_comp_stmt.code_block.len(), comp_stmt.code_block.len());

            if let compound_statement_object::CompoundStatementObject::Declaration(decl) =
                &merge_comp_stmt.code_block[0]
            {
                assert_eq!(decl.id, declaration_one.id);
                assert_eq!(decl.primitive_type, declaration_one.primitive_type);
                assert_eq!(decl.identifier, new_identifier_one);
                if let expression_object::ExpressionObject::NumberLiteral(number) =
                    decl.value.as_ref().unwrap().as_ref()
                {
                    assert_eq!(number.id, number_one.id);
                    assert_eq!(number.value, number_one.value);
                }
            } else {
                panic!("expected NumberLiteral");
            }

            if let compound_statement_object::CompoundStatementObject::Declaration(decl) =
                &merge_comp_stmt.code_block[1]
            {
                assert_eq!(decl.id, declaration_two.id);
                assert_eq!(decl.primitive_type, declaration_two.primitive_type);
                assert_eq!(decl.identifier, new_identifier_two);
                if let expression_object::ExpressionObject::NumberLiteral(number) =
                    decl.value.as_ref().unwrap().as_ref()
                {
                    assert_eq!(number.id, number_two.id);
                    assert_eq!(number.value, number_two.value);
                }
            } else {
                panic!("expected Declaration");
            }

            if let compound_statement_object::CompoundStatementObject::Declaration(decl) =
                &merge_comp_stmt.code_block[2]
            {
                assert_eq!(decl.id, declaration_three.id);
                assert_eq!(decl.primitive_type, declaration_three.primitive_type);
                assert_eq!(decl.identifier, new_identifier_three);

                if let expression_object::ExpressionObject::BinaryExpression(binary) =
                    decl.value.as_ref().unwrap().as_ref()
                {
                    assert_eq!(binary.id, operation.id);
                    assert_eq!(binary.operator, operator);

                    if let expression_object::ExpressionObject::Reference(reference) =
                        binary.left.as_ref()
                    {
                        assert_eq!(reference.id, reference_two.id);
                        assert_eq!(reference.declaration_id, reference_two.declaration_id);
                    } else {
                        panic!("expected Reference");
                    }

                    if let expression_object::ExpressionObject::Reference(reference) =
                        binary.right.as_ref()
                    {
                        assert_eq!(reference.id, reference_one.id);
                        assert_eq!(reference.declaration_id, reference_one.declaration_id);
                    } else {
                        panic!("expected Reference");
                    }
                }
            } else {
                panic!("expected BinaryExpression");
            }
        } else {
            panic!("expected FunctionDefinition");
        }
    }
}
