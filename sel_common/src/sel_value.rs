use super::sel_types::{List, Pair, Range, Symbol};
use super::{from_byte_vec, to_byte_vec, DataType};
use core::fmt::Debug;
use std::fmt;

#[derive(Clone, Serialize, Deserialize)]
pub struct SELValue {
    data_type: DataType,
    value: Option<Vec<u8>>,
}

impl SELValue {
    pub fn unknown() -> Self {
        return SELValue {
            data_type: DataType::Unknown,
            value: None,
        };
    }

    pub fn new() -> Self {
        return SELValue {
            data_type: DataType::Unit,
            value: None,
        };
    }

    pub fn new_from_raw(data_type: DataType, v: Option<Vec<u8>>) -> Self {
        return SELValue {
            data_type,
            value: v,
        };
    }

    pub fn new_from_int(num: i64) -> Self {
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

    pub fn new_from_range(lower: i64, upper: i64) -> Self {
        return SELValue {
            data_type: DataType::Range,
            value: Some(to_byte_vec(Range::new(lower, upper))),
        };
    }

    pub fn new_from_pair(pair: Pair) -> Self {
        return SELValue {
            data_type: DataType::Pair,
            value: Some(to_byte_vec(pair)),
        };
    }

    pub fn new_from_symbol(symbol: Symbol) -> Self {
        return SELValue {
            data_type: DataType::Symbol,
            value: Some(to_byte_vec(symbol)),
        };
    }

    pub fn new_from_list(list: List) -> Self {
        return SELValue {
            data_type: DataType::List,
            value: Some(to_byte_vec(list)),
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
        match self.data_type {
            DataType::Symbol => {
                let val = self.get_value();
                let symbol: Symbol = from_byte_vec(val.unwrap());
                write!(
                    f,
                    "{:?}({}) - :{}",
                    self.data_type,
                    symbol.get_table_index(),
                    symbol.get_identifier()
                )
            }
            _ => write!(f, "{:?} - {}", self.data_type, self),
        }
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
            DataType::Symbol => {
                let symbol: Symbol = from_byte_vec(val.unwrap());
                format!(":{}", symbol.get_identifier())
            }
            DataType::Range => {
                let range: Range = from_byte_vec(val.unwrap());
                format!("{}..{}", range.get_lower(), range.get_upper())
            }
            DataType::Pair => {
                let pair: Pair = from_byte_vec(val.unwrap());
                format!(
                    "{} = {}",
                    pair_format(&pair.get_left()),
                    pair_format(&pair.get_right())
                )
            }
            DataType::List => {
                let list: List = from_byte_vec(val.unwrap());
                let mut item_strs: Vec<String> = vec![];

                for item in list.get_values() {
                    item_strs.push(format!("{}", item));
                }

                format!("{}", item_strs.join(", "))
            }
            DataType::Unit => String::from("()"),
            _ => none_str,
        };

        write!(f, "{}", val_str)
    }
}

fn pair_format(value: &SELValue) -> String {
    format!(
        "{}",
        if value.get_type() == DataType::Pair {
            format!("({})", value)
        } else {
            format!("{}", value)
        }
    )
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
    fn display_symbol() {
        let result = SELValue::new_from_symbol(Symbol::new(String::from("value"), 10));

        let formatted = format!("{}", result);

        assert_eq!(formatted, ":value");
    }

    #[test]
    fn display_range() {
        let result = SELValue::new_from_range(5, 10);

        let formatted = format!("{}", result);

        assert_eq!(formatted, "5..10");
    }

    #[test]
    fn display_pair() {
        let result = SELValue::new_from_pair(Pair::new(
            SELValue::new_from_string(&String::from("value")),
            SELValue::new_from_int(10),
        ));

        let formatted = format!("{}", result);

        assert_eq!(formatted, "\"value\" = 10");
    }

    #[test]
    fn display_list() {
        let mut list = List::new();
        list.push(SELValue::new_from_string(&String::from("value")));
        list.push(SELValue::new_from_int(10));

        let result = SELValue::new_from_list(list);

        let formatted = format!("{}", result);

        assert_eq!(formatted, "\"value\", 10");
    }

    #[test]
    fn display_pair_pair() {
        let result = SELValue::new_from_pair(Pair::new(
            SELValue::new_from_string(&String::from("value")),
            SELValue::new_from_pair(Pair::new(
                SELValue::new_from_symbol(Symbol::new(String::from("field"), 0)),
                SELValue::new_from_int(50),
            )),
        ));

        let formatted = format!("{}", result);

        assert_eq!(formatted, "\"value\" = (:field = 50)");
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
    fn debug_symbol() {
        let result = SELValue::new_from_symbol(Symbol::new(String::from("value"), 10));

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "Symbol(10) - :value");
    }

    #[test]
    fn debug_range() {
        let result = SELValue::new_from_range(5, 10);

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "Range - 5..10");
    }

    #[test]
    fn debug_list() {
        let mut list = List::new();
        list.push(SELValue::new_from_string(&String::from("value")));
        list.push(SELValue::new_from_int(10));

        let result = SELValue::new_from_list(list);

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "List - \"value\", 10");
    }

    #[test]
    fn debug_pair() {
        let result = SELValue::new_from_pair(Pair::new(
            SELValue::new_from_string(&String::from("value")),
            SELValue::new_from_int(10),
        ));

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "Pair - \"value\" = 10");
    }

    #[test]
    fn debug_pair_pair() {
        let result = SELValue::new_from_pair(Pair::new(
            SELValue::new_from_string(&String::from("value")),
            SELValue::new_from_pair(Pair::new(
                SELValue::new_from_symbol(Symbol::new(String::from("field"), 0)),
                SELValue::new_from_int(50),
            )),
        ));

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "Pair - \"value\" = (:field = 50)");
    }

    #[test]
    fn debug_unit() {
        let result = SELValue::new();

        let formatted = format!("{:?}", result);

        assert_eq!(formatted, "Unit - ()");
    }
}
