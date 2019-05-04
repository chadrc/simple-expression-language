#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Expression {
    root: usize,
}

impl Expression {
    pub fn new(root: usize) -> Self {
        return Expression { root };
    }

    pub fn get_root(&self) -> usize {
        return self.root;
    }
}
