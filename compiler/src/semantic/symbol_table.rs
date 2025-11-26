use std::collections::HashMap;
use crate::parser::ast::{Type, FunctionDecl};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Symbol {
    Variable {
        name: String,
        type_: Type,
        defined: bool,
    },
    Function {
        declaration: FunctionDecl,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope {
    symbols: HashMap<String, Symbol>,
    parent: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
    current_scope: usize,
}

impl Clone for SymbolTable {
    fn clone(&self) -> Self {
        Self {
            scopes: self.scopes.clone(),
            current_scope: self.current_scope,
        }
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        let global_scope = Scope {
            symbols: HashMap::new(),
            parent: None,
        };
        
        Self {
            scopes: vec![global_scope],
            current_scope: 0,
        }
    }

    pub fn enter_scope(&mut self) {
        let new_scope = Scope {
            symbols: HashMap::new(),
            parent: Some(self.current_scope),
        };
        self.scopes.push(new_scope);
        self.current_scope = self.scopes.len() - 1;
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    pub fn insert_symbol(&mut self, name: String, symbol: Symbol) -> Result<(), String> {
        let current_scope = &mut self.scopes[self.current_scope];
        
        if current_scope.symbols.contains_key(&name) {
            return Err(format!("Símbolo '{}' já declarado neste escopo", name));
        }
        
        current_scope.symbols.insert(name, symbol);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        let mut scope_index = Some(self.current_scope);
        
        while let Some(idx) = scope_index {
            let scope = &self.scopes[idx];
            if let Some(symbol) = scope.symbols.get(name) {
                return Some(symbol);
            }
            scope_index = scope.parent;
        }
        
        None
    }

    pub fn lookup_current_scope(&self, name: &str) -> Option<&Symbol> {
        self.scopes[self.current_scope].symbols.get(name)
    }

    pub fn update_variable_definition(&mut self, name: &str) -> Result<(), String> {
        let mut scope_index = Some(self.current_scope);
        
        while let Some(idx) = scope_index {
            let scope = &mut self.scopes[idx];
            if let Some(Symbol::Variable { defined, .. }) = scope.symbols.get_mut(name) {
                *defined = true;
                return Ok(());
            }
            scope_index = scope.parent;
        }
        
        Err(format!("Variável '{}' não encontrada", name))
    }
}