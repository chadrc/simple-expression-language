mod sel_value;

pub use sel_value::SELValue;

pub struct SELContext {
    input: Option<SELValue>,
}

impl SELContext {
    pub fn new() -> Self {
        return SELContext { input: None };
    }

    pub fn set_input(&mut self, value: SELValue) {
        self.input = Some(value);
    }

    pub fn get_input(&self) -> Option<&SELValue> {
        return match &self.input {
            Some(val) => Some(&val),
            None => None,
        };
    }
}
