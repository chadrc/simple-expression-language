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

#[cfg(test)]
mod tests {
    use super::super::get_node_result;
    use sel_common::{from_byte_vec, DataHeap, DataType, Operation, SELTree, SELTreeNode};

    #[test]
    fn executes_unit_touch() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        nodes.push(SELTreeNode::new(Operation::Touch, DataType::Unit, 0, None));

        let tree = SELTree::new(0, nodes, DataHeap::new());

        let result = get_node_result(&tree, tree.get_root());

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }

    #[test]
    fn executes_integer_touch() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let value = heap.insert_from_string(DataType::Integer, &String::from("9"));
        nodes.push(SELTreeNode::new(
            Operation::Touch,
            DataType::Integer,
            0,
            value,
        ));

        let tree = SELTree::new(0, nodes, heap);

        let result = get_node_result(&tree, tree.get_root());

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(result_value, Some(9));
    }

    #[test]
    fn executes_decimal_touch() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let value = heap.insert_from_string(DataType::Decimal, &String::from("3.14"));
        nodes.push(SELTreeNode::new(
            Operation::Touch,
            DataType::Decimal,
            0,
            value,
        ));

        let tree = SELTree::new(0, nodes, heap);

        let result = get_node_result(&tree, tree.get_root());

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Decimal);
        assert_eq!(result_value, Some(3.14));
    }

    #[test]
    fn executes_string_touch() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let value = heap.insert_from_string(DataType::String, &String::from("Hello World"));
        nodes.push(SELTreeNode::new(
            Operation::Touch,
            DataType::String,
            0,
            value,
        ));

        let tree = SELTree::new(0, nodes, heap);

        let result = get_node_result(&tree, tree.get_root());

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::String);
        assert_eq!(result_value, Some(String::from("Hello World")));
    }

    #[test]
    fn executes_boolean_touch() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let value = heap.insert_from_string(DataType::Boolean, &String::from("true"));
        nodes.push(SELTreeNode::new(
            Operation::Touch,
            DataType::Boolean,
            0,
            value,
        ));

        let tree = SELTree::new(0, nodes, heap);

        let result = get_node_result(&tree, tree.get_root());

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(true));
    }
}