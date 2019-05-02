use super::SELExecutionContext;
use crate::opexec::execution_result::SELExecutionResult;
use crate::opexec::get_node_result;
use crate::opexec::utils::get_left_right_results;
use sel_common::{
    from_byte_vec, to_byte_vec, DataType, List, Operation, SELTree, SELTreeNode, SELValue,
};

pub fn pipe_first_right_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

pub fn pipe_first_left_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

pub fn pipe_last_right_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

pub fn pipe_last_left_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

#[cfg(test)]
mod tests {
    use super::super::{get_node_result, SELExecutionContext};
    use sel_common::{
        from_byte_vec, DataHeap, DataType, List, Operation, SELContext, SELTree, SELTreeNode,
        SELValue, SymbolTable,
    };
    use sel_compiler::Compiler;

    #[test]
    fn executes_pipe_first_right() {}
}
