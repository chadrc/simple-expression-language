use sel_common::{from_byte_vec, DataType};
use std::fmt;

pub struct SELExecutionResult {
    data_type: DataType,
    value: Option<Vec<u8>>,
}

impl SELExecutionResult {
    pub fn new(data_type: DataType, value: Option<Vec<u8>>) -> Self {
        return SELExecutionResult {
            data_type: data_type,
            value: value,
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

impl std::fmt::Display for SELExecutionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let none_str = String::from("None");

        let val_str = match self.data_type {
            DataType::String => match self.get_value() {
                Some(val) => format!("\"{}\"", from_byte_vec::<String>(val)),
                None => none_str,
            },
            DataType::Integer => match self.get_value() {
                Some(val) => format!("{}", from_byte_vec::<i32>(val)),
                None => none_str,
            },
            _ => none_str,
        };

        write!(f, "{} - {}", self.data_type, val_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sel_common::to_byte_vec;

    #[test]
    fn display_str() {
        let result = SELExecutionResult::new(
            DataType::String,
            Some(to_byte_vec(&String::from("Hello World"))),
        );

        let formatted = format!("{}", result);

        assert_eq!(formatted, "String - \"Hello World\"");
    }

    #[test]
    fn display_int() {
        let result = SELExecutionResult::new(DataType::Integer, Some(to_byte_vec(10)));

        let formatted = format!("{}", result);

        assert_eq!(formatted, "Integer - 10");
    }
}
