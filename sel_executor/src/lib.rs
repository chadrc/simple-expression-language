mod context;
mod opexec;
mod tests;

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
