use language::language::c::language_object::{
    self, LanguageObject, declaration_object::DeclarationObject,
    statement_object::compound_statement::compound_statement_object::CompoundStatementObject,
};

pub fn replace_object(
    object: &mut LanguageObject,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    match object {
        LanguageObject::SourceFile(obj) => replace_source_file(obj, new_object),

        LanguageObject::AssignmentExpression(obj) => replace_assignment_expression(obj, new_object),
        LanguageObject::BinaryExpression(obj) => replace_binary_expression(obj, new_object),
        LanguageObject::CallExpression(obj) => replace_call_expression(obj, new_object),
        LanguageObject::NumberLiteral(obj) => replace_number_literal(obj, new_object),
        LanguageObject::Reference(obj) => replace_reference(obj, new_object),
        LanguageObject::StringLiteral(obj) => replace_string_literal(obj, new_object),

        LanguageObject::CompoundStatement(obj) => replace_compound_statement(obj, new_object),
        LanguageObject::IfStatement(obj) => replace_if_statement(obj, new_object),
        LanguageObject::ReturnStatement(obj) => replace_return_statement(obj, new_object),

        LanguageObject::Declaration(obj) => replace_declaration(obj, new_object),
        LanguageObject::FunctionDeclaration(obj) => replace_function_declaration(obj, new_object),
        LanguageObject::FunctionDefinition(obj) => replace_function_definition(obj, new_object),
        LanguageObject::PreprocInclude(obj) => replace_preproc_include(obj, new_object),

        LanguageObject::Comment(obj) => replace_comment(obj, new_object),
        LanguageObject::Unknown(obj) => replace_unknown(obj, new_object),

        LanguageObject::FunctionParameter(obj) => replace_function_parameter(obj, new_object),
        LanguageObject::ElseClause(obj) => replace_else_clause(obj, new_object),
    }
}

fn replace_expression_object(
    object: &mut language_object::expression_object::ExpressionObject,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    match object {
        language_object::expression_object::ExpressionObject::AssignmentExpression(obj) => {
            replace_assignment_expression(obj, new_object)
        }
        language_object::expression_object::ExpressionObject::BinaryExpression(obj) => {
            replace_binary_expression(obj, new_object)
        }
        language_object::expression_object::ExpressionObject::CallExpression(obj) => {
            replace_call_expression(obj, new_object)
        }
        language_object::expression_object::ExpressionObject::NumberLiteral(obj) => {
            replace_number_literal(obj, new_object)
        }
        language_object::expression_object::ExpressionObject::Reference(obj) => {
            replace_reference(obj, new_object)
        }
        language_object::expression_object::ExpressionObject::StringLiteral(obj) => {
            replace_string_literal(obj, new_object)
        }
        language_object::expression_object::ExpressionObject::Unknown(obj) => {
            replace_unknown(obj, new_object)
        }
    }
}

fn replace_statement_object(
    object: &mut language_object::statement_object::StatementObject,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    match object {
        language_object::statement_object::StatementObject::CompoundStatement(obj) => {
            replace_compound_statement(obj, new_object)
        }
        language_object::statement_object::StatementObject::IfStatement(obj) => {
            replace_if_statement(obj, new_object)
        }
        language_object::statement_object::StatementObject::ReturnStatement(obj) => {
            replace_return_statement(obj, new_object)
        }
        language_object::statement_object::StatementObject::Unknown(obj) => {
            replace_unknown(obj, new_object)
        }
    }
}

fn replace_declaration_object(
    object: &mut DeclarationObject,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    match object {
        DeclarationObject::Declaration(obj) => replace_declaration(obj, new_object),
        DeclarationObject::FunctionDeclaration(obj) => {
            replace_function_declaration(obj, new_object)
        }
        DeclarationObject::FunctionDefinition(obj) => replace_function_definition(obj, new_object),
        DeclarationObject::PreprocInclude(obj) => replace_preproc_include(obj, new_object),
        DeclarationObject::Comment(obj) => replace_comment(obj, new_object),
        DeclarationObject::Unknown(obj) => replace_unknown(obj, new_object),
    }
}

