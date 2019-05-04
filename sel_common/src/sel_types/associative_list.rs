use crate::sel_types::list::List;
use crate::sel_types::pair::Pair;
use crate::sel_types::symbol::Symbol;
use crate::{from_byte_vec, DataType, SELValue};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct AssociativeList {
    list: List,
    associations: HashMap<usize, usize>,
}

impl AssociativeList {
    pub fn new() -> Self {
        return AssociativeList {
            list: List::new(),
            associations: HashMap::new(),
        };
    }

    pub fn from(list: List) -> Self {
        return AssociativeList {
            list: list.clone(),
            associations: HashMap::new(),
        };
    }

    pub fn get_list(&self) -> &List {
        return &self.list;
    }

    pub fn get_associations(&self) -> &HashMap<usize, usize> {
        return &self.associations;
    }

    pub fn push(&mut self, value: SELValue) {
        self.list.push(value);
    }

    pub fn push_association(&mut self, symbol: Symbol, pair: &Pair) {
        let new_index = self.list.get_values().len();
        self.list.push(SELValue::new_from_pair(pair.clone()));
        self.associations
            .insert(symbol.get_table_index(), new_index);
    }

    pub fn get_by_index(&self, index: usize) -> Option<SELValue> {
        return self.list.get_values().get(index).map(|value| value.clone());
    }

    pub fn get_by_association_index(&self, index: usize) -> Option<SELValue> {
        return self
            .associations
            .get(&index)
            .and_then(|associated_index| self.list.get_values().get(*associated_index))
            .map(|sel_value| from_byte_vec::<Pair>(sel_value.get_value().unwrap()))
            .map(|pair| pair.get_right().clone());
    }
}
