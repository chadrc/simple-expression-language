mod context;
mod opexec;
mod tests;

pub use context::SELContext;
pub use context::SELValue;
pub use opexec::SELExecutionResult;
use sel_common::{DataType, SELTree};

pub fn execute_sel_tree(tree: &SELTree, context: &SELContext) -> Vec<SELExecutionResult> {
    let mut results: Vec<SELExecutionResult> = vec![];

    if tree.get_nodes().len() > 0 {
        results.push(opexec::get_node_result(tree, tree.get_root(), context));

        for sub_root_index in 0..tree.get_sub_roots().len() {
            match tree.get_sub_root(sub_root_index) {
                Some(sub_root) => results.push(opexec::get_node_result(tree, sub_root, context)),
                None => (),
            }
        }
    } else {
        results.push(SELExecutionResult::new(DataType::Unknown, None));
    }

    return results;
}
