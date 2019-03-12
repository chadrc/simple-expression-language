use sel_common::{from_byte_vec, to_byte_vec, DataType};
use std::fmt;

pub struct SELValue {
    data_type: DataType,
    value: Option<Vec<u8>>,
}

impl SELValue {
    pub fn new() -> Self {
        return SELValue {
            data_type: DataType::Unit,
            value: None,
        };
    }

    pub fn new_from_raw(data_type: DataType, v: Option<Vec<u8>>) -> Self {
        return SELValue {
            data_type: data_type,
            value: v,
        };
    }

    pub fn new_from_int(num: i32) -> Self {
        return SELValue {
            data_type: DataType::Integer,
            value: Some(to_byte_vec(num)),
        };
    }

    pub fn new_from_decimal(num: f64) -> Self {
        return SELValue {
            data_type: DataType::Decimal,
            value: Some(to_byte_vec(num)),
        };
    }

    pub fn new_from_string(s: &String) -> Self {
        return SELValue {
            data_type: DataType::String,
            value: Some(to_byte_vec(s)),
        };
    }

    pub fn new_from_boolean(b: bool) -> Self {
        return SELValue {
            data_type: DataType::Boolean,
            value: Some(to_byte_vec(b)),
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

impl std::fmt::Debug for SELValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} - {}", self.data_type, self)
    }
}

impl std::fmt::Display for SELValue {
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

    #[test]
    fn display_str() {
        let result = SELValue::new_from_string(&String::from("Hello World"));

        let formatted = format!("{}", result);

        assert_eq!(formatted, "\"Hello World\"");
    }

    #[test]
    fn display_int() {
        let result = SELValue::new_from_int(10);

        let formatted = format!("{}", result);

        assert_eq!(formatted, "10");
    }

    #[test]
    fn display_decimal() {
        let result = SELValue::new_from_decimal(3.14);

        let formatted = format!("{}", result);

        assert_eq!(formatted, "3.14");
    }

    #[test]
    fn display_bool() {
        let result = SELValue::new_from_boolean(false);

        let formatted = format!("{}", result);

        assert_eq!(formatted, "false");
    }

    #[test]
    fn display_unit() {
        let result = SELValue::new();

        let formatted = format!("{}", result);

        assert_eq!(formatted, "()");
    }

    #[test]
    fn debug_str() {
        let result = SELValue::new_from_string(&String::from("Hello World"));

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "String - \"Hello World\"");
    }

    #[test]
    fn debug_int() {
        let result = SELValue::new_from_int(10);

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "Integer - 10");
    }

    #[test]
    fn debug_decimal() {
        let result = SELValue::new_from_decimal(3.14);

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "Decimal - 3.14");
    }

    #[test]
    fn debug_bool() {
        let result = SELValue::new_from_boolean(false);

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "Boolean - false");
    }

    #[test]
    fn debug_unit() {
        let result = SELValue::new();

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "Unit - ()");
    }
}
