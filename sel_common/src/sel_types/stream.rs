use crate::sel_types::list::List;
use crate::{from_byte_vec, DataType, SELValue};
use std::path::Iter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SELStream {
    source: SELValue,
    processor: usize,
}

impl SELStream {
    pub fn new(source: SELValue, processor: usize) -> Self {
        return SELStream { source, processor };
    }

    pub fn empty() -> Self {
        return SELStream {
            source: SELValue::new(),
            processor: 0,
        };
    }

    pub fn iter(&self) -> SELStreamIterator {
        return SELStreamIterator {
            source: self.source.clone(),
            current: 0,
        };
    }

    pub fn get_processor_root(&self) -> usize {
        return self.processor;
    }
}

pub struct SELStreamIterator {
    source: SELValue,
    current: usize,
}

impl Iterator for SELStreamIterator {
    type Item = SELValue;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.get_type() {
            DataType::List => {
                let list: List = from_byte_vec(self.source.get_value().unwrap());

                let val = list.get_values().get(self.current).map(|v| v.to_owned());

                self.current += 1;

                val
            }
            _ => None,
        }
    }
}
