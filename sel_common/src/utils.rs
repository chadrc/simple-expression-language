use super::sel_types::{AssociativeList, Expression, List, Pair, Range, Symbol};
use bincode::{deserialize, serialize};

pub trait ToByteVec {
    fn to_byte_vec(&self) -> Vec<u8>;
}

pub trait FromByteVec {
    fn from_byte_vec(v: &Vec<u8>) -> Self;
}

impl ToByteVec for &String {
    fn to_byte_vec(&self) -> Vec<u8> {
        return serialize(self).unwrap_or(vec![]);
    }
}

impl FromByteVec for String {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        return deserialize(v).unwrap_or(String::from(""));
    }
}

impl ToByteVec for i32 {
    fn to_byte_vec(&self) -> Vec<u8> {
        return serialize(self).unwrap_or(vec![]);
    }
}

impl FromByteVec for i32 {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        return deserialize(v).unwrap_or(0);
    }
}

impl ToByteVec for i64 {
    fn to_byte_vec(&self) -> Vec<u8> {
        return serialize(self).unwrap_or(vec![]);
    }
}

impl FromByteVec for i64 {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        return deserialize(v).unwrap_or(0);
    }
}

impl ToByteVec for f64 {
    fn to_byte_vec(&self) -> Vec<u8> {
        return serialize(self).unwrap_or(vec![]);
    }
}

impl FromByteVec for f64 {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        return deserialize(v).unwrap_or(0.0);
    }
}

impl ToByteVec for usize {
    fn to_byte_vec(&self) -> Vec<u8> {
        return serialize(self).unwrap_or(vec![]);
    }
}

impl FromByteVec for usize {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        return deserialize(v).unwrap_or(0);
    }
}

impl ToByteVec for bool {
    fn to_byte_vec(&self) -> Vec<u8> {
        return serialize(self).unwrap_or(vec![]);
    }
}

impl FromByteVec for bool {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        return deserialize(v).unwrap_or(false);
    }
}

impl ToByteVec for Pair {
    fn to_byte_vec(&self) -> Vec<u8> {
        return serialize(self).unwrap_or(vec![]);
    }
}

impl FromByteVec for Pair {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        return deserialize(v).unwrap_or(Pair::empty());
    }
}

impl ToByteVec for Range {
    fn to_byte_vec(&self) -> Vec<u8> {
        return serialize(self).unwrap_or(vec![]);
    }
}

impl FromByteVec for Range {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        return deserialize(v).unwrap_or(Range::new(0, 0));
    }
}

impl ToByteVec for Symbol {
    fn to_byte_vec(&self) -> Vec<u8> {
        return serialize(self).unwrap_or(vec![]);
    }
}

impl FromByteVec for Symbol {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        return deserialize(v).unwrap_or(Symbol::new(String::from(""), 0));
    }
}

impl ToByteVec for List {
    fn to_byte_vec(&self) -> Vec<u8> {
        return serialize(self).unwrap_or(vec![]);
    }
}

impl FromByteVec for List {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        return deserialize(v).unwrap_or(List::new());
    }
}

impl ToByteVec for AssociativeList {
    fn to_byte_vec(&self) -> Vec<u8> {
        return serialize(self).unwrap_or(vec![]);
    }
}

impl FromByteVec for AssociativeList {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        return deserialize(v).unwrap_or(AssociativeList::new());
    }
}

impl ToByteVec for Expression {
    fn to_byte_vec(&self) -> Vec<u8> {
        return serialize(self).unwrap_or(vec![]);
    }
}

impl FromByteVec for Expression {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        return deserialize(v).unwrap_or(Expression::new(None));
    }
}

pub fn to_byte_vec<T: ToByteVec>(val: T) -> Vec<u8> {
    return val.to_byte_vec();
}

pub fn from_byte_vec<T: FromByteVec>(v: &Vec<u8>) -> T {
    return T::from_byte_vec(v);
}
