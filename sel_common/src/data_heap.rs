use super::utils::to_byte_vec;
use super::DataType;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct DataHeap {
    data: Vec<Vec<u8>>,
}

impl DataHeap {
    pub fn new() -> Self {
        return DataHeap { data: vec![] };
    }

    pub fn insert_from_string(&mut self, data_type: DataType, value_str: &String) -> Option<usize> {
        return match data_type {
            DataType::Integer => {
                let num = value_str.parse::<i64>().unwrap();
                self.data.push(to_byte_vec(num));
                Some(self.data.len() - 1)
            }
            DataType::Decimal => {
                let num = value_str.parse::<f64>().unwrap();
                self.data.push(to_byte_vec(num));
                Some(self.data.len() - 1)
            }
            DataType::String => {
                self.data.push(to_byte_vec(value_str));
                Some(self.data.len() - 1)
            }
            DataType::Boolean => {
                let b: bool = match FromStr::from_str(value_str) {
                    Ok(val) => val,
                    Err(_) => false, // probably panic?
                };

                self.data.push(to_byte_vec(b));
                Some(self.data.len() - 1)
            }
            _ => None,
        };
    }

    pub fn get_bytes(&self, index: usize) -> Option<Vec<u8>> {
        return match self.data.get(index) {
            Some(datum) => Some(datum.clone()),
            None => None,
        };
    }

    pub fn get_integer(&self, index: usize) -> Option<i64> {
        return match self.data.get(index) {
            Some(datum) => match Cursor::new(datum).read_i64::<LittleEndian>() {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            None => None,
        };
    }

    pub fn get_decimal(&self, index: usize) -> Option<f64> {
        return match self.data.get(index) {
            Some(datum) => match Cursor::new(datum).read_f64::<LittleEndian>() {
                Ok(val) => Some(val),
                Err(_) => None,
            },
            None => None,
        };
    }

    pub fn get_string(&self, index: usize) -> Option<String> {
        return match self.data.get(index) {
            Some(datum) => {
                let cow = String::from_utf8_lossy(datum);
                Some(cow.to_owned().to_string())
            }
            None => None,
        };
    }

    pub fn get_boolean(&self, index: usize) -> Option<bool> {
        return match self.data.get(index) {
            Some(datum) => match datum.get(0) {
                Some(num) => match num {
                    0 => Some(false),
                    1 => Some(true),
                    _ => None,
                },
                None => None,
            },
            None => None,
        };
    }
}
