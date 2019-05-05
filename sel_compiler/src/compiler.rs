use crate::build::build_tree_from_string;
use sel_common::{SELContext, SELTree};

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        return Compiler {};
    }

    pub fn compile(&self, s: &String) -> SELTree {
        return build_tree_from_string(s, SELContext::new());
    }

    pub fn compile_with_context(&self, s: &String, context: SELContext) -> SELTree {
        return build_tree_from_string(s, context);
    }
}
