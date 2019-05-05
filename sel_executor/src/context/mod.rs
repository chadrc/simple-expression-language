use crate::opexec::execution_result::SELExecutionResult;
use sel_common::{SELContext, SELFunction, SELValue};
use std::collections::HashMap;

#[derive(Clone)]
pub struct SELExecutionContext {
    input: Option<SELValue>,
    results: Vec<SELExecutionResult>,
    functions: HashMap<String, SELFunction>,
}

impl SELExecutionContext {
    pub fn new() -> Self {
        return SELExecutionContext {
            input: None,
            results: vec![],
            functions: HashMap::new(),
        };
    }

    pub fn from(context: &SELContext) -> Self {
        return SELExecutionContext {
            input: None,
            results: vec![],
            functions: context.get_functions().clone(),
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

    pub fn get_function(&self, name: &str) -> Option<&SELFunction> {
        return self.functions.get(name);
    }
}
