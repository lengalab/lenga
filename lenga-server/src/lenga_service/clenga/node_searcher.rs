use language::language::c::language_object::{
    self, LanguageObject, declaration_object::DeclarationObject,
    expression_object::ExpressionObject,
    statement_object::compound_statement::compound_statement_object::CompoundStatementObject,
};
use uuid::Uuid;

pub fn find_node(
    file: &language_object::special_object::source_file::SourceFile,
    id: Uuid,
) -> Option<LanguageObject> {
    search_source_file(file, id)
}

fn search_source_file(
    file: &language_object::special_object::source_file::SourceFile,
    id: Uuid,
) -> Option<LanguageObject> {
    if file.id == id {
        return Some(LanguageObject::SourceFile(file.clone()));
    }
    for child in &file.code {
        if let Some(found) = search_declaration_object(child, id) {
            return Some(found);
        }
    }
    None
}

fn search_declaration_object(decl_obj: &DeclarationObject, id: Uuid) -> Option<LanguageObject> {
    match decl_obj {
        DeclarationObject::Declaration(decl) => {
            if decl.id == id {
                return Some(LanguageObject::Declaration(decl.clone()));
            }
            if let Some(val) = &decl.value {
                return search_expression_object(val, id);
            }
            None
        }
        DeclarationObject::FunctionDeclaration(func_decl) => {
            if func_decl.id == id {
                return Some(LanguageObject::FunctionDeclaration(func_decl.clone()));
            }
            None
        }
        DeclarationObject::FunctionDefinition(func_def) => {
            if func_def.id == id {
                return Some(LanguageObject::FunctionDefinition(func_def.clone()));
            }
            search_compound_statement(&func_def.compound_statement, id)
        }
        DeclarationObject::PreprocInclude(preproc) => {
            if preproc.id == id {
                return Some(LanguageObject::PreprocInclude(preproc.clone()));
            }
            None
        }
        DeclarationObject::Comment(comment) => {
            if comment.id == id {
                return Some(LanguageObject::Comment(comment.clone()));
            }
            None
        }
        DeclarationObject::Unknown(unknown) => {
            if unknown.id == id {
                return Some(LanguageObject::Unknown(unknown.clone()));
            }
            None
        }
    }
}

fn search_expression_object(expr_obj: &ExpressionObject, id: Uuid) -> Option<LanguageObject> {
    match expr_obj {
        ExpressionObject::AssignmentExpression(expr) => {
            if expr.id == id {
                return Some(LanguageObject::AssignmentExpression(expr.clone()));
            }
            search_expression_object(&expr.value, id)
        }
        ExpressionObject::BinaryExpression(expr) => {
            if expr.id == id {
                return Some(LanguageObject::BinaryExpression(expr.clone()));
            }
            if let Some(found) = search_expression_object(&expr.left, id) {
                return Some(found);
            }
            search_expression_object(&expr.right, id)
        }
        ExpressionObject::CallExpression(expr) => {
            if expr.id == id {
                return Some(LanguageObject::CallExpression(expr.clone()));
            }
            for arg in &expr.argument_list {
                if let Some(found) = search_expression_object(arg, id) {
                    return Some(found);
                }
            }
            None
        }
        ExpressionObject::NumberLiteral(lit) => {
            if lit.id == id {
                return Some(LanguageObject::NumberLiteral(lit.clone()));
            }
            None
        }
        ExpressionObject::Reference(ref_) => {
            if ref_.id == id {
                return Some(LanguageObject::Reference(ref_.clone()));
            }
            None
        }
        ExpressionObject::StringLiteral(lit) => {
            if lit.id == id {
                return Some(LanguageObject::StringLiteral(lit.clone()));
            }
            None
        }
        ExpressionObject::Unknown(unknown) => {
            if unknown.id == id {
                return Some(LanguageObject::Unknown(unknown.clone()));
            }
            None
        }
    }
}

fn search_compound_statement(
    comp_stmt: &language_object::statement_object::compound_statement::CompoundStatement,
    id: Uuid,
) -> Option<LanguageObject> {
    if comp_stmt.id == id {
        return Some(LanguageObject::CompoundStatement(comp_stmt.clone()));
    }
    for child in &comp_stmt.code_block {
        if let Some(found) = search_compound_statement_object(child, id) {
            return Some(found);
        }
    }
    None
}

