use crate::{DataType, SELValue};

#[derive(Clone, Serialize, Deserialize)]
pub struct Pair {
    left: SELValue,
    right: SELValue,
}

impl Pair {
    pub fn empty() -> Self {
        return Pair {
            left: SELValue::unknown(),
            right: SELValue::unknown(),
        };
    }

    pub fn new(left: SELValue, right: SELValue) -> Self {
        return Pair { left, right };
    }

    pub fn get_left(&self) -> &SELValue {
        return &self.left;
    }

    pub fn get_right(&self) -> &SELValue {
        return &self.right;
    }
}