fn replace_compound_statement_object(
    object: &mut CompoundStatementObject,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    match object {
        CompoundStatementObject::AssignmentExpression(obj) => {
            replace_assignment_expression(obj, new_object)
        }
        CompoundStatementObject::BinaryExpression(obj) => {
            replace_binary_expression(obj, new_object)
        }
        CompoundStatementObject::CallExpression(obj) => replace_call_expression(obj, new_object),
        CompoundStatementObject::NumberLiteral(obj) => replace_number_literal(obj, new_object),
        CompoundStatementObject::Reference(obj) => replace_reference(obj, new_object),
        CompoundStatementObject::StringLiteral(obj) => replace_string_literal(obj, new_object),

        CompoundStatementObject::CompoundStatement(obj) => {
            replace_compound_statement(obj, new_object)
        }
        CompoundStatementObject::IfStatement(obj) => replace_if_statement(obj, new_object),
        CompoundStatementObject::ReturnStatement(obj) => replace_return_statement(obj, new_object),

        CompoundStatementObject::Declaration(obj) => replace_declaration(obj, new_object),

        CompoundStatementObject::Comment(obj) => replace_comment(obj, new_object),
        CompoundStatementObject::Unknown(obj) => replace_unknown(obj, new_object),
    }
}

pub fn replace_source_file(
    file: &mut language_object::special_object::source_file::SourceFile,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if file.id == new_object.id() {
        if let LanguageObject::SourceFile(new_file) = new_object {
            return Some(LanguageObject::SourceFile(std::mem::replace(
                file, new_file,
            )));
        }
    } else {
        for child in &mut file.code {
            if let Some(found) = replace_declaration_object(child, new_object.clone()) {
                return Some(found);
            }
        }
    }
    None
}

fn replace_assignment_expression(
    expr: &mut language_object::expression_object::assignment_expression::AssignmentExpression,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if expr.id == new_object.id() {
        if let LanguageObject::AssignmentExpression(new_expr) = new_object {
            return Some(LanguageObject::AssignmentExpression(std::mem::replace(
                expr, new_expr,
            )));
        }
    } else if let Some(found) = replace_expression_object(&mut expr.value, new_object) {
        return Some(found);
    }
    None
}

fn replace_binary_expression(
    expr: &mut language_object::expression_object::binary_expression::BinaryExpression,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if expr.id == new_object.id() {
        if let LanguageObject::BinaryExpression(new_expr) = new_object {
            return Some(LanguageObject::BinaryExpression(std::mem::replace(
                expr, new_expr,
            )));
        }
    } else {
        if let Some(found) = replace_expression_object(&mut expr.left, new_object.clone()) {
            return Some(found);
        }
        if let Some(found) = replace_expression_object(&mut expr.right, new_object) {
            return Some(found);
        }
    }
    None
}

fn replace_call_expression(
    call: &mut language_object::expression_object::call_expression::CallExpression,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if call.id == new_object.id() {
        if let LanguageObject::CallExpression(new_call) = new_object {
            return Some(LanguageObject::CallExpression(std::mem::replace(
                call, new_call,
            )));
        }
    } else {
        for arg in &mut call.argument_list {
            if let Some(found) = replace_expression_object(arg, new_object.clone()) {
                return Some(found);
            }
        }
    }
    None
}

fn replace_comment(
    comment: &mut language_object::special_object::comment::Comment,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if comment.id == new_object.id() {
        if let LanguageObject::Comment(new_comment) = new_object {
            return Some(LanguageObject::Comment(std::mem::replace(
                comment,
                new_comment,
            )));
        }
    }
    None
}

fn replace_declaration(
    decl: &mut language_object::declaration_object::declaration::Declaration,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if decl.id == new_object.id() {
        if let LanguageObject::Declaration(new_decl) = new_object {
            return Some(LanguageObject::Declaration(std::mem::replace(
                decl, new_decl,
            )));
        }
    } else if let Some(val) = &mut decl.value {
        if let Some(found) = replace_expression_object(val, new_object) {
            return Some(found);
        }
    }
    None
}

fn replace_else_clause(
    else_clause: &mut language_object::statement_object::if_statement::else_clause::ElseClause,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if else_clause.id == new_object.id() {
        if let LanguageObject::ElseClause(new_else) = new_object {
            return Some(LanguageObject::ElseClause(std::mem::replace(
                else_clause,
                new_else,
            )));
        }
    } else if let Some(found) = replace_compound_statement_object(&mut else_clause.body, new_object)
    {
        return Some(found);
    }
    None
}

fn replace_function_declaration(
    decl: &mut language_object::declaration_object::function_declaration::FunctionDeclaration,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if decl.id == new_object.id() {
        if let LanguageObject::FunctionDeclaration(new_decl) = new_object {
            return Some(LanguageObject::FunctionDeclaration(std::mem::replace(
                decl, new_decl,
            )));
        }
    } else {
        for param in &mut decl.parameter_list {
            if let Some(found) = replace_function_parameter(param, new_object.clone()) {
                return Some(found);
            }
        }
    }
    None
}

