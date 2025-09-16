use crate::language::c::{
    language_object::{
        LanguageObject as CLanguageObject, compound_statement::CompoundStatement,
        else_clause::ElseClause,
    },
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Box<CLanguageObject>,
    pub compound_statement: CompoundStatement,
    pub else_clause: Option<ElseClause>,
}

impl IfStatement {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_if_statement(&self)
    }
}

impl PartialEq for IfStatement {
    fn eq(&self, other: &Self) -> bool {
        crate::language::PartialEqAny::eq_dyn(&self.condition, &other.condition)
            && crate::language::PartialEqAny::eq_dyn(
                &self.compound_statement,
                &other.compound_statement,
            )
            && self.else_clause == other.else_clause
    }
}
