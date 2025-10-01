use language::language::c::language_object::{self, LanguageObject};

fn replace_node(
    node: &mut LanguageObject,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    match node {
        LanguageObject::SourceFile(obj) => replace_source_file(obj, new_node),
        LanguageObject::AssignmentExpression(obj) => replace_assignment_expression(obj, new_node),
        LanguageObject::BinaryExpression(obj) => replace_binary_expression(obj, new_node),
        LanguageObject::CallExpression(obj) => replace_call_expression(obj, new_node),
        LanguageObject::Comment(obj) => replace_comment(obj, new_node),
        LanguageObject::Declaration(obj) => replace_declaration(obj, new_node),
        LanguageObject::ElseClause(obj) => replace_else_clause(obj, new_node),
        LanguageObject::ExpressionStatement(obj) => replace_expression_statement(obj, new_node),
        LanguageObject::FunctionDeclaration(obj) => replace_function_declaration(obj, new_node),
        LanguageObject::FunctionDefinition(obj) => replace_function_definition(obj, new_node),
        LanguageObject::FunctionParameter(obj) => replace_function_parameter(obj, new_node),
        LanguageObject::IfStatement(obj) => replace_if_statement(obj, new_node),
        LanguageObject::NumberLiteral(obj) => replace_number_literal(obj, new_node),
        LanguageObject::PreprocInclude(obj) => replace_preproc_include(obj, new_node),
        LanguageObject::Reference(obj) => replace_reference(obj, new_node),
        LanguageObject::ReturnStatement(obj) => replace_return_statement(obj, new_node),
        LanguageObject::StringLiteral(obj) => replace_string_literal(obj, new_node),
        LanguageObject::CompoundStatement(obj) => replace_compound_statement(obj, new_node),
    }
}

fn replace_source_file(
    file: &mut language_object::source_file::SourceFile,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if file.id == new_node.id() {
        if let LanguageObject::SourceFile(new_file) = new_node {
            return Some(LanguageObject::SourceFile(std::mem::replace(file, new_file)));
        }
    } else {
        for child in &mut file.code {
            if let Some(found) = replace_node(child, new_node.clone()) {
                return Some(found);
            }
        }
    }
    None
}

fn replace_assignment_expression(
    expr: &mut language_object::assignment_expression::AssignmentExpression,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if expr.id == new_node.id() {
        if let LanguageObject::AssignmentExpression(new_expr) = new_node {
            return Some(LanguageObject::AssignmentExpression(std::mem::replace(expr, new_expr)));
        }
    } else {
        if let Some(found) = replace_node(&mut expr.value, new_node) {
            return Some(found);
        }
    }
    None
}

fn replace_binary_expression(
    expr: &mut language_object::binary_expression::BinaryExpression,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if expr.id == new_node.id() {
        if let LanguageObject::BinaryExpression(new_expr) = new_node {
            return Some(LanguageObject::BinaryExpression(std::mem::replace(expr, new_expr)));
        }
    } else {
        if let Some(found) = replace_node(&mut expr.left, new_node.clone()) {
            return Some(found);
        }
        if let Some(found) = replace_node(&mut expr.right, new_node) {
            return Some(found);
        }
    }
    None
}

fn replace_call_expression(
    call: &mut language_object::call_expression::CallExpression,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if call.id == new_node.id() {
        if let LanguageObject::CallExpression(new_call) = new_node {
            return Some(LanguageObject::CallExpression(std::mem::replace(call, new_call)));
        }
    } else {
        for arg in &mut call.argument_list {
            if let Some(found) = replace_node(arg, new_node.clone()) {
                return Some(found);
            }
        }
    }
    None
}

fn replace_comment(
    comment: &mut language_object::comment::Comment,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if comment.id == new_node.id() {
        if let LanguageObject::Comment(new_comment) = new_node {
            return Some(LanguageObject::Comment(std::mem::replace(comment, new_comment)));
        }
    }
    None
}

fn replace_declaration(
    decl: &mut language_object::declaration::Declaration,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if decl.id == new_node.id() {
        if let LanguageObject::Declaration(new_decl) = new_node {
            return Some(LanguageObject::Declaration(std::mem::replace(decl, new_decl)));
        }
    } else {
        if let Some(val) = &mut decl.value {
            if let Some(found) = replace_node(val, new_node) {
                return Some(found);
            }
        }
    }
    None
}

