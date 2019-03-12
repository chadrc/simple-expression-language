mod context;
mod opexec;

pub use context::SELContext;
pub use context::SELValue;
pub use opexec::SELExecutionResult;
use sel_common::{DataType, SELTree};

pub fn execute_sel_tree(tree: &SELTree, context: &SELContext) -> SELExecutionResult {
    if tree.get_nodes().len() > 0 {
        return opexec::get_node_result(tree, tree.get_root(), context);
    }

    return SELExecutionResult::new(DataType::Unknown, None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use sel_common::{DataHeap, DataType, SELTree};

    #[test]
    fn executes_empty() {
        let tree = SELTree::new(0, vec![], vec![], DataHeap::new());

        let context = context::SELContext::new();

        let result = execute_sel_tree(&tree, &context);

        assert_eq!(result.get_type(), DataType::Unknown);
        assert_eq!(result.get_value(), None);
    }
}
