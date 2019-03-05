use super::sel_tree::SELTree;
use super::sel_tree_builder::SELTreeBuilder;

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        return Compiler {};
    }

    pub fn compile(&self, s: &String) -> SELTree {
        let mut builder = SELTreeBuilder::new();
        return builder.build(s);
    }
}
