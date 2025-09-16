use bimap::{BiHashMap, Overwritten};
use uuid::Uuid;

#[derive(Debug)]
pub struct SymbolAlreadyExists {}

pub struct Context<'a> {
    /// A map of symbol names to their corresponding IDs (<id, name>)
    symbols: BiHashMap<Uuid, String>,
    parent: Option<&'a Context<'a>>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Self {
            symbols: BiHashMap::new(),
            parent: None,
        }
    }

    pub fn branch(&'a self) -> Context<'a> {
        Context {
            symbols: BiHashMap::new(),
            parent: Some(&self),
        }
    }

    pub fn get_symbol_id(&self, identifier: &String, is_fn: bool) -> Option<Uuid> {
        self.symbols
            .get_by_right(&variable_or_function_identifier(identifier, is_fn))
            .cloned()
            .or(self
                .parent
                .and_then(|parent| parent.get_symbol_id(identifier, is_fn)))
    }

    pub fn get_symbol_identifier(&self, id: &Uuid) -> Option<String> {
        self.symbols
            .get_by_left(id)
            .cloned()
            .or(self
                .parent
                .and_then(|parent| parent.get_symbol_identifier(id)))
            .map(strip_function_differentiatior_from_identifier)
    }

    pub fn get_or_insert_symbol(&mut self, identifier: &String, is_fn: bool) -> Uuid {
        self.get_symbol_id(identifier, is_fn)
            .unwrap_or_else(|| self.insert_symbol(identifier, is_fn).unwrap())
    }

    pub fn insert_symbol(
        &mut self,
        identifier: &String,
        is_fn: bool,
    ) -> Result<Uuid, SymbolAlreadyExists> {
        let id = Uuid::new_v4();
        let res = self
            .symbols
            .insert(id, variable_or_function_identifier(identifier, is_fn)); // TODO figure out a better way to differentiate fn from variables
        match res {
            Overwritten::Neither => Ok(id),
            Overwritten::Right(_, _) => Err(SymbolAlreadyExists {}),
            other => panic!("Unexpected overwrite case: {:?}", other),
        }
    }

    pub fn insert_symbol_with_id(
        &mut self,
        identifier: &String,
        id: Uuid,
        is_fn: bool,
    ) -> Result<Uuid, SymbolAlreadyExists> {
        let res = self
            .symbols
            .insert(id, variable_or_function_identifier(identifier, is_fn));
        match res {
            Overwritten::Neither => Ok(id),
            _ => Err(SymbolAlreadyExists {}),
        }
    }

    pub fn overwrite_symbol(&mut self, identifier: &String, is_fn: bool) -> Uuid {
        // TODO is this even necesary?
        let id = Uuid::new_v4();
        self.symbols
            .insert(id, variable_or_function_identifier(identifier, is_fn));
        id
    }
}

fn variable_or_function_identifier(identifier: &String, is_fn: bool) -> String {
    if is_fn {
        format!("{}()", identifier) // TODO improve how to differentiate functions with variable symbols
    } else {
        format!("{}", identifier)
    }
}

fn strip_function_differentiatior_from_identifier(identifier: String) -> String {
    identifier.trim_end_matches("()").to_string()
}
