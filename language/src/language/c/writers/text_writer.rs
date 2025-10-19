pub mod style;

use super::{Writer, writer_error::WriterError};
use crate::language::c::language_object::{
    declaration_object::{
        declaration::Declaration,
        function_declaration::{FunctionDeclaration, function_parameter::FunctionParameter},
        function_definition::FunctionDefinition,
        preproc_include::PreprocInclude,
    },
    expression_object::{
        assignment_expression::AssignmentExpression, binary_expression::BinaryExpression,
        call_expression::CallExpression, number_literal::NumberLiteral, reference::Reference,
        string_literal::StringLiteral,
    },
    special_object::{comment::Comment, source_file::SourceFile, unknown::Unknown},
    statement_object::{
        compound_statement::CompoundStatement,
        if_statement::{IfStatement, else_clause::ElseClause},
        return_statement::ReturnStatement,
    },
};
use crate::language::c::writers::Cursor;

pub struct TextWriter<'a> {
    indent_level: usize,
    writer: Box<&'a mut dyn std::io::Write>,
    delimitator_stack: Vec<Delimitator>,
    style: style::Style,
    new_line: bool,
    semicolon_on_next: bool,
}

impl<'a> TextWriter<'a> {
    pub fn new(writer: Box<&'a mut dyn std::io::Write>, style: style::Style) -> Self {
        TextWriter {
            indent_level: 0,
            writer,
            delimitator_stack: Vec::new(),
            new_line: true,
            style,
            semicolon_on_next: true,
        }
    }

    fn indent(&mut self) {
        self.indent_level += 1;
    }

    fn dedent(&mut self) {
        self.indent_level -= 1;
    }

    fn pad(&mut self) -> Result<(), WriterError> {
        self.write(&" ".repeat(self.indent_level * self.style.indent_size))?;
        Ok(())
    }

    pub fn open_block(&mut self, delimitator: Delimitator) -> Result<(), WriterError> {
        self.write(&format!("{}", delimitator.open()))?;
        self.delimitator_stack.push(delimitator);
        self.indent();
        Ok(())
    }

    pub fn close_block(&mut self) -> Result<(), WriterError> {
        self.dedent();
        let closing_delimitator = self.delimitator_stack.pop().unwrap();
        if self.new_line {
            self.pad()?;
        }
        self.write(&format!("{}", closing_delimitator.close()))?;
        Ok(())
    }

    fn finish_line(&mut self, text: &str) -> Result<(), WriterError> {
        self.write(&format!("{}\n", text))?;
        self.new_line = true;
        Ok(())
    }

    fn writeln(&mut self, text: &str) -> Result<(), WriterError> {
        self.pad()?;
        self.finish_line(text)?;
        Ok(())
    }

    fn write_statement<C>(&mut self, mut content: C) -> Result<(), WriterError>
    where
        C: FnMut(&mut Self) -> Result<(), WriterError>,
    {
        self.pad()?;
        self.semicolon_on_next = true;
        content(self)?;
        if self.semicolon_on_next {
            self.finish_line(";")?;
        } else {
            self.finish_line("")?;
        }

        Ok(())
    }

    fn write(&mut self, text: &str) -> Result<(), WriterError> {
        self.writer.write(text.as_bytes())?;
        self.new_line = false;
        Ok(())
    }

    fn write_paren_block<I, F>(&mut self, args: I) -> Result<(), WriterError>
    where
        I: ExactSizeIterator<Item = F>,
        F: Fn(&mut Self) -> Result<(), WriterError>,
    {
        let len = args.len();
        self.open_block(Delimitator::Paren)?;
        if len == 0 {
            return self.close_block();
        }
        let mut arg_count_in_line = 0;
        for (i, arg) in args.enumerate() {
            if i > 0 && i < len {
                self.write(",")?;
            }
            if arg_count_in_line >= self.style.fn_call_max_arg_per_line {
                self.finish_line("")?;
                self.pad()?;
                arg_count_in_line = 0;
            } else if i > 0 && i < len {
                self.write(" ")?;
            }
            arg(self)?;
            arg_count_in_line += 1;
        }

        if self.style.fn_call_max_arg_per_line == 0 {
            self.finish_line("")?;
        }
        self.close_block()?;
        Ok(())
    }

    fn skip_semicolon(&mut self) {
        self.semicolon_on_next = false;
    }
}

impl<'a> Writer for TextWriter<'a> {
    fn write_file(&mut self, src_file: &SourceFile) -> Result<(), WriterError> {
        self.write_source_file(src_file)?;
        Ok(())
    }
}

impl<'a> Cursor for TextWriter<'a> {
    fn write_source_file(&mut self, src_file: &SourceFile) -> Result<(), WriterError> {
        for object in &src_file.code {
            object.write(self)?;
            self.finish_line("")?;
        }
        Ok(())
    }

    fn write_unknown(&mut self, unknown: &Unknown) -> Result<(), WriterError> {
        self.writeln(&unknown.content)
    }

    fn write_assignment_expression(
        &mut self,
        assignment_expression: &AssignmentExpression,
    ) -> Result<(), WriterError> {
        self.write(&format!("{} = ", assignment_expression.identifier,))?;
        assignment_expression.value.write(self)?;
        Ok(())
    }

    fn write_binary_expression(
        &mut self,
        binary_expression: &BinaryExpression,
    ) -> Result<(), WriterError> {
        binary_expression.left.write(self)?;
        self.write(&format!(" {} ", binary_expression.operator))?;
        binary_expression.right.write(self)?;
        Ok(())
    }

