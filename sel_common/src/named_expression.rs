#[derive(Debug)]
pub struct NamedExpression {
    root: usize,
    symbol_index: usize,
}

impl NamedExpression {
    pub fn new(root: usize, symbol_index: usize) -> Self {
        return NamedExpression { root, symbol_index };
    }

    pub fn get_root(&self) -> usize {
        return self.root;
    }

    pub fn get_symbol(&self) -> usize {
        return self.symbol_index;
    }
}
