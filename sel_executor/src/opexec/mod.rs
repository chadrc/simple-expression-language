mod addition;
mod division;
mod equality;
mod execution_result;
mod exponential;
mod greater_than;
mod greater_than_equal;
mod inequality;
mod less_than;
mod less_than_equal;
mod logical;
mod modulo;
mod multiplication;
mod negation;
mod subtraction;
mod touch;
mod utils;

pub use execution_result::SELExecutionResult;
use sel_common::{DataType, Operation, SELTree, SELTreeNode};
pub use touch::touch_operation;

pub fn get_node_result(tree: &SELTree, node: &SELTreeNode) -> SELExecutionResult {
    return match node.get_operation() {
        Operation::Touch => touch::touch_operation(tree, node),
        Operation::Addition => addition::addition_operation(tree, node),
        Operation::Subtraction => subtraction::subtraction_operation(tree, node),
        Operation::Multiplication => multiplication::multiplication_operation(tree, node),
        Operation::Division => division::division_operation(tree, node),
        Operation::Modulo => modulo::modulo_operation(tree, node),
        Operation::Exponential => exponential::exponential_operation(tree, node),
        Operation::Negation => negation::negation_operation(tree, node),
        Operation::LogicalOr => logical::logical_or_operation(tree, node),
        Operation::LogicalAnd => logical::logical_and_operation(tree, node),
        Operation::GreaterThan => greater_than::greater_than_operation(tree, node),
        Operation::GreaterThanOrEqual => {
            greater_than_equal::greater_than_equal_operation(tree, node)
        }
        Operation::LessThan => less_than::operation(tree, node),
        Operation::LessThanOrEqual => less_than_equal::operation(tree, node),
        Operation::Equality => equality::operation(tree, node),
        Operation::Inequality => inequality::operation(tree, node),
        _ => SELExecutionResult::new(DataType::Unknown, None),
    };
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use sel_common::{DataHeap, DataType, Operation, SELTree, SELTreeNode};

    pub fn result_of_binary_op(
        op: Operation,
        left_type: DataType,
        left_value: &str,
        right_type: DataType,
        right_value: &str,
    ) -> SELExecutionResult {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let mut left = SELTreeNode::new(
            Operation::Touch,
            left_type,
            0,
            heap.insert_from_string(left_type, &String::from(left_value)),
        );

        let mut right = SELTreeNode::new(
            Operation::Touch,
            right_type,
            1,
            heap.insert_from_string(right_type, &String::from(right_value)),
        );

        let mut root = SELTreeNode::new(op, DataType::Unknown, 2, None);

        left.set_parent(Some(2));
        right.set_parent(Some(2));

        root.set_left(Some(0));
        root.set_right(Some(1));

        nodes.push(left);
        nodes.push(right);
        nodes.push(root);

        let tree = SELTree::new(2, nodes, heap);

        return get_node_result(&tree, tree.get_root());
    }
}
