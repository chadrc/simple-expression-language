use crate::SELValue;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct List {
    values: Vec<SELValue>,
}

impl List {
    pub fn new() -> Self {
        return List { values: vec![] };
    }

    pub fn get_values(&self) -> &Vec<SELValue> {
        return &self.values;
    }

    pub fn push(&mut self, value: SELValue) {
        self.values.push(value);
    }

    pub fn insert(&mut self, index: usize, value: SELValue) {
        self.values.insert(index, value);
    }
}
