#[derive(Clone, Serialize, Deserialize)]
pub struct Symbol {
    identifier: String,
    table_index: usize,
}

impl Symbol {
    pub fn new(identifier: String, table_index: usize) -> Self {
        return Symbol {
            identifier,
            table_index,
        };
    }

    pub fn get_identifier(&self) -> &String {
        return &self.identifier;
    }

    pub fn get_table_index(&self) -> usize {
        return self.table_index;
    }
}