fn replace_else_clause(
    else_clause: &mut language_object::else_clause::ElseClause,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if else_clause.id == new_node.id() {
        if let LanguageObject::ElseClause(new_else) = new_node {
            return Some(LanguageObject::ElseClause(std::mem::replace(else_clause, new_else)));
        }
    } else {
        if let Some(found) = replace_compound_statement(&mut else_clause.compound_statement, new_node) {
            return Some(found);
        }
    }
    None
}

fn replace_expression_statement(
    stmt: &mut language_object::expression_statement::ExpressionStatement,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if stmt.id == new_node.id() {
        if let LanguageObject::ExpressionStatement(new_stmt) = new_node {
            return Some(LanguageObject::ExpressionStatement(std::mem::replace(stmt, new_stmt)));
        }
    } else {
        for arg in &mut stmt.argument_list {
            if let Some(found) = replace_node(arg, new_node.clone()) {
                return Some(found);
            }
        }
    }
    None
}

fn replace_function_declaration(
    decl: &mut language_object::function_declaration::FunctionDeclaration,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if decl.id == new_node.id() {
        if let LanguageObject::FunctionDeclaration(new_decl) = new_node {
            return Some(LanguageObject::FunctionDeclaration(std::mem::replace(decl, new_decl)));
        }
    } else {
        for param in &mut decl.parameter_list {
            if let Some(found) = replace_function_parameter(param, new_node.clone()) {
                return Some(found);
            }
        }
    }
    None
}

fn replace_function_definition(
    def: &mut language_object::function_definition::FunctionDefinition,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if def.id == new_node.id() {
        if let LanguageObject::FunctionDefinition(new_def) = new_node {
            return Some(LanguageObject::FunctionDefinition(std::mem::replace(def, new_def)));
        }
    } else {
        for param in &mut def.parameter_list {
            if let Some(found) = replace_function_parameter(param, new_node.clone()) {
                return Some(found);
            }
        }
        if let Some(found) = replace_compound_statement(&mut def.compound_statement, new_node) {
            return Some(found);
        }
    }
    None
}

fn replace_function_parameter(
    param: &mut language_object::function_parameter::FunctionParameter,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if param.id == new_node.id() {
        if let LanguageObject::FunctionParameter(new_param) = new_node {
            return Some(LanguageObject::FunctionParameter(std::mem::replace(param, new_param)));
        }
    }
    None
}

fn replace_if_statement(
    stmt: &mut language_object::if_statement::IfStatement,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if stmt.id == new_node.id() {
        if let LanguageObject::IfStatement(new_stmt) = new_node {
            return Some(LanguageObject::IfStatement(std::mem::replace(stmt, new_stmt)));
        }
    } else {
        if let Some(found) = replace_node(&mut stmt.condition, new_node.clone()) {
            return Some(found);
        }
        if let Some(found) = replace_compound_statement(&mut stmt.compound_statement, new_node.clone()) {
            return Some(found);
        }
        if let Some(else_clause) = &mut stmt.else_clause {
            if let Some(found) = replace_else_clause(else_clause, new_node) {
                return Some(found);
            }
        }
    }
    None
}

fn replace_number_literal(
    lit: &mut language_object::number_literal::NumberLiteral,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if lit.id == new_node.id() {
        if let LanguageObject::NumberLiteral(new_lit) = new_node {
            return Some(LanguageObject::NumberLiteral(std::mem::replace(lit, new_lit)));
        }
    }
    None
}

fn replace_preproc_include(
    inc: &mut language_object::preproc_include::PreprocInclude,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if inc.id == new_node.id() {
        if let LanguageObject::PreprocInclude(new_inc) = new_node {
            return Some(LanguageObject::PreprocInclude(std::mem::replace(inc, new_inc)));
        }
    }
    None
}

fn replace_reference(
    r: &mut language_object::reference::Reference,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if r.id == new_node.id() {
        if let LanguageObject::Reference(new_r) = new_node {
            return Some(LanguageObject::Reference(std::mem::replace(r, new_r)));
        }
    }
    None
}

fn replace_return_statement(
    stmt: &mut language_object::return_statement::ReturnStatement,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if stmt.id == new_node.id() {
        if let LanguageObject::ReturnStatement(new_stmt) = new_node {
            return Some(LanguageObject::ReturnStatement(std::mem::replace(stmt, new_stmt)));
        }
    } else {
        if let Some(found) = replace_node(&mut stmt.value, new_node) {
            return Some(found);
        }
    }
    None
}

