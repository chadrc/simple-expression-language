use super::sel_tree_builder::build_tree_from_string;
use sel_common::SELTree;

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        return Compiler {};
    }

    pub fn compile(&self, s: &String) -> SELTree {
        return build_tree_from_string(s);
    }
}
