use crate::{DataType, SELValue};

#[derive(Clone, Serialize, Deserialize)]
pub struct Range {
    lower: i64,
    upper: i64,
}

impl Range {
    pub fn new(lower: i64, upper: i64) -> Self {
        return Range { lower, upper };
    }

    pub fn get_lower(&self) -> i64 {
        return self.lower;
    }

    pub fn get_upper(&self) -> i64 {
        return self.upper;
    }
}
