mod context;
mod opexec;

#[cfg(test)]
mod tests;

use crate::opexec::execution_result::SELExecutionResult;
pub use context::SELExecutionContext;
use sel_common::{DataType, SELTree};

pub fn execute_sel_tree(tree: &SELTree, context: &SELExecutionContext) -> Vec<SELExecutionResult> {
    let mut current_context = context.clone();

    if tree.get_nodes().len() > 0 {
        let result = opexec::get_node_result(tree, tree.get_root(), &mut current_context);

        current_context.push_result(result);

        for sub_root_index in 0..tree.get_sub_roots().len() {
            match tree.get_sub_root(sub_root_index) {
                Some(sub_root) => {
                    let result = opexec::get_node_result(tree, sub_root, &mut current_context);

                    current_context.push_result(result);
                }
                None => (),
            }
        }
    } else {
        current_context.push_result(SELExecutionResult::new(DataType::Unknown, None));
    }

    return current_context.get_results().clone();
}
