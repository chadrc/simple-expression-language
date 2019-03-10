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

impl std::fmt::Debug for SELExecutionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} - {}", self.data_type, self)
    }
}

impl std::fmt::Display for SELExecutionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let none_str = String::from("None");

        let val = self.get_value();

        let val_str = match self.data_type {
            DataType::String => format!("\"{}\"", from_byte_vec::<String>(val.unwrap())),
            DataType::Integer => format!("{}", from_byte_vec::<i32>(val.unwrap())),
            DataType::Decimal => format!("{}", from_byte_vec::<f64>(val.unwrap())),
            DataType::Boolean => format!("{}", from_byte_vec::<bool>(val.unwrap())),
            DataType::Unit => String::from("()"),
            _ => none_str,
        };

        write!(f, "{}", val_str)
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

        assert_eq!(formatted, "\"Hello World\"");
    }

    #[test]
    fn display_int() {
        let result = SELExecutionResult::new(DataType::Integer, Some(to_byte_vec(10)));

        let formatted = format!("{}", result);

        assert_eq!(formatted, "10");
    }

    #[test]
    fn display_decimal() {
        let result = SELExecutionResult::new(DataType::Decimal, Some(to_byte_vec(3.14)));

        let formatted = format!("{}", result);

        assert_eq!(formatted, "3.14");
    }

    #[test]
    fn display_bool() {
        let result = SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(false)));

        let formatted = format!("{}", result);

        assert_eq!(formatted, "false");
    }

    #[test]
    fn display_unit() {
        let result = SELExecutionResult::new(DataType::Unit, None);

        let formatted = format!("{}", result);

        assert_eq!(formatted, "()");
    }

    #[test]
    fn debug_str() {
        let result = SELExecutionResult::new(
            DataType::String,
            Some(to_byte_vec(&String::from("Hello World"))),
        );

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "String - \"Hello World\"");
    }

    #[test]
    fn debug_int() {
        let result = SELExecutionResult::new(DataType::Integer, Some(to_byte_vec(10)));

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "Integer - 10");
    }

    #[test]
    fn debug_decimal() {
        let result = SELExecutionResult::new(DataType::Decimal, Some(to_byte_vec(3.14)));

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "Decimal - 3.14");
    }

    #[test]
    fn debug_bool() {
        let result = SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(false)));

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "Boolean - false");
    }

    #[test]
    fn debug_unit() {
        let result = SELExecutionResult::new(DataType::Unit, None);

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "Unit - ()");
    }
}
