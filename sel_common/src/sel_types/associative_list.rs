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
        let mut associative_list = AssociativeList::new();

        for value in list.get_values() {
            associative_list.push(value.clone());
        }

        return associative_list;
    }

    pub fn get_list(&self) -> &List {
        return &self.list;
    }

    pub fn get_associations(&self) -> &HashMap<usize, usize> {
        return &self.associations;
    }

    pub fn push(&mut self, value: SELValue) {
        if value.get_type() == DataType::Pair {
            let pair: Pair = from_byte_vec(value.get_value().unwrap());

            if pair.get_left().get_type() == DataType::Symbol {
                let symbol: Symbol = from_byte_vec(pair.get_left().get_value().unwrap());

                let new_index = self.list.get_values().len();
                self.list.push(SELValue::new_from_pair(pair.clone()));
                self.associations
                    .insert(symbol.get_table_index(), new_index);
            }
        } else {
            self.list.push(value);
        }
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
