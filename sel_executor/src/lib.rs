mod context;
mod opexec;

pub use opexec::SELExecutionResult;
use sel_common::{DataType, SELTree};

pub fn execute_sel_tree(tree: SELTree) -> SELExecutionResult {
    if tree.get_nodes().len() > 0 {
        let root = tree.get_root();

        let context = context::SELContext::new();

        return opexec::get_node_result(&tree, &root, &context);
    }

    return SELExecutionResult::new(DataType::Unknown, None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use sel_common::{DataHeap, DataType, SELTree};

    #[test]
    fn executes_empty() {
        let tree = SELTree::new(0, vec![], DataHeap::new());

        let result = execute_sel_tree(tree);

        assert_eq!(result.get_type(), DataType::Unknown);
        assert_eq!(result.get_value(), None);
    }
}
