use sel_common::DataType;

pub struct SELExecutionResult {
    data_type: DataType,
    value: Option<Vec<u8>>,
}

impl SELExecutionResult {
    pub fn new(data_type: DataType, value: Option<Vec<u8>>) -> Self {
        return SELExecutionResult {
            data_type: data_type,
            value: value,
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