fn replace_function_definition(
    def: &mut language_object::declaration_object::function_definition::FunctionDefinition,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if def.id == new_object.id() {
        if let LanguageObject::FunctionDefinition(new_def) = new_object {
            return Some(LanguageObject::FunctionDefinition(std::mem::replace(
                def, new_def,
            )));
        }
    } else {
        for param in &mut def.parameter_list {
            if let Some(found) = replace_function_parameter(param, new_object.clone()) {
                return Some(found);
            }
        }
        if let Some(found) = replace_compound_statement(&mut def.compound_statement, new_object) {
            return Some(found);
        }
    }
    None
}

fn replace_function_parameter(
    param: &mut language_object::declaration_object::function_declaration::function_parameter::FunctionParameter,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if param.id == new_object.id() {
        if let LanguageObject::FunctionParameter(new_param) = new_object {
            return Some(LanguageObject::FunctionParameter(std::mem::replace(
                param, new_param,
            )));
        }
    }
    None
}

fn replace_if_statement(
    stmt: &mut language_object::statement_object::if_statement::IfStatement,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if stmt.id == new_object.id() {
        if let LanguageObject::IfStatement(new_stmt) = new_object {
            return Some(LanguageObject::IfStatement(std::mem::replace(
                stmt, new_stmt,
            )));
        }
    } else {
        if let Some(found) = replace_expression_object(&mut stmt.condition, new_object.clone()) {
            return Some(found);
        }
        if let Some(found) = replace_compound_statement_object(&mut stmt.body, new_object.clone()) {
            return Some(found);
        }
        if let Some(else_statement) = &mut stmt.else_statement {
            match else_statement {
                language_object::statement_object::if_statement::ElseStatement::ElseIf(
                    if_statement,
                ) => {
                    if let Some(found) = replace_if_statement(if_statement, new_object.clone()) {
                        return Some(found);
                    }
                }
                language_object::statement_object::if_statement::ElseStatement::ElseClause(
                    else_clause,
                ) => {
                    if let Some(found) = replace_else_clause(else_clause, new_object.clone()) {
                        return Some(found);
                    }
                }
            }
        }
    }
    None
}

fn replace_number_literal(
    lit: &mut language_object::expression_object::number_literal::NumberLiteral,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if lit.id == new_object.id() {
        if let LanguageObject::NumberLiteral(new_lit) = new_object {
            return Some(LanguageObject::NumberLiteral(std::mem::replace(
                lit, new_lit,
            )));
        }
    }
    None
}

fn replace_preproc_include(
    inc: &mut language_object::declaration_object::preproc_include::PreprocInclude,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if inc.id == new_object.id() {
        if let LanguageObject::PreprocInclude(new_inc) = new_object {
            return Some(LanguageObject::PreprocInclude(std::mem::replace(
                inc, new_inc,
            )));
        }
    }
    None
}

fn replace_reference(
    r: &mut language_object::expression_object::reference::Reference,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if r.id == new_object.id() {
        if let LanguageObject::Reference(new_r) = new_object {
            return Some(LanguageObject::Reference(std::mem::replace(r, new_r)));
        }
    }
    None
}

fn replace_return_statement(
    stmt: &mut language_object::statement_object::return_statement::ReturnStatement,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if stmt.id == new_object.id() {
        if let LanguageObject::ReturnStatement(new_stmt) = new_object {
            return Some(LanguageObject::ReturnStatement(std::mem::replace(
                stmt, new_stmt,
            )));
        }
    } else if let Some(found) = match &mut stmt.value {
        Some(value) => replace_expression_object(value, new_object),
        None => None,
    } {
        return Some(found);
    }
    None
}

fn replace_string_literal(
    lit: &mut language_object::expression_object::string_literal::StringLiteral,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if lit.id == new_object.id() {
        if let LanguageObject::StringLiteral(new_lit) = new_object {
            return Some(LanguageObject::StringLiteral(std::mem::replace(
                lit, new_lit,
            )));
        }
    }
    None
}

fn replace_compound_statement(
    comp_stmt: &mut language_object::statement_object::compound_statement::CompoundStatement,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if comp_stmt.id == new_object.id() {
        if let LanguageObject::CompoundStatement(new_comp_stmt) = new_object {
            return Some(LanguageObject::CompoundStatement(std::mem::replace(
                comp_stmt,
                new_comp_stmt,
            )));
        }
    } else {
        for child in &mut comp_stmt.code_block {
            if let Some(found) = replace_compound_statement_object(child, new_object.clone()) {
                return Some(found);
            }
        }
    }
    None
}

