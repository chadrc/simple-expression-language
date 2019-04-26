use super::execution_result::SELExecutionResult;
use super::{get_node_result, SELContext};
use sel_common::{DataType, SELTree, SELTreeNode};

pub fn exclusive_range_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    _context: &SELContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

pub fn inclusive_range_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    _context: &SELContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

#[cfg(test)]
mod tests {
    use super::super::get_node_result;
    use super::*;
    use sel_common::{from_byte_vec, DataHeap, DataType, Operation, SELTree, SELTreeNode};
}
