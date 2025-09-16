pub mod node_writer;
pub mod text_writer;
pub mod writer_error;
use writer_error::WriterError;

use crate::language::c::language_object::{
    LanguageObject, assignment_expression::AssignmentExpression,
    binary_expression::BinaryExpression, call_expression::CallExpression, comment::Comment,
    compound_statement::CompoundStatement, declaration::Declaration, else_clause::ElseClause,
    expression_statement::ExpressionStatement, function_declaration::FunctionDeclaration,
    function_definition::FunctionDefinition, if_statement::IfStatement,
    number_literal::NumberLiteral, preproc_include::PreprocInclude, reference::Reference,
    return_statement::ReturnStatement, source_file::SourceFile, string_literal::StringLiteral,
};

pub trait Writer {
    fn write_file(&mut self, src: &SourceFile) -> Result<(), WriterError>;
}

pub trait Cursor {
    fn write_source_file(&mut self, src_file: &SourceFile) -> Result<(), WriterError>;

    fn write_assignment_expression(
        &mut self,
        assignment_expression: &AssignmentExpression,
    ) -> Result<(), WriterError>;

    fn write_binary_expression(
        &mut self,
        binary_expression: &BinaryExpression,
    ) -> Result<(), WriterError>;

    fn write_call_expression(
        &mut self,
        call_expression: &CallExpression,
    ) -> Result<(), WriterError>;

    fn write_comment(&mut self, comment: &Comment) -> Result<(), WriterError>;

    fn write_declaration(&mut self, declaration: &Declaration) -> Result<(), WriterError>;

    fn write_else_clause(&mut self, else_clause: &ElseClause) -> Result<(), WriterError>;

    fn write_expression_statement(
        &mut self,
        expression_statement: &ExpressionStatement,
    ) -> Result<(), WriterError>;

    fn write_function_declaration(
        &mut self,
        function_declaration: &FunctionDeclaration,
    ) -> Result<(), WriterError>;

    fn write_function_definition(
        &mut self,
        function_definition: &FunctionDefinition,
    ) -> Result<(), WriterError>;

    fn write_if_statement(&mut self, if_statement: &IfStatement) -> Result<(), WriterError>;

    fn write_number_literal(&mut self, number_literal: &NumberLiteral) -> Result<(), WriterError>;

    fn write_preproc_include(
        &mut self,
        preproc_include: &PreprocInclude,
    ) -> Result<(), WriterError>;

    fn write_reference(&mut self, reference: &Reference) -> Result<(), WriterError>;

    fn write_return_statement(
        &mut self,
        return_statement: &ReturnStatement,
    ) -> Result<(), WriterError>;

    fn write_string_literal(&mut self, string_literal: &StringLiteral) -> Result<(), WriterError>;

    fn write_compound_statement(
        &mut self,
        compound_statement: &CompoundStatement,
    ) -> Result<(), WriterError>;
}
