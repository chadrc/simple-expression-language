#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Expression {
    root: Option<usize>,
}

impl Expression {
    pub fn new(root: Option<usize>) -> Self {
        return Expression { root };
    }

    pub fn get_root(&self) -> Option<usize> {
        return self.root;
    }
}