fn replace_string_literal(
    lit: &mut language_object::string_literal::StringLiteral,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if lit.id == new_node.id() {
        if let LanguageObject::StringLiteral(new_lit) = new_node {
            return Some(LanguageObject::StringLiteral(std::mem::replace(lit, new_lit)));
        }
    }
    None
}

fn replace_compound_statement(
    comp_stmt: &mut language_object::compound_statement::CompoundStatement,
    new_node: LanguageObject,
) -> Option<LanguageObject> {
    if comp_stmt.id == new_node.id() {
        if let LanguageObject::CompoundStatement(new_comp_stmt) = new_node {
            return Some(LanguageObject::CompoundStatement(std::mem::replace(comp_stmt, new_comp_stmt)));
        }
    } else {
        for child in &mut comp_stmt.code_block {
            if let Some(found) = replace_node(child, new_node.clone()) {
                return Some(found);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;
    use uuid::Uuid;
    use language::language::c;

    #[test]
    fn test_01_replace_root() {
        let decl_id = Uuid::new_v4();
        let primitive_type = c::object_types::c_type::CType::Int;
        let decl_identifier = "a";
        let decl = c::language_object::declaration::Declaration{
            id: decl_id,
            primitive_type,
            identifier: decl_identifier.to_string(),
            value: None,
        };

        let decl_in_id = Uuid::new_v4();
        let primitive_type_in = c::object_types::c_type::CType::Int;
        let decl_in_identifier = "b";
        let decl_in = c::language_object::declaration::Declaration{
            id: decl_in_id,
            primitive_type: primitive_type_in,
            identifier: decl_in_identifier.to_string(),
            value: None,
        };

        let comp_stmt_id = Uuid::new_v4();
        let comp_stmt = c::language_object::compound_statement::CompoundStatement{
            id: comp_stmt_id,
            code_block: vec![
                c::language_object::LanguageObject::Declaration(decl_in),
                ],
        };

        let src_file_id = Uuid::new_v4();
        let src_file = c::language_object::source_file::SourceFile{
            id: src_file_id,
            code: vec![
                c::language_object::LanguageObject::Declaration(decl),
                c::language_object::LanguageObject::CompoundStatement(comp_stmt),
            ],
        };

        let replace = c::language_object::LanguageObject::SourceFile(
            c::language_object::source_file::SourceFile{
                id: src_file_id,
                code: vec![],
            }
        );

        let mut code = c::language_object::LanguageObject::SourceFile(src_file);
        let replaced_node = replace_node(&mut code, replace);

        assert!(replaced_node.is_some());
        match code {
            c::language_object::LanguageObject::SourceFile(src) => {
                assert_eq!(src.code.len(), 0);
            },
            _ => panic!("Expected a SourceFile"),
        }
    }

    #[test]
    fn test_02_replace_child() {
        let decl_id = Uuid::new_v4();
        let primitive_type = c::object_types::c_type::CType::Int;
        let decl_identifier = "a";
        let decl = c::language_object::declaration::Declaration{
            id: decl_id,
            primitive_type,
            identifier: decl_identifier.to_string(),
            value: None,
        };

        let decl_in_id = Uuid::new_v4();
        let primitive_type_in = c::object_types::c_type::CType::Int;
        let decl_in_identifier = "b";
        let decl_in = c::language_object::declaration::Declaration{
            id: decl_in_id,
            primitive_type: primitive_type_in,
            identifier: decl_in_identifier.to_string(),
            value: None,
        };

        let comp_stmt_id = Uuid::new_v4();
        let comp_stmt = c::language_object::compound_statement::CompoundStatement{
            id: comp_stmt_id,
            code_block: vec![
                c::language_object::LanguageObject::Declaration(decl_in),
                ],
        };

        let src_file_id = Uuid::new_v4();
        let src_file = c::language_object::source_file::SourceFile{
            id: src_file_id,
            code: vec![
                c::language_object::LanguageObject::Declaration(decl),
                c::language_object::LanguageObject::CompoundStatement(comp_stmt),
            ],
        };

        let replace = c::language_object::LanguageObject::CompoundStatement(
            c::language_object::compound_statement::CompoundStatement{
                id: comp_stmt_id,
                code_block: vec![],
            }
        );

        let mut code = c::language_object::LanguageObject::SourceFile(src_file);
        let replaced_node = replace_node(&mut code, replace);

        assert!(replaced_node.is_some());
        match code {
            c::language_object::LanguageObject::SourceFile(src) => {
                match &src.code[1] {
                    c::language_object::LanguageObject::CompoundStatement(comp) => {
                        assert_eq!(comp.code_block.len(), 0);
                    },
                    _ => panic!("Expected a CompoundStatement"),
                }
            },
            _ => panic!("Expected a SourceFile"),
        }
    }

    #[test]
    fn test_03_replace_leaf() {
        let decl_id = Uuid::new_v4();
        let primitive_type = c::object_types::c_type::CType::Int;
        let decl_identifier = "a";
        let decl = c::language_object::declaration::Declaration{
            id: decl_id,
            primitive_type,
            identifier: decl_identifier.to_string(),
            value: None,
        };

        let decl_in_id = Uuid::new_v4();
        let primitive_type_in = c::object_types::c_type::CType::Int;
        let decl_in_identifier = "b";
        let decl_in = c::language_object::declaration::Declaration{
            id: decl_in_id,
            primitive_type: primitive_type_in,
            identifier: decl_in_identifier.to_string(),
            value: None,
        };

        let comp_stmt_id = Uuid::new_v4();
        let comp_stmt = c::language_object::compound_statement::CompoundStatement{
            id: comp_stmt_id,
            code_block: vec![
                c::language_object::LanguageObject::Declaration(decl_in),
                ],
        };

        let src_file_id = Uuid::new_v4();
        let src_file = c::language_object::source_file::SourceFile{
            id: src_file_id,
            code: vec![
                c::language_object::LanguageObject::Declaration(decl),
                c::language_object::LanguageObject::CompoundStatement(comp_stmt),
            ],
        };

        let new_type = c::object_types::c_type::CType::Char;
        let replace = c::language_object::LanguageObject::Declaration(
            c::language_object::declaration::Declaration{
                id: decl_id,
                primitive_type: new_type.clone(),
                identifier: decl_identifier.to_string(),
                value: None,
            }
        );

        let mut code = c::language_object::LanguageObject::SourceFile(src_file);
        let replaced_node = replace_node(&mut code, replace);

        assert!(replaced_node.is_some());
        match code {
            c::language_object::LanguageObject::SourceFile(src) => {
                match &src.code[0] {
                    c::language_object::LanguageObject::Declaration(decl) => {
                        assert_eq!(decl.identifier, decl_identifier);
                        assert_eq!(decl.primitive_type, new_type);
                        assert!(decl.value.is_none());
                    },
                    _ => panic!("Expected a CompoundStatement"),
                }
            },
            _ => panic!("Expected a SourceFile"),
        }
    }

    #[test]
    fn test_04_replace_inexistent() {
        let decl_id = Uuid::new_v4();
        let primitive_type = c::object_types::c_type::CType::Int;
        let decl_identifier = "a";
        let decl = c::language_object::declaration::Declaration{
            id: decl_id,
            primitive_type,
            identifier: decl_identifier.to_string(),
            value: None,
        };

        let decl_in_id = Uuid::new_v4();
        let primitive_type_in = c::object_types::c_type::CType::Int;
        let decl_in_identifier = "b";
        let decl_in = c::language_object::declaration::Declaration{
            id: decl_in_id,
            primitive_type: primitive_type_in,
            identifier: decl_in_identifier.to_string(),
            value: None,
        };

        let comp_stmt_id = Uuid::new_v4();
        let comp_stmt = c::language_object::compound_statement::CompoundStatement{
            id: comp_stmt_id,
            code_block: vec![
                c::language_object::LanguageObject::Declaration(decl_in),
                ],
        };

        let src_file_id = Uuid::new_v4();
        let src_file = c::language_object::source_file::SourceFile{
            id: src_file_id,
            code: vec![
                c::language_object::LanguageObject::Declaration(decl),
                c::language_object::LanguageObject::CompoundStatement(comp_stmt),
            ],
        };

        let replace = c::language_object::LanguageObject::Comment(
            c::language_object::comment::Comment{
                id: Uuid::new_v4(),
                content: "test".to_string(),
            }
        );

        let mut code = c::language_object::LanguageObject::SourceFile(src_file);
        let replaced_node = replace_node(&mut code, replace);

        assert!(replaced_node.is_none());
    }
}