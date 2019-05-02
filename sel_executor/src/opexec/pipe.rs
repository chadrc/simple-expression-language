use super::SELExecutionContext;
use crate::opexec::execution_result::SELExecutionResult;
use sel_common::{DataType, SELTree, SELTreeNode};

pub fn pipe_first_right_operation(
    _tree: &SELTree,
    _node: &SELTreeNode,
    _context: &SELExecutionContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

pub fn pipe_first_left_operation(
    _tree: &SELTree,
    _node: &SELTreeNode,
    _context: &SELExecutionContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

pub fn pipe_last_right_operation(
    _tree: &SELTree,
    _node: &SELTreeNode,
    _context: &SELExecutionContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

pub fn pipe_last_left_operation(
    _tree: &SELTree,
    _node: &SELTreeNode,
    _context: &SELExecutionContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

#[cfg(test)]
mod tests {

    #[test]
    fn executes_pipe_first_right() {}
}
