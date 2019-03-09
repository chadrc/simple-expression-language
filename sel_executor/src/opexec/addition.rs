use super::execution_result::SELExecutionResult;
use super::get_node_result;
use sel_common::{from_byte_vec, to_byte_vec, DataType, FromByteVec, SELTree, SELTreeNode};

fn get_values_from_results<L: FromByteVec, R: FromByteVec>(
    left: &SELExecutionResult,
    right: &SELExecutionResult,
) -> (L, R) {
    let left_val: Option<L> = match left.get_value() {
        Some(value) => Some(from_byte_vec(value)),
        None => None,
    };

    let right_val: Option<R> = match right.get_value() {
        Some(value) => Some(from_byte_vec(value)),
        None => None,
    };

    return (left_val.unwrap(), right_val.unwrap());
}

pub fn addition_operation(tree: &SELTree, node: &SELTreeNode) -> SELExecutionResult {
    let left = tree.get_nodes().get(node.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(node.get_right().unwrap()).unwrap();

    let left_result = get_node_result(tree, &left);
    let right_result = get_node_result(tree, &right);

    let operand_types = (left_result.get_type(), right_result.get_type());

    let (result, result_type) = match operand_types {
        (DataType::Integer, DataType::Integer) => {
            let (left_val, right_val) =
                get_values_from_results::<i32, i32>(&left_result, &right_result);

            let result = left_val + right_val;

            (Some(to_byte_vec(result)), DataType::Integer)
        }
        (DataType::Integer, DataType::Decimal) => {
            let (left_val, right_val) =
                get_values_from_results::<i32, f64>(&left_result, &right_result);

            let result = f64::from(left_val) + right_val;

            (Some(to_byte_vec(result)), DataType::Decimal)
        }
        (DataType::Decimal, DataType::Integer) => {
            let (left_val, right_val) =
                get_values_from_results::<f64, i32>(&left_result, &right_result);

            let result = left_val + f64::from(right_val);

            (Some(to_byte_vec(result)), DataType::Decimal)
        }
        (DataType::Decimal, DataType::Decimal) => {
            let (left_val, right_val) =
                get_values_from_results::<f64, f64>(&left_result, &right_result);

            let result = left_val + right_val;

            (Some(to_byte_vec(result)), DataType::Decimal)
        }
        _ => (Some(vec![]), DataType::Unknown),
    };

    return SELExecutionResult::new(result_type, result);
}

#[cfg(test)]
mod tests {
    use super::super::get_node_result;
    use sel_common::{from_byte_vec, DataHeap, DataType, Operation, SELTree, SELTreeNode};

    #[test]
    fn executes_integer_addition() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let mut left = SELTreeNode::new(
            Operation::Touch,
            DataType::Integer,
            0,
            heap.insert_from_string(DataType::Integer, &String::from("9")),
        );

        let mut right = SELTreeNode::new(
            Operation::Touch,
            DataType::Integer,
            1,
            heap.insert_from_string(DataType::Integer, &String::from("5")),
        );

        let mut root = SELTreeNode::new(Operation::Addition, DataType::Unknown, 2, None);

        left.set_parent(Some(2));
        right.set_parent(Some(2));

        root.set_left(Some(0));
        root.set_right(Some(1));

        nodes.push(left);
        nodes.push(right);
        nodes.push(root);

        let tree = SELTree::new(2, nodes, heap);

        let result = get_node_result(&tree, tree.get_root());

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(result_value, Some(14));
    }

    #[test]
    fn executes_integer_decimal_addition() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let mut left = SELTreeNode::new(
            Operation::Touch,
            DataType::Integer,
            0,
            heap.insert_from_string(DataType::Integer, &String::from("9")),
        );

        let mut right = SELTreeNode::new(
            Operation::Touch,
            DataType::Decimal,
            1,
            heap.insert_from_string(DataType::Decimal, &String::from("3.14")),
        );

        let mut root = SELTreeNode::new(Operation::Addition, DataType::Unknown, 2, None);

        left.set_parent(Some(2));
        right.set_parent(Some(2));

        root.set_left(Some(0));
        root.set_right(Some(1));

        nodes.push(left);
        nodes.push(right);
        nodes.push(root);

        let tree = SELTree::new(2, nodes, heap);

        let result = get_node_result(&tree, tree.get_root());

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Decimal);
        assert_eq!(result_value, Some(12.14));
    }

    #[test]
    fn executes_decimal_integer_addition() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let mut left = SELTreeNode::new(
            Operation::Touch,
            DataType::Decimal,
            0,
            heap.insert_from_string(DataType::Decimal, &String::from("3.14")),
        );

        let mut right = SELTreeNode::new(
            Operation::Touch,
            DataType::Integer,
            1,
            heap.insert_from_string(DataType::Integer, &String::from("9")),
        );

        let mut root = SELTreeNode::new(Operation::Addition, DataType::Unknown, 2, None);

        left.set_parent(Some(2));
        right.set_parent(Some(2));

        root.set_left(Some(0));
        root.set_right(Some(1));

        nodes.push(left);
        nodes.push(right);
        nodes.push(root);

        let tree = SELTree::new(2, nodes, heap);

        let result = get_node_result(&tree, tree.get_root());

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Decimal);
        assert_eq!(result_value, Some(12.14));
    }

    #[test]
    fn executes_decimal_addition() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let mut left = SELTreeNode::new(
            Operation::Touch,
            DataType::Decimal,
            0,
            heap.insert_from_string(DataType::Decimal, &String::from("3.14")),
        );

        let mut right = SELTreeNode::new(
            Operation::Touch,
            DataType::Decimal,
            1,
            heap.insert_from_string(DataType::Decimal, &String::from("6.45")),
        );

        let mut root = SELTreeNode::new(Operation::Addition, DataType::Unknown, 2, None);

        left.set_parent(Some(2));
        right.set_parent(Some(2));

        root.set_left(Some(0));
        root.set_right(Some(1));

        nodes.push(left);
        nodes.push(right);
        nodes.push(root);

        let tree = SELTree::new(2, nodes, heap);

        let result = get_node_result(&tree, tree.get_root());

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Decimal);
        assert_eq!(result_value, Some(9.59));
    }
}
