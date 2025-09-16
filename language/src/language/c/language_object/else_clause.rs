use crate::language::c::{
    language_object::{LanguageObject as CLanguageObject, compound_statement::CompoundStatement},
    writers::{Cursor, writer_error::WriterError},
};

#[derive(Debug, Clone)]
pub struct ElseClause {
    pub condition: Option<Box<CLanguageObject>>,
    pub compound_statement: CompoundStatement,
}

impl ElseClause {
    pub fn write(&self, w: &mut dyn Cursor) -> Result<(), WriterError> {
        w.write_else_clause(&self)
    }
}

impl PartialEq for ElseClause {
    fn eq(&self, other: &Self) -> bool {
        // crate::language::PartialEqAny::eq_dyn(&self.condition, &other.condition)
        crate::language::PartialEqAny::eq_dyn(&self.compound_statement, &other.compound_statement)
    }
}
