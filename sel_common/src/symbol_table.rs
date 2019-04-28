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

    pub fn add(&mut self, symbol: &String) -> usize {
        return match self.key_to_index.get(symbol) {
            Some(value) => *value,
            None => {
                let index = self.symbols.len();
                self.symbols.push(symbol.clone());
                self.key_to_index.insert(symbol.clone(), index);

                index
            }
        };
    }

    pub fn get_symbol(&self, value: usize) -> Option<&String> {
        return self.symbols.get(value);
    }

    pub fn get_value(&self, symbol: &String) -> Option<&usize> {
        return self.key_to_index.get(symbol);
    }
}

#[cfg(test)]
mod tests {
    use super::SymbolTable;

    #[test]
    fn create() {
        SymbolTable::new();
    }

    #[test]
    fn add() {
        let mut table = SymbolTable::new();
        table.add(&String::from("symbol1"));

        assert_eq!(table.symbols.len(), 1);
    }

    #[test]
    fn add_same() {
        let mut table = SymbolTable::new();
        table.add(&String::from("symbol1"));
        table.add(&String::from("symbol1"));

        assert_eq!(table.symbols.len(), 1);
    }
}
