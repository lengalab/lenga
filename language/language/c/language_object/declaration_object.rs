use crate::language::c::language_object::{
    LanguageObject,
    special_object::{comment, unknown::Unknown},
};

use uuid::Uuid;

pub mod declaration;
pub mod function_declaration;
pub mod function_definition;
pub mod preproc_include;

#[derive(
    Debug,
    PartialEq,
    Clone,
    lenga_field_inspect_derive::VariantProvider,
    lenga_field_inspect_derive::FieldInspect,
)]
pub enum DeclarationObject {
    Declaration(declaration::Declaration),
    FunctionDeclaration(function_declaration::FunctionDeclaration),
    FunctionDefinition(function_definition::FunctionDefinition),
    PreprocInclude(preproc_include::PreprocInclude),
    Comment(comment::Comment), // TODO: This doesn't belong here. To avoid putting comments in every enum, I woud try to match them with their corresponding node
    Unknown(Unknown),
}

impl DeclarationObject {
    pub fn write(
        &self,
        w: &mut dyn crate::language::c::writers::Cursor,
    ) -> Result<(), crate::language::c::writers::writer_error::WriterError> {
        match self {
            DeclarationObject::Declaration(decl) => decl.write(w),
            DeclarationObject::FunctionDeclaration(func_decl) => func_decl.write(w),
            DeclarationObject::FunctionDefinition(func_def) => func_def.write(w),
            DeclarationObject::PreprocInclude(preproc) => preproc.write(w),
            DeclarationObject::Unknown(unknown) => unknown.write(w),
            DeclarationObject::Comment(comment) => comment.write(w),
        }
    }

    pub fn id(&self) -> Uuid {
        match self {
            DeclarationObject::Declaration(decl) => decl.id,
            DeclarationObject::FunctionDeclaration(func_decl) => func_decl.id,
            DeclarationObject::FunctionDefinition(func_def) => func_def.id,
            DeclarationObject::PreprocInclude(preproc) => preproc.id,
            DeclarationObject::Unknown(unknown) => unknown.id,
            DeclarationObject::Comment(comment) => comment.id,
        }
    }
}

impl Default for DeclarationObject {
    fn default() -> Self {
        DeclarationObject::Unknown(Unknown::default())
    }
}

impl From<DeclarationObject> for LanguageObject {
    fn from(value: DeclarationObject) -> Self {
        match value {
            DeclarationObject::Declaration(decl) => LanguageObject::Declaration(decl.clone()),
            DeclarationObject::FunctionDeclaration(func_decl) => {
                LanguageObject::FunctionDeclaration(func_decl.clone())
            }
            DeclarationObject::FunctionDefinition(func_def) => {
                LanguageObject::FunctionDefinition(func_def.clone())
            }
            DeclarationObject::PreprocInclude(preproc) => {
                LanguageObject::PreprocInclude(preproc.clone())
            }
            DeclarationObject::Unknown(unknown) => LanguageObject::Unknown(unknown.clone()),
            DeclarationObject::Comment(comment) => LanguageObject::Comment(comment.clone()),
        }
    }
}

impl TryFrom<LanguageObject> for DeclarationObject {
    type Error = crate::language::c::language_object::ConversionError;

    fn try_from(value: LanguageObject) -> Result<Self, Self::Error> {
        match value {
            LanguageObject::Declaration(decl) => Ok(DeclarationObject::Declaration(decl)),
            LanguageObject::FunctionDeclaration(func_decl) => {
                Ok(DeclarationObject::FunctionDeclaration(func_decl))
            }
            LanguageObject::FunctionDefinition(func_def) => {
                Ok(DeclarationObject::FunctionDefinition(func_def))
            }
            LanguageObject::PreprocInclude(preproc) => {
                Ok(DeclarationObject::PreprocInclude(preproc))
            }
            LanguageObject::Unknown(unknown) => Ok(DeclarationObject::Unknown(unknown)),
            LanguageObject::Comment(comment) => Ok(DeclarationObject::Comment(comment)),
            _ => Err(crate::language::c::language_object::ConversionError(
                "Cannot convert LanguageObject to DeclarationObject".into(),
            )),
        }
    }
}
