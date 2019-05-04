use crate::sel_types::list::List;
use crate::SELValue;

#[derive(Clone, Serialize, Deserialize)]
pub struct AssociativeList {
    list: List,
}

impl AssociativeList {
    pub fn new() -> Self {
        return AssociativeList { list: List::new() };
    }

    pub fn from(list: List) -> Self {
        return AssociativeList { list: list.clone() };
    }

    pub fn get_list(&self) -> &List {
        return &self.list;
    }

    pub fn push(&mut self, value: SELValue) {
        self.list.push(value);
    }
}
