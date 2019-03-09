use super::execution_result::SELExecutionResult;
use sel_common::{DataType, SELTree, SELTreeNode};

pub fn touch_operation(tree: &SELTree, node: &SELTreeNode) -> SELExecutionResult {
    return match node.get_data_type() {
        DataType::Unit => SELExecutionResult::new(DataType::Unit, None),
        DataType::Integer | DataType::Decimal | DataType::String | DataType::Boolean => {
            SELExecutionResult::new(node.get_data_type(), tree.get_value_bytes_of(node))
        }
        _ => SELExecutionResult::new(DataType::Unknown, None),
    };
}
