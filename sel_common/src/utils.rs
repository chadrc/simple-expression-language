use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

pub trait ToByteVec {
    fn to_byte_vec(&self) -> Vec<u8>;
}

pub trait FromByteVec {
    fn from_byte_vec(v: &Vec<u8>) -> Self;
}

impl ToByteVec for &String {
    fn to_byte_vec(&self) -> Vec<u8> {
        return (*self).clone().into_bytes();
    }
}

impl FromByteVec for String {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        let cow = String::from_utf8_lossy(v);
        return cow.to_owned().to_string();
    }
}

impl ToByteVec for i64 {
    fn to_byte_vec(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes.write_i64::<LittleEndian>(*self).unwrap();
        return bytes;
    }
}

impl FromByteVec for i64 {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        return match Cursor::new(v).read_i64::<LittleEndian>() {
            Ok(val) => val,
            Err(_) => 0,
        };
    }
}

impl ToByteVec for f64 {
    fn to_byte_vec(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes.write_f64::<LittleEndian>(*self).unwrap();
        return bytes;
    }
}

impl FromByteVec for f64 {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        return match Cursor::new(v).read_f64::<LittleEndian>() {
            Ok(val) => val,
            Err(_) => 0.0,
        };
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

impl FromByteVec for bool {
    fn from_byte_vec(v: &Vec<u8>) -> Self {
        return match v.get(0) {
            Some(num) => match num {
                0 => false,
                1 => true,
                _ => false,
            },
            None => false,
        };
    }
}

pub fn to_byte_vec<T: ToByteVec>(val: T) -> Vec<u8> {
    return val.to_byte_vec();
}

pub fn from_byte_vec<T: FromByteVec>(v: &Vec<u8>) -> T {
    return T::from_byte_vec(v);
}
