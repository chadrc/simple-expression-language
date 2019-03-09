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

fn concat_results<L: FromByteVec + ToString, R: FromByteVec + ToString>(
    left: &SELExecutionResult,
    right: &SELExecutionResult,
) -> SELExecutionResult {
    let (left_val, right_val) = get_values_from_results::<L, R>(left, right);

    let result = left_val.to_string() + &right_val.to_string();

    SELExecutionResult::new(DataType::String, Some(to_byte_vec(&result)))
}

pub fn addition_operation(tree: &SELTree, node: &SELTreeNode) -> SELExecutionResult {
    let left = tree.get_nodes().get(node.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(node.get_right().unwrap()).unwrap();

    let left_result = get_node_result(tree, &left);
    let right_result = get_node_result(tree, &right);

    return match (left_result.get_type(), right_result.get_type()) {
        (DataType::Integer, DataType::Integer) => {
            let (left_val, right_val) =
                get_values_from_results::<i32, i32>(&left_result, &right_result);

            let result = left_val + right_val;

            SELExecutionResult::new(DataType::Integer, Some(to_byte_vec(result)))
        }
        (DataType::Integer, DataType::Decimal) => {
            let (left_val, right_val) =
                get_values_from_results::<i32, f64>(&left_result, &right_result);

            let result = f64::from(left_val) + right_val;

            SELExecutionResult::new(DataType::Decimal, Some(to_byte_vec(result)))
        }
        (DataType::Decimal, DataType::Integer) => {
            let (left_val, right_val) =
                get_values_from_results::<f64, i32>(&left_result, &right_result);

            let result = left_val + f64::from(right_val);

            SELExecutionResult::new(DataType::Decimal, Some(to_byte_vec(result)))
        }
        (DataType::Decimal, DataType::Decimal) => {
            let (left_val, right_val) =
                get_values_from_results::<f64, f64>(&left_result, &right_result);

            let result = left_val + right_val;

            SELExecutionResult::new(DataType::Decimal, Some(to_byte_vec(result)))
        }
        (DataType::String, DataType::String) => {
            concat_results::<String, String>(&left_result, &right_result)
        }
        (DataType::String, DataType::Integer) => {
            concat_results::<String, i32>(&left_result, &right_result)
        }
        (DataType::Integer, DataType::String) => {
            concat_results::<i32, String>(&left_result, &right_result)
        }
        (DataType::String, DataType::Decimal) => {
            concat_results::<String, f64>(&left_result, &right_result)
        }
        (DataType::Decimal, DataType::String) => {
            concat_results::<f64, String>(&left_result, &right_result)
        }
        (DataType::String, DataType::Boolean) => {
            concat_results::<String, bool>(&left_result, &right_result)
        }
        (DataType::Boolean, DataType::String) => {
            concat_results::<bool, String>(&left_result, &right_result)
        }
        (_, DataType::Unit) | (DataType::Unit, _) => SELExecutionResult::new(DataType::Unit, None),
        _ => SELExecutionResult::new(DataType::Unknown, Some(vec![])),
    };
}

#[cfg(test)]
mod tests {
    use super::super::get_node_result;
    use super::*;
    use sel_common::{from_byte_vec, DataHeap, DataType, Operation, SELTree, SELTreeNode};

    #[test]
    fn executes_integer_addition() {
        let result = result_of_binary_op(
            Operation::Addition,
            DataType::Integer,
            "9",
            DataType::Integer,
            "5",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(result_value, Some(14));
    }

    #[test]
    fn executes_integer_decimal_addition() {
        let result = result_of_binary_op(
            Operation::Addition,
            DataType::Integer,
            "9",
            DataType::Decimal,
            "3.14",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Decimal);
        assert_eq!(result_value, Some(12.14));
    }

    #[test]
    fn executes_decimal_integer_addition() {
        let result = result_of_binary_op(
            Operation::Addition,
            DataType::Decimal,
            "3.14",
            DataType::Integer,
            "9",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Decimal);
        assert_eq!(result_value, Some(12.14));
    }

    #[test]
    fn executes_decimal_addition() {
        let result = result_of_binary_op(
            Operation::Addition,
            DataType::Decimal,
            "3.14",
            DataType::Decimal,
            "6.45",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Decimal);
        assert_eq!(result_value, Some(9.59));
    }

    #[test]
    fn executes_string_addition() {
        let result = result_of_binary_op(
            Operation::Addition,
            DataType::String,
            "Hello ",
            DataType::String,
            "World",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::String);
        assert_eq!(result_value, Some(String::from("Hello World")));
    }

    #[test]
    fn executes_string_integer_addition() {
        let result = result_of_binary_op(
            Operation::Addition,
            DataType::String,
            "Number: ",
            DataType::Integer,
            "6",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::String);
        assert_eq!(result_value, Some(String::from("Number: 6")));
    }

    #[test]
    fn executes_integer_string_addition() {
        let result = result_of_binary_op(
            Operation::Addition,
            DataType::Integer,
            "6",
            DataType::String,
            "Number: ",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::String);
        assert_eq!(result_value, Some(String::from("6Number: ")));
    }

    #[test]
    fn executes_string_decimal_addition() {
        let result = result_of_binary_op(
            Operation::Addition,
            DataType::String,
            "Number: ",
            DataType::Decimal,
            "3.14",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::String);
        assert_eq!(result_value, Some(String::from("Number: 3.14")));
    }

    #[test]
    fn executes_decimal_string_addition() {
        let result = result_of_binary_op(
            Operation::Addition,
            DataType::Decimal,
            "3.14",
            DataType::String,
            "Number: ",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::String);
        assert_eq!(result_value, Some(String::from("3.14Number: ")));
    }

    #[test]
    fn executes_boolean_string_addition() {
        let result = result_of_binary_op(
            Operation::Addition,
            DataType::Boolean,
            "true",
            DataType::String,
            " is True",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::String);
        assert_eq!(result_value, Some(String::from("true is True")));
    }

    #[test]
    fn executes_string_boolean_addition() {
        let result = result_of_binary_op(
            Operation::Addition,
            DataType::String,
            "this is ",
            DataType::Boolean,
            "true",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::String);
        assert_eq!(result_value, Some(String::from("this is true")));
    }

    #[test]
    fn executes_integer_unit_addition() {
        let result = result_of_binary_op(
            Operation::Addition,
            DataType::Integer,
            "9",
            DataType::Unit,
            "()",
        );

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }

    #[test]
    fn executes_unit_integer_addition() {
        let result = result_of_binary_op(
            Operation::Addition,
            DataType::Unit,
            "()",
            DataType::Integer,
            "9",
        );

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }

    fn result_of_binary_op(
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
