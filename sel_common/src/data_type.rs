use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum DataType {
    Unknown,
    Unit,
    Integer,
    Decimal,
    String,
    Boolean,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
