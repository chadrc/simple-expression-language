use std::vec::Vec;

#[derive(Debug)]
pub struct SELSubTree {
    root: Option<usize>,
    sub_roots: Vec<usize>,
}

impl SELSubTree {
    pub fn new(root: Option<usize>, sub_roots: Vec<usize>) -> Self {
        return SELSubTree { root, sub_roots };
    }

    pub fn get_root(&self) -> Option<usize> {
        return self.root;
    }

    pub fn get_sub_roots(&self) -> &Vec<usize> {
        return &self.sub_roots;
    }
}
