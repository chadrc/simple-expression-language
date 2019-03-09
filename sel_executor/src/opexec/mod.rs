mod addition;
mod execution_result;
mod touch;

pub use execution_result::SELExecutionResult;
use sel_common::{DataType, Operation, SELTree, SELTreeNode};
pub use touch::touch_operation;

pub fn get_node_result(tree: &SELTree, node: &SELTreeNode) -> SELExecutionResult {
    return match node.get_operation() {
        Operation::Touch => touch::touch_operation(tree, node),
        Operation::Addition => addition::addition_operation(tree, node),
        _ => SELExecutionResult::new(DataType::Unknown, None),
    };
}
