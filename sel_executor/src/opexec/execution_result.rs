use sel_common::{DataType, SELValue};
use std::fmt;

#[derive(Clone)]
pub struct SELExecutionResult {
    value: SELValue,
}

impl SELExecutionResult {
    pub fn new(data_type: DataType, value: Option<Vec<u8>>) -> Self {
        return SELExecutionResult {
            value: SELValue::new_from_raw(data_type, value),
        };
    }

    pub fn from(value: &SELValue) -> Self {
        return SELExecutionResult {
            value: value.clone(),
        };
    }

    pub fn get_type(&self) -> DataType {
        return self.value.get_type();
    }

    pub fn get_value(&self) -> Option<&Vec<u8>> {
        return match &self.value.get_value() {
            Some(v) => Some(&v),
            None => None,
        };
    }

    pub fn get_sel_value(&self) -> &SELValue {
        return &self.value;
    }
}

impl std::fmt::Debug for SELExecutionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} - {}", self.get_type(), self)
    }
}

impl std::fmt::Display for SELExecutionResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
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
