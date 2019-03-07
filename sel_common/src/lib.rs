mod data_type;
mod operation;
mod sel_tree;

pub use data_type::DataType;
pub use operation::Operation;
pub use sel_tree::{NodeSide, SELTree, SELTreeNode};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
