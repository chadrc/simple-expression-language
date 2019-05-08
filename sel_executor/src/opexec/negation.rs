use super::super::context::SELExecutionContext;
use super::{get_node_result, SELExecutionResult};
use sel_common::{from_byte_vec, to_byte_vec, DataType, SELTree, SELTreeNode};

pub fn operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    let right = tree.get_nodes().get(node.get_right().unwrap()).unwrap();
    let result = get_node_result(tree, &right, context);

    return match result.get_type() {
        DataType::Integer => {
            let right_val: Option<i32> = match result.get_value() {
                Some(value) => Some(from_byte_vec(value)),
                None => None,
            };

            let val = -(right_val.unwrap());

            SELExecutionResult::new(DataType::Integer, Some(to_byte_vec(val)))
        }
        DataType::Decimal => {
            let right_val: Option<f64> = match result.get_value() {
                Some(value) => Some(from_byte_vec(value)),
                None => None,
            };

            let val = -(right_val.unwrap());

            SELExecutionResult::new(DataType::Decimal, Some(to_byte_vec(val)))
        }
        _ => SELExecutionResult::new(DataType::Unit, None),
    };
}

#[cfg(test)]
mod tests {
    use super::super::get_node_result;
    use super::super::test_utils::result_of_binary_op;
    use super::*;
    use sel_common::{
        from_byte_vec, DataHeap, DataType, Operation, SELContext, SELTree, SELTreeNode,
    };

    #[test]
    fn executes_integer_negation() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let mut right = SELTreeNode::new(
            Operation::Touch,
            DataType::Integer,
            0,
            heap.insert_from_string(DataType::Integer, &String::from("5")),
        );

        let mut root = SELTreeNode::new(Operation::Negation, DataType::Unknown, 1, None);

        right.set_parent(Some(1));

        root.set_left(None);
        root.set_right(Some(0));

        nodes.push(right);
        nodes.push(root);

        let tree = SELTree::new(
            1,
            vec![],
            vec![],
            nodes,
            heap,
            SELContext::new(),
            vec![],
            vec![],
        );

        let context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &context);

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(result_value, Some(-5));
    }

    #[test]
    fn executes_decimal_negation() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let mut right = SELTreeNode::new(
            Operation::Touch,
            DataType::Decimal,
            0,
            heap.insert_from_string(DataType::Decimal, &String::from("3.14")),
        );

        let mut root = SELTreeNode::new(Operation::Negation, DataType::Unknown, 1, None);

        right.set_parent(Some(1));

        root.set_left(None);
        root.set_right(Some(0));

        nodes.push(right);
        nodes.push(root);

        let tree = SELTree::new(
            1,
            vec![],
            vec![],
            nodes,
            heap,
            SELContext::new(),
            vec![],
            vec![],
        );

        let context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &context);

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Decimal);
        assert_eq!(result_value, Some(-3.14));
    }

    #[test]
    fn executes_integer_unit_negation() {
        let result = result_of_binary_op(
            Operation::Negation,
            DataType::Integer,
            "9",
            DataType::Unit,
            "()",
        );

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }
}