    fn write_call_expression(
        &mut self,
        call_expression: &CallExpression,
    ) -> Result<(), WriterError> {
        self.write(&format!("{}", call_expression.identifier))?;
        let argf = call_expression
            .argument_list
            .iter()
            .map(|a| |w: &mut Self| a.write(w));
        self.write_paren_block(argf)?;
        Ok(())
    }

    fn write_comment(&mut self, comment: &Comment) -> Result<(), WriterError> {
        self.write(&comment.content)
    }

    fn write_declaration(&mut self, declaration: &Declaration) -> Result<(), WriterError> {
        self.write(&format!(
            "{} {}",
            declaration.primitive_type.as_str(),
            declaration.identifier,
        ))?;
        if let Some(value) = &declaration.value {
            self.write(" = ")?;
            value.write(self)?;
        }
        Ok(())
    }

    fn write_else_clause(&mut self, else_clause: &ElseClause) -> Result<(), WriterError> {
        self.write(" else")?;
        if let Some(condition) = &else_clause.condition {
            self.open_block(Delimitator::Paren)?;
            condition.write(self)?;
            self.close_block()?;
        }
        self.write(" ")?;
        else_clause.compound_statement.write(self)?;
        Ok(())
    }

    fn write_function_declaration(
        &mut self,
        function_declaration: &FunctionDeclaration,
    ) -> Result<(), WriterError> {
        self.write(&format!("{}", function_declaration.return_type.as_str()))?;

        if self.style.function_type_always_above {
            self.write(&format!("\n{}", function_declaration.identifier))?;
        } else {
            self.write(&format!(" {}", function_declaration.identifier))?;
        }

        let argf = function_declaration.parameter_list.iter().map(
            |FunctionParameter {
                 id,
                 identifier,
                 param_type,
             }| {
                |w: &mut Self| {
                    w.write(&format!(
                        "{} {}",
                        param_type.as_str(),
                        identifier.to_string()
                    ))
                }
            },
        );
        self.write_paren_block(argf)?;
        self.finish_line(";")?;
        Ok(())
    }

    fn write_function_definition(
        &mut self,
        function_definition: &FunctionDefinition,
    ) -> Result<(), WriterError> {
        self.write(&format!("{}", function_definition.return_type.as_str()))?;

        if self.style.function_type_always_above {
            self.write(&format!("\n{}", function_definition.identifier))?;
        } else {
            self.write(&format!(" {}", function_definition.identifier))?;
        }

        let argf = function_definition.parameter_list.iter().map(
            |FunctionParameter {
                 id,
                 identifier,
                 param_type,
             }| {
                |w: &mut Self| {
                    w.write(&format!(
                        "{} {}",
                        param_type.as_str(),
                        identifier.to_string()
                    ))
                }
            },
        );
        self.write_paren_block(argf)?;
        if self.style.block_always_below {
            self.finish_line("")?;
        } else {
            self.write(" ")?;
        }

        self.write_compound_statement(&function_definition.compound_statement)?;
        self.finish_line("")?;
        Ok(())
    }

    fn write_if_statement(&mut self, if_statement: &IfStatement) -> Result<(), WriterError> {
        self.write("if ")?;

        self.open_block(Delimitator::Paren)?;
        if_statement.condition.write(self)?;
        self.close_block()?;

        self.write(&format!(" "))?;

        if_statement.compound_statement.write(self)?;
        if let Some(else_clause) = &if_statement.else_clause {
            else_clause.write(self)?
        }
        self.finish_line("")?;
        Ok(())
    }

    fn write_number_literal(&mut self, number_literal: &NumberLiteral) -> Result<(), WriterError> {
        self.write(&number_literal.value.to_string())?;
        Ok(())
    }

    fn write_preproc_include(
        &mut self,
        preproc_include: &PreprocInclude,
    ) -> Result<(), WriterError> {
        self.writeln(&format!("#include {}", preproc_include.content))?;
        Ok(())
    }

    fn write_reference(&mut self, reference: &Reference) -> Result<(), WriterError> {
        self.write(&reference.identifier)?;
        Ok(())
    }

    fn write_return_statement(
        &mut self,
        return_statement: &ReturnStatement,
    ) -> Result<(), WriterError> {
        self.write("return ")?;
        return_statement.value.write(self)?;
        Ok(())
    }

    fn write_string_literal(&mut self, string_literal: &StringLiteral) -> Result<(), WriterError> {
        self.write(&format!("\"{}\"", string_literal.value))?;
        Ok(())
    }

    fn write_compound_statement(
        &mut self,
        compound_statement: &CompoundStatement,
    ) -> Result<(), WriterError> {
        self.open_block(Delimitator::CurlyBrace)?;
        self.finish_line("")?;
        for object in compound_statement.code_block.iter() {
            self.write_statement(|w: &mut Self| {
                object.write(w)?;
                Ok(())
            })?;
        }
        self.close_block()?;
        self.skip_semicolon();
        Ok(())
    }
}

pub enum Delimitator {
    Paren,
    SquareBracket,
    CurlyBrace,
}

impl Delimitator {
    fn open(&self) -> &str {
        match self {
            Delimitator::Paren => "(",
            Delimitator::SquareBracket => "[",
            Delimitator::CurlyBrace => "{",
        }
    }

    fn close(&self) -> &str {
        match self {
            Delimitator::Paren => ")",
            Delimitator::SquareBracket => "]",
            Delimitator::CurlyBrace => "}",
        }
    }
}
