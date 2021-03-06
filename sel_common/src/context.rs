use crate::{from_byte_vec, to_byte_vec, SELValue, SymbolTable};
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};

pub type SELFunction = fn(SELValue, &SymbolTable) -> SELValue;

pub struct SELContext {
    symbol_table: SymbolTable,
    symbol_values: HashMap<usize, SELValue>,
    functions: HashMap<String, SELFunction>,
}

impl Debug for SELContext {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&format!(
            "{:?} - {:?} - {:?}",
            self.symbol_table,
            self.symbol_values,
            self.functions.keys()
        ))
    }
}

impl SELContext {
    pub fn new() -> Self {
        return SELContext {
            symbol_table: SymbolTable::new(),
            symbol_values: HashMap::new(),
            functions: HashMap::new(),
        };
    }

    pub fn add_symbol(&mut self, symbol: &String) -> usize {
        return self.symbol_table.add(symbol);
    }

    pub fn get_symbol_table(&self) -> &SymbolTable {
        return &self.symbol_table;
    }

    pub fn set_integer_symbol(&mut self, symbol: &String, value: i64) -> usize {
        let index = self.symbol_table.add(symbol);
        self.symbol_values
            .insert(index, SELValue::new_from_int(value));

        index
    }

    pub fn set_decimal_symbol(&mut self, symbol: &String, value: f64) -> usize {
        let index = self.symbol_table.add(symbol);
        self.symbol_values
            .insert(index, SELValue::new_from_decimal(value));

        index
    }

    pub fn get_value(&self, index: usize) -> Option<&SELValue> {
        return self.symbol_values.get(&index);
    }

    pub fn get_integer_value(&self, index: usize) -> Option<i64> {
        return self
            .symbol_values
            .get(&index)
            .and_then(|val| val.get_value())
            .map(|bytes| from_byte_vec(bytes));
    }

    pub fn get_integer_value_with_key(&self, key: &String) -> Option<i64> {
        return self
            .symbol_table
            .get_value(key)
            .and_then(|index| self.get_integer_value(*index));
    }

    pub fn register_function(&mut self, name: &str, func: SELFunction) {
        self.functions.insert(String::from(name), func);
    }

    pub fn get_functions(&self) -> &HashMap<String, SELFunction> {
        return &self.functions;
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
        let sel_value = context.symbol_values.get(&insert_index).unwrap();
        let value: i64 = sel_value
            .get_value()
            .map_or(0, |bytes| from_byte_vec(bytes));

        assert_eq!(value, 10);
    }

    #[test]
    fn get_integer_value_with_index() {
        let mut context = SELContext::new();
        let insert_index = context.set_integer_symbol(&String::from("value"), 10);

        let value = context.get_integer_value(insert_index).unwrap();

        assert_eq!(value, 10);
    }

    #[test]
    fn get_integer_value_with_key() {
        let mut context = SELContext::new();
        let insert_index = context.set_integer_symbol(&String::from("value"), 10);

        let value = context
            .get_integer_value_with_key(&String::from("value"))
            .unwrap();

        assert_eq!(value, 10);
    }
}
