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

        let val_str = match self.get_value() {
            Some(val) => match self.data_type {
                DataType::String => format!("\"{}\"", from_byte_vec::<String>(val)),
                DataType::Integer => format!("{}", from_byte_vec::<i32>(val)),
                DataType::Decimal => format!("{}", from_byte_vec::<f64>(val)),
                DataType::Boolean => format!("{}", from_byte_vec::<bool>(val)),
                _ => none_str,
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

    #[test]
    fn display_decimal() {
        let result = SELExecutionResult::new(DataType::Decimal, Some(to_byte_vec(3.14)));

        let formatted = format!("{}", result);

        assert_eq!(formatted, "Decimal - 3.14");
    }

    #[test]
    fn display_bool() {
        let result = SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(false)));

        let formatted = format!("{}", result);

        assert_eq!(formatted, "Boolean - false");
    }
}
