mod access;
mod addition;
mod associative_list;
mod bitwise;
mod division;
mod equality;
mod execution_result;
mod exponential;
mod greater_than;
mod greater_than_equal;
mod group;
mod inequality;
mod input;
mod keys_equal;
mod less_than;
mod less_than_equal;
mod list;
mod logical;
mod logical_not;
mod modulo;
mod multiplication;
mod negation;
mod pair;
mod pipe;
mod range;
mod result;
mod subtraction;
mod touch;
mod utils;

use super::context::SELExecutionContext;
pub use execution_result::SELExecutionResult;
use sel_common::{DataType, Operation, SELTree, SELTreeNode};

pub fn get_node_result(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return match node.get_operation() {
        Operation::Touch => touch::operation(tree, node, context),
        Operation::Input => input::operation(tree, node, context),
        Operation::CurrentResult => result::operation(tree, node, context),
        Operation::Addition => addition::operation(tree, node, context),
        Operation::Subtraction => subtraction::operation(tree, node, context),
        Operation::Multiplication => multiplication::operation(tree, node, context),
        Operation::Division => division::division_operation(tree, node, context),
        Operation::IntegerDivision => division::integer_division_operation(tree, node, context),
        Operation::Modulo => modulo::operation(tree, node, context),
        Operation::Exponential => exponential::operation(tree, node, context),
        Operation::Negation => negation::operation(tree, node, context),
        Operation::LogicalOr => logical::or_operation(tree, node, context),
        Operation::LogicalXOR => logical::xor_operation(tree, node, context),
        Operation::LogicalAnd => logical::and_operation(tree, node, context),
        Operation::Not => logical_not::operation(tree, node, context),
        Operation::BitwiseOr => bitwise::or_operation(tree, node, context),
        Operation::BitwiseAnd => bitwise::and_operation(tree, node, context),
        Operation::BitwiseXOR => bitwise::xor_operation(tree, node, context),
        Operation::BitwiseLeftShift => bitwise::left_shift_operation(tree, node, context),
        Operation::BitwiseRightShift => bitwise::right_shift_operation(tree, node, context),
        Operation::GreaterThan => greater_than::operation(tree, node, context),
        Operation::GreaterThanOrEqual => greater_than_equal::operation(tree, node, context),
        Operation::LessThan => less_than::operation(tree, node, context),
        Operation::LessThanOrEqual => less_than_equal::operation(tree, node, context),
        Operation::Equality => equality::operation(tree, node, context),
        Operation::Inequality => inequality::operation(tree, node, context),
        Operation::InclusiveRange => range::inclusive_range_operation(tree, node, context),
        Operation::ExclusiveRange => range::exclusive_range_operation(tree, node, context),
        Operation::Pair => pair::operation(tree, node, context),
        Operation::List => list::operation(tree, node, context),
        Operation::DotAccess => access::dot_access_operation(tree, node, context),
        Operation::PipeFirstRight => pipe::pipe_first_right_operation(tree, node, context),
        Operation::PipeFirstLeft => pipe::pipe_first_left_operation(tree, node, context),
        Operation::PipeLastRight => pipe::pipe_last_right_operation(tree, node, context),
        Operation::PipeLastLeft => pipe::pipe_last_left_operation(tree, node, context),
        Operation::Group => group::operation(tree, node, context),
        Operation::AssociativeList => associative_list::operation(tree, node, context),
        Operation::KeysEqual => keys_equal::equal_operation(tree, node, context),
        Operation::KeysNotEqual => keys_equal::not_equal_operation(tree, node, context),
        _ => SELExecutionResult::new(DataType::Unknown, None),
    };
}

#[cfg(test)]
pub mod test_utils {
    use super::super::context;
    use super::*;
    use sel_common::{DataHeap, DataType, Operation, SELContext, SELTree, SELTreeNode};

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

        let tree = SELTree::new(2, vec![], nodes, heap, SELContext::new());

        let context = context::SELExecutionContext::new();

        return get_node_result(&tree, tree.get_root(), &context);
    }
}
