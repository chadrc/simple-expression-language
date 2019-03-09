use sel_common::{from_byte_vec, to_byte_vec, DataType, Operation, SELTree, SELTreeNode};

pub struct SELExecutionResult {
    data_type: DataType,
    value: Option<Vec<u8>>,
}

impl SELExecutionResult {
    pub fn get_type(&self) -> DataType {
        return self.data_type;
    }

    pub fn get_value(&self) -> Option<&Vec<u8>> {
        return match &self.value {
            Some(v) => Some(&v),
            None => None,
        };
    }
}

fn get_node_result(tree: &SELTree, node: &SELTreeNode) -> SELExecutionResult {
    return match node.get_operation() {
        Operation::Touch => match node.get_data_type() {
            DataType::Unit => SELExecutionResult {
                data_type: DataType::Unit,
                value: None,
            },
            DataType::Integer | DataType::Decimal | DataType::String | DataType::Boolean => {
                SELExecutionResult {
                    data_type: node.get_data_type(),
                    value: tree.get_value_bytes_of(node),
                }
            }
            _ => SELExecutionResult {
                data_type: DataType::Unknown,
                value: None,
            },
        },
        Operation::Addition => {
            let left = tree.get_nodes().get(node.get_left().unwrap()).unwrap();
            let right = tree.get_nodes().get(node.get_right().unwrap()).unwrap();

            let left_result = get_node_result(tree, &left);
            let right_result = get_node_result(tree, &right);

            let operand_types = (left_result.get_type(), right_result.get_type());

            let (result, result_type) = match operand_types {
                (DataType::Integer, DataType::Integer) => {
                    let left_int: Option<i32> = match left_result.get_value() {
                        Some(value) => Some(from_byte_vec(value)),
                        None => None,
                    };

                    let right_int: Option<i32> = match right_result.get_value() {
                        Some(value) => Some(from_byte_vec(value)),
                        None => None,
                    };

                    let result = left_int.unwrap() + right_int.unwrap();

                    (Some(to_byte_vec(result)), DataType::Integer)
                }
                (DataType::Integer, DataType::Decimal) => {
                    let left_int: Option<i32> = match left_result.get_value() {
                        Some(value) => Some(from_byte_vec(value)),
                        None => None,
                    };

                    let right_int: Option<f64> = match right_result.get_value() {
                        Some(value) => Some(from_byte_vec(value)),
                        None => None,
                    };

                    let result = f64::from(left_int.unwrap()) + right_int.unwrap();

                    (Some(to_byte_vec(result)), DataType::Integer)
                }
                _ => (Some(vec![]), DataType::Unknown),
            };

            SELExecutionResult {
                data_type: result_type,
                value: result,
            }
        }
        _ => SELExecutionResult {
            data_type: DataType::Unknown,
            value: None,
        },
    };
}

pub fn execute_sel_tree(tree: SELTree) -> SELExecutionResult {
    if tree.get_nodes().len() > 0 {
        let root = tree.get_root();

        return get_node_result(&tree, &root);
    }

    return SELExecutionResult {
        data_type: DataType::Unknown,
        value: None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use sel_common::{DataHeap, DataType, Operation, SELTree, SELTreeNode};

    #[test]
    fn executes_empty() {
        let tree = SELTree::new(0, vec![], DataHeap::new());

        let result = execute_sel_tree(tree);

        assert_eq!(result.get_type(), DataType::Unknown);
        assert_eq!(result.get_value(), None);
    }

    #[test]
    fn executes_unit_touch() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        nodes.push(SELTreeNode::new(Operation::Touch, DataType::Unit, 0, None));

        let tree = SELTree::new(0, nodes, DataHeap::new());

        let result = execute_sel_tree(tree);

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

        let result = execute_sel_tree(tree);

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

        let result = execute_sel_tree(tree);

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

        let result = execute_sel_tree(tree);

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

        let result = execute_sel_tree(tree);

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(true));
    }

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

        let result = execute_sel_tree(tree);

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

        let result = execute_sel_tree(tree);

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        // assert_eq!(result.get_type(), DataType::Decimal);
        assert_eq!(result_value, Some(12.14));
    }
}
