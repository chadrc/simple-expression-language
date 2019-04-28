use super::opexec::SELExecutionResult;
use sel_common::SELValue;

#[derive(Clone)]
pub struct SELExecutionContext {
    input: Option<SELValue>,
    results: Vec<SELExecutionResult>,
}

impl SELExecutionContext {
    pub fn new() -> Self {
        return SELExecutionContext {
            input: None,
            results: vec![],
        };
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

    pub fn push_result(&mut self, result: SELExecutionResult) {
        self.results.push(result);
    }

    pub fn get_results(&self) -> &Vec<SELExecutionResult> {
        return &self.results;
    }
}