fn search_compound_statement_object(
    obj: &CompoundStatementObject,
    id: Uuid,
) -> Option<LanguageObject> {
    match obj {
        CompoundStatementObject::Declaration(decl) => {
            if decl.id == id {
                return Some(LanguageObject::Declaration(decl.clone()));
            }
            if let Some(val) = &decl.value {
                return search_expression_object(val, id);
            }
            None
        }
        CompoundStatementObject::AssignmentExpression(expr) => {
            if expr.id == id {
                return Some(LanguageObject::AssignmentExpression(expr.clone()));
            }
            search_expression_object(&expr.value, id)
        }
        CompoundStatementObject::BinaryExpression(expr) => {
            if expr.id == id {
                return Some(LanguageObject::BinaryExpression(expr.clone()));
            }
            if let Some(found) = search_expression_object(&expr.left, id) {
                return Some(found);
            }
            search_expression_object(&expr.right, id)
        }
        CompoundStatementObject::CallExpression(expr) => {
            if expr.id == id {
                return Some(LanguageObject::CallExpression(expr.clone()));
            }
            for arg in &expr.argument_list {
                if let Some(found) = search_expression_object(arg, id) {
                    return Some(found);
                }
            }
            None
        }
        CompoundStatementObject::NumberLiteral(lit) => {
            if lit.id == id {
                return Some(LanguageObject::NumberLiteral(lit.clone()));
            }
            None
        }
        CompoundStatementObject::Reference(ref_) => {
            if ref_.id == id {
                return Some(LanguageObject::Reference(ref_.clone()));
            }
            None
        }
        CompoundStatementObject::StringLiteral(lit) => {
            if lit.id == id {
                return Some(LanguageObject::StringLiteral(lit.clone()));
            }
            None
        }
        CompoundStatementObject::CompoundStatement(stmt) => search_compound_statement(stmt, id),
        CompoundStatementObject::IfStatement(stmt) => search_if_statement(stmt, id),
        CompoundStatementObject::ReturnStatement(stmt) => search_return_statement(stmt, id),
        CompoundStatementObject::Comment(comment) => {
            if comment.id == id {
                return Some(LanguageObject::Comment(comment.clone()));
            }
            None
        }
        CompoundStatementObject::Unknown(unknown) => {
            if unknown.id == id {
                return Some(LanguageObject::Unknown(unknown.clone()));
            }
            None
        }
    }
}

#[allow(dead_code)]
fn search_statement_object(
    stmt_obj: &language_object::statement_object::StatementObject,
    id: Uuid,
) -> Option<LanguageObject> {
    match stmt_obj {
        language_object::statement_object::StatementObject::CompoundStatement(stmt) => {
            search_compound_statement(stmt, id)
        }
        language_object::statement_object::StatementObject::IfStatement(stmt) => {
            search_if_statement(stmt, id)
        }
        language_object::statement_object::StatementObject::ReturnStatement(stmt) => {
            search_return_statement(stmt, id)
        }
        language_object::statement_object::StatementObject::Unknown(unknown) => {
            if unknown.id == id {
                return Some(LanguageObject::Unknown(unknown.clone()));
            }
            None
        }
    }
}

fn search_if_statement(
    stmt: &language_object::statement_object::if_statement::IfStatement,
    id: Uuid,
) -> Option<LanguageObject> {
    if stmt.id == id {
        return Some(LanguageObject::IfStatement(stmt.clone()));
    }
    if let Some(found) = search_expression_object(&stmt.condition, id) {
        return Some(found);
    }
    if let Some(found) = search_compound_statement_object(&stmt.body, id) {
        return Some(found);
    }
    if let Some(else_statement) = &stmt.else_statement {
        match else_statement {
            language_object::statement_object::if_statement::ElseStatement::ElseIf(
                if_statement,
            ) => {
                if let Some(found) = search_if_statement(if_statement, id) {
                    return Some(found);
                }
            }
            language_object::statement_object::if_statement::ElseStatement::ElseClause(
                else_clause,
            ) => {
                if let Some(found) = search_else_clause(else_clause, id) {
                    return Some(found);
                }
            }
        }
    }
    None
}

fn search_else_clause(
    else_clause: &language_object::statement_object::if_statement::else_clause::ElseClause,
    id: Uuid,
) -> Option<LanguageObject> {
    if else_clause.id == id {
        return Some(LanguageObject::ElseClause(else_clause.clone()));
    }
    search_compound_statement_object(&else_clause.body, id)
}

fn search_return_statement(
    stmt: &language_object::statement_object::return_statement::ReturnStatement,
    id: Uuid,
) -> Option<LanguageObject> {
    if stmt.id == id {
        return Some(LanguageObject::ReturnStatement(stmt.clone()));
    }
    match &stmt.value {
        None => None,
        Some(value) => search_expression_object(value, id),
    }
}
