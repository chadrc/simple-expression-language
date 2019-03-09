use super::get_node_result;
use super::utils::get_values_from_results;
use super::SELExecutionResult;
use sel_common::{to_byte_vec, DataType, SELTree, SELTreeNode};

pub fn subtraction_operation(tree: &SELTree, node: &SELTreeNode) -> SELExecutionResult {
    let left = tree.get_nodes().get(node.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(node.get_right().unwrap()).unwrap();

    let left_result = get_node_result(tree, &left);
    let right_result = get_node_result(tree, &right);

    return match (left_result.get_type(), right_result.get_type()) {
        (DataType::Integer, DataType::Integer) => {
            let (left_val, right_val) =
                get_values_from_results::<i32, i32>(&left_result, &right_result);

            let result = left_val - right_val;

            SELExecutionResult::new(DataType::Integer, Some(to_byte_vec(result)))
        }
        (DataType::Integer, DataType::Decimal) => {
            let (left_val, right_val) =
                get_values_from_results::<i32, f64>(&left_result, &right_result);

            let result = f64::from(left_val) - right_val;

            SELExecutionResult::new(DataType::Decimal, Some(to_byte_vec(result)))
        }
        (DataType::Decimal, DataType::Integer) => {
            let (left_val, right_val) =
                get_values_from_results::<f64, i32>(&left_result, &right_result);

            let result = left_val - f64::from(right_val);

            SELExecutionResult::new(DataType::Decimal, Some(to_byte_vec(result)))
        }
        (DataType::Decimal, DataType::Decimal) => {
            let (left_val, right_val) =
                get_values_from_results::<f64, f64>(&left_result, &right_result);

            let result = left_val - right_val;

            SELExecutionResult::new(DataType::Decimal, Some(to_byte_vec(result)))
        }
        _ => SELExecutionResult::new(DataType::Unknown, Some(vec![])),
    };
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::result_of_binary_op;
    use sel_common::{from_byte_vec, DataType, Operation};

    #[test]
    fn executes_integer_subtraction() {
        let result = result_of_binary_op(
            Operation::Subtraction,
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
        assert_eq!(result_value, Some(4));
    }

    #[test]
    fn executes_integer_decimal_subtraction() {
        let result = result_of_binary_op(
            Operation::Subtraction,
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
        assert_eq!(result_value, Some(9.0 - 3.14));
    }

    #[test]
    fn executes_decimal_integer_addition() {
        let result = result_of_binary_op(
            Operation::Subtraction,
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
        assert_eq!(result_value, Some(3.14 - 9.0));
    }

    #[test]
    fn executes_decimal_addition() {
        let result = result_of_binary_op(
            Operation::Subtraction,
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
        assert_eq!(result_value, Some(3.14 - 6.45));
    }
}
