use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DataType {
    Unknown,
    Unit,
    Symbol,
    Identifier,
    Integer,
    Decimal,
    String,
    Boolean,
    Range,
    Pair,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
