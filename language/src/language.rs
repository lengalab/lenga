use std::{any::Any, fmt::Debug};

pub mod c;

pub trait LanguageObject: Debug + Any + PartialEqAny {
    fn as_any(&self) -> &dyn Any;
}

pub trait PartialEqAny {
    fn eq_dyn(&self, other: &dyn PartialEqAny) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl<T> PartialEqAny for T
where
    T: 'static + PartialEq + Any,
{
    fn eq_dyn(&self, other: &dyn PartialEqAny) -> bool {
        other.as_any().downcast_ref::<T>() == Some(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PartialEq for dyn LanguageObject {
    fn eq(&self, other: &Self) -> bool {
        <dyn PartialEqAny>::eq_dyn(self, other)
    }
}

pub trait Language {
    type Object: LanguageObject;
    type SourceFile;

    fn file_extension(&self) -> String;
    fn name(&self) -> String;
    fn parse_text(&self, content: &str) -> Result<Self::SourceFile, String>;
    fn write_to_text(&self, src_file: Self::SourceFile) -> Result<String, String>;
    fn parse_nodes(&self, nodes: Vec<u8>) -> Result<Self::SourceFile, String>;
    fn write_to_nodes(&self, src_file: Self::SourceFile) -> Result<Vec<u8>, String>;
}
