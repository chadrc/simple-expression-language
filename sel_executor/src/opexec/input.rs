use super::{SELContext, SELExecutionResult};
use sel_common::{DataType, SELTree, SELTreeNode};

pub fn operation(_tree: &SELTree, _node: &SELTreeNode, context: &SELContext) -> SELExecutionResult {
    return match context.get_input() {
        Some(input) => SELExecutionResult::new(
            input.get_type(),
            match input.get_value() {
                Some(value) => Some(std::vec::Vec::from(value.as_slice())),
                None => None,
            },
        ),
        None => SELExecutionResult::new(DataType::Unit, None),
    };
}

#[cfg(test)]
mod tests {
    use super::super::super::context::SELValue;
    use super::super::{get_node_result, SELContext};
    use sel_common::{from_byte_vec, DataHeap, DataType, Operation, SELTree, SELTreeNode};

    #[test]
    fn executes_input() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let heap = DataHeap::new();

        let root = SELTreeNode::new(Operation::Input, DataType::Unknown, 0, None);

        nodes.push(root);

        let tree = SELTree::new(0, nodes, heap);

        let mut context = SELContext::new();

        let input = SELValue::new_from_int(100);

        context.set_input(input);

        let result = get_node_result(&tree, tree.get_root(), &context);

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(result_value, Some(100));
    }

    #[test]
    fn executes_empty_input() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let heap = DataHeap::new();

        let root = SELTreeNode::new(Operation::Input, DataType::Unknown, 0, None);

        nodes.push(root);

        let tree = SELTree::new(0, nodes, heap);

        let context = SELContext::new();

        let result = get_node_result(&tree, tree.get_root(), &context);

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }

    // #[test]
    // fn executes_addition_op_with_input() {
    //     let result = result_of_binary_op(
    //         Operation::Addition,
    //         DataType::Integer,
    //         "9",
    //         DataType::Integer,
    //         "5",
    //     );

    //     let result_value = match result.get_value() {
    //         Some(value) => Some(from_byte_vec(value)),
    //         None => None,
    //     };

    //     assert_eq!(result.get_type(), DataType::Integer);
    //     assert_eq!(result_value, Some(14));
    // }
}
