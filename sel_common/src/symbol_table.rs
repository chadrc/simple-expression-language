use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<String>,
    key_to_index: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        return SymbolTable {
            symbols: vec![],
            key_to_index: HashMap::new(),
        };
    }

    pub fn add(&mut self, symbol: &String) {
        self.symbols.push(symbol.clone());
        self.key_to_index
            .insert(symbol.clone(), self.symbols.len() - 1);
    }

    pub fn get_symbol(&self, value: usize) -> Option<&String> {
        return self.symbols.get(value);
    }

    pub fn get_value(&self, symbol: &String) -> Option<&usize> {
        return self.key_to_index.get(symbol);
    }
}
