use crate::{from_byte_vec, to_byte_vec, SymbolTable};
use std::collections::HashMap;

pub struct SELContext {
    symbol_table: SymbolTable,
    symbol_values: HashMap<usize, Vec<u8>>,
}

impl SELContext {
    pub fn new() -> Self {
        return SELContext {
            symbol_table: SymbolTable::new(),
            symbol_values: HashMap::new(),
        };
    }

    pub fn set_integer_symbol(&mut self, symbol: &String, value: i64) -> usize {
        let index = self.symbol_table.add(symbol);
        self.symbol_values.insert(index, to_byte_vec(value));

        index
    }

    pub fn get_integer_value(&self, index: usize) -> Option<i64> {
        return self.symbol_values.get(&index).map(|val| from_byte_vec(val));
    }
}

#[cfg(test)]
mod tests {
    use crate::context::SELContext;
    use crate::from_byte_vec;

    #[test]
    fn create() {
        SELContext::new();
    }

    #[test]
    fn set_integer_value() {
        let mut context = SELContext::new();
        let insert_index = context.set_integer_symbol(&String::from("value"), 10);
        let bytes = context.symbol_values.get(&insert_index).unwrap();
        let value: i64 = from_byte_vec(bytes);

        assert_eq!(value, 10);
    }

    #[test]
    fn get_integer_value_with_index() {
        let mut context = SELContext::new();
        let insert_index = context.set_integer_symbol(&String::from("value"), 10);

        let value = context.get_integer_value(insert_index).unwrap();

        assert_eq!(value, 10);
    }
}
