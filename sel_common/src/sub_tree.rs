use std::vec::Vec;

#[derive(Debug)]
pub struct SELSubTree {
    roots: Vec<usize>,
}

impl SELSubTree {
    pub fn new(sub_roots: Vec<usize>) -> Self {
        return SELSubTree { roots: sub_roots };
    }

    pub fn get_roots(&self) -> &Vec<usize> {
        return &self.roots;
    }
}
