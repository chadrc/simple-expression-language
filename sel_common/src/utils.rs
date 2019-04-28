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

pub fn to_byte_vec<T: ToByteVec>(val: T) -> Vec<u8> {
    return val.to_byte_vec();
}

pub fn from_byte_vec<T: FromByteVec>(v: &Vec<u8>) -> T {
    return T::from_byte_vec(v);
}