fn replace_unknown(
    unknown: &mut language_object::special_object::unknown::Unknown,
    new_object: LanguageObject,
) -> Option<LanguageObject> {
    if unknown.id == new_object.id() {
        if let LanguageObject::Unknown(new_unknown) = new_object {
            return Some(LanguageObject::Unknown(std::mem::replace(
                unknown,
                new_unknown,
            )));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;
    use language::language::c;
    use uuid::Uuid;

    #[test]
    fn test_01_replace_root() {
        let decl_id = Uuid::new_v4();
        let primitive_type = c::c_type::CType::Int;
        let decl_identifier = "a";
        let decl = c::language_object::declaration_object::declaration::Declaration {
            id: decl_id,
            primitive_type,
            identifier: decl_identifier.to_string(),
            value: None,
        };

        let param_id = Uuid::new_v4();
        let param_type = c::c_type::CType::Int;
        let param_identifier = "b";
        let param = c::language_object::declaration_object::function_declaration::function_parameter::FunctionParameter {
            id: param_id,
            identifier: param_identifier.to_string(),
            param_type,
        };

        let func_decl_id = Uuid::new_v4();
        let func_decl_return_type = c::c_type::CType::Int;
        let func_decl_identifier = "c";
        let func_decl =
            c::language_object::declaration_object::function_declaration::FunctionDeclaration {
                id: func_decl_id,
                return_type: func_decl_return_type,
                identifier: func_decl_identifier.to_string(),
                parameter_list: vec![param],
            };

        let src_file_id = Uuid::new_v4();
        let src_file = c::language_object::special_object::source_file::SourceFile {
            id: src_file_id,
            code: vec![
                c::language_object::declaration_object::DeclarationObject::Declaration(decl),
                c::language_object::declaration_object::DeclarationObject::FunctionDeclaration(
                    func_decl,
                ),
            ],
        };

        let replace = c::language_object::LanguageObject::SourceFile(
            c::language_object::special_object::source_file::SourceFile {
                id: src_file_id,
                code: vec![],
            },
        );

        let mut code = c::language_object::LanguageObject::SourceFile(src_file);
        let replaced_object = replace_object(&mut code, replace);

        assert!(replaced_object.is_some());
        match code {
            c::language_object::LanguageObject::SourceFile(src) => {
                assert_eq!(src.code.len(), 0);
            }
            _ => panic!("Expected a SourceFile"),
        }
    }

    #[test]
    fn test_02_replace_child() {
        let decl_id = Uuid::new_v4();
        let primitive_type = c::c_type::CType::Int;
        let decl_identifier = "a";
        let decl = c::language_object::declaration_object::declaration::Declaration {
            id: decl_id,
            primitive_type,
            identifier: decl_identifier.to_string(),
            value: None,
        };

        let param_id = Uuid::new_v4();
        let param_type = c::c_type::CType::Int;
        let param_identifier = "b";
        let param = c::language_object::declaration_object::function_declaration::function_parameter::FunctionParameter {
            id: param_id,
            identifier: param_identifier.to_string(),
            param_type,
        };

        let func_decl_id = Uuid::new_v4();
        let func_decl_return_type = c::c_type::CType::Int;
        let func_decl_identifier = "c";
        let func_decl =
            c::language_object::declaration_object::function_declaration::FunctionDeclaration {
                id: func_decl_id,
                return_type: func_decl_return_type.clone(),
                identifier: func_decl_identifier.to_string(),
                parameter_list: vec![param],
            };

        let src_file_id = Uuid::new_v4();
        let src_file = c::language_object::special_object::source_file::SourceFile {
            id: src_file_id,
            code: vec![
                c::language_object::declaration_object::DeclarationObject::Declaration(decl),
                c::language_object::declaration_object::DeclarationObject::FunctionDeclaration(
                    func_decl,
                ),
            ],
        };

        let replace = c::language_object::LanguageObject::FunctionDeclaration(
            c::language_object::declaration_object::function_declaration::FunctionDeclaration {
                id: func_decl_id,
                return_type: func_decl_return_type,
                identifier: func_decl_identifier.to_string(),
                parameter_list: vec![],
            },
        );

        let mut code = c::language_object::LanguageObject::SourceFile(src_file);
        let replaced_object = replace_object(&mut code, replace);

        assert!(replaced_object.is_some());
        match code {
            c::language_object::LanguageObject::SourceFile(src) => match &src.code[1] {
                c::language_object::declaration_object::DeclarationObject::FunctionDeclaration(
                    func,
                ) => {
                    assert_eq!(func.parameter_list.len(), 0);
                }
                _ => panic!("Expected a CompoundStatement"),
            },
            _ => panic!("Expected a SourceFile"),
        }
    }

    #[test]
    fn test_03_replace_leaf() {
        let decl_id = Uuid::new_v4();
        let primitive_type = c::c_type::CType::Int;
        let decl_identifier = "a";
        let decl = c::language_object::declaration_object::declaration::Declaration {
            id: decl_id,
            primitive_type,
            identifier: decl_identifier.to_string(),
            value: None,
        };

        let param_id = Uuid::new_v4();
        let param_type = c::c_type::CType::Int;
        let param_identifier = "b";
        let param = c::language_object::declaration_object::function_declaration::function_parameter::FunctionParameter {
            id: param_id,
            identifier: param_identifier.to_string(),
            param_type,
        };

        let func_decl_id = Uuid::new_v4();
        let func_decl_return_type = c::c_type::CType::Int;
        let func_decl_identifier = "c";
        let func_decl =
            c::language_object::declaration_object::function_declaration::FunctionDeclaration {
                id: func_decl_id,
                return_type: func_decl_return_type.clone(),
                identifier: func_decl_identifier.to_string(),
                parameter_list: vec![param],
            };

        let src_file_id = Uuid::new_v4();
        let src_file = c::language_object::special_object::source_file::SourceFile {
            id: src_file_id,
            code: vec![
                language_object::declaration_object::DeclarationObject::Declaration(decl),
                language_object::declaration_object::DeclarationObject::FunctionDeclaration(
                    func_decl,
                ),
            ],
        };

        let new_type = c::c_type::CType::Char;
        let replace = c::language_object::LanguageObject::Declaration(
            c::language_object::declaration_object::declaration::Declaration {
                id: decl_id,
                primitive_type: new_type.clone(),
                identifier: decl_identifier.to_string(),
                value: None,
            },
        );

        let mut code = c::language_object::LanguageObject::SourceFile(src_file);
        let replaced_object = replace_object(&mut code, replace);

        assert!(replaced_object.is_some());
        match code {
            c::language_object::LanguageObject::SourceFile(src) => match &src.code[0] {
                c::language_object::declaration_object::DeclarationObject::Declaration(decl) => {
                    assert_eq!(decl.identifier, decl_identifier);
                    assert_eq!(decl.primitive_type, new_type);
                    assert!(decl.value.is_none());
                }
                _ => panic!("Expected a CompoundStatement"),
            },
            _ => panic!("Expected a SourceFile"),
        }
    }

    #[test]
    fn test_04_replace_inexistent() {
        let decl_id = Uuid::new_v4();
        let primitive_type = c::c_type::CType::Int;
        let decl_identifier = "a";
        let decl = c::language_object::declaration_object::declaration::Declaration {
            id: decl_id,
            primitive_type,
            identifier: decl_identifier.to_string(),
            value: None,
        };

        let param_id = Uuid::new_v4();
        let param_type = c::c_type::CType::Int;
        let param_identifier = "b";
        let param = c::language_object::declaration_object::function_declaration::function_parameter::FunctionParameter {
            id: param_id,
            identifier: param_identifier.to_string(),
            param_type,
        };

        let func_decl_id = Uuid::new_v4();
        let func_decl_return_type = c::c_type::CType::Int;
        let func_decl_identifier = "c";
        let func_decl =
            c::language_object::declaration_object::function_declaration::FunctionDeclaration {
                id: func_decl_id,
                return_type: func_decl_return_type.clone(),
                identifier: func_decl_identifier.to_string(),
                parameter_list: vec![param],
            };

        let src_file_id = Uuid::new_v4();
        let src_file = c::language_object::special_object::source_file::SourceFile {
            id: src_file_id,
            code: vec![
                c::language_object::declaration_object::DeclarationObject::Declaration(decl),
                c::language_object::declaration_object::DeclarationObject::FunctionDeclaration(
                    func_decl,
                ),
            ],
        };

        let replace = c::language_object::LanguageObject::Comment(
            c::language_object::special_object::comment::Comment {
                id: Uuid::new_v4(),
                content: "test".to_string(),
            },
        );

        let mut code = c::language_object::LanguageObject::SourceFile(src_file);
        let replaced_object = replace_object(&mut code, replace);

        assert!(replaced_object.is_none());
    }
}
