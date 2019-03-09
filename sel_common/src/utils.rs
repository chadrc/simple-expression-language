use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

pub trait ToByteVec {
    fn to_byte_vec(&self) -> Vec<u8>;
}

impl ToByteVec for &String {
    fn to_byte_vec(&self) -> Vec<u8> {
        return (*self).clone().into_bytes();
    }
}

impl ToByteVec for i64 {
    fn to_byte_vec(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes.write_i64::<LittleEndian>(*self).unwrap();
        return bytes;
    }
}

impl ToByteVec for f64 {
    fn to_byte_vec(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes.write_f64::<LittleEndian>(*self).unwrap();
        return bytes;
    }
}

impl ToByteVec for bool {
    fn to_byte_vec(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        match self {
            true => bytes.push(1),
            false => bytes.push(0),
        }
        return bytes;
    }
}

pub fn to_byte_vec<T: ToByteVec>(val: T) -> Vec<u8> {
    return val.to_byte_vec();
}
