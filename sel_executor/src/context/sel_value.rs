use sel_common::{from_byte_vec, to_byte_vec, DataType};
use std::fmt;

pub struct SELValue {
    data_type: DataType,
    value: Option<Vec<u8>>,
}

impl SELValue {
    pub fn new_from_int(num: i32) -> Self {
        return SELValue {
            data_type: DataType::Integer,
            value: Some(to_byte_vec(num)),
        };
    }
    pub fn get_type(&self) -> DataType {
        return self.data_type;
    }

    pub fn get_value(&self) -> Option<&Vec<u8>> {
        return match &self.value {
            Some(v) => Some(&v),
            None => None,
        };
    }
}
