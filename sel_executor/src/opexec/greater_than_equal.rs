use super::utils::{get_values_from_results, match_comparison_ops, OptionOr};
use super::SELExecutionResult;
use sel_common::{to_byte_vec, DataType, SELTree, SELTreeNode};

pub fn greater_than_equal_operation(tree: &SELTree, node: &SELTreeNode) -> SELExecutionResult {
    return match match_comparison_ops(
        tree,
        node,
        |left, right| left >= right,
        |left, right| left >= right,
    ) {
        OptionOr::Some(result) => result,
        OptionOr::Or((left, right)) => match (left.get_type(), right.get_type()) {
            (DataType::String, DataType::String) => {
                let (left_val, right_val) =
                    get_values_from_results::<String, String>(&left, &right);

                let result = left_val >= right_val;

                SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(result)))
            }
            _ => SELExecutionResult::new(DataType::Unknown, Some(vec![])),
        },
    };
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::result_of_binary_op;
    use sel_common::{from_byte_vec, DataType, Operation};

    #[test]
    fn executes_integer() {
        let result = result_of_binary_op(
            Operation::GreaterThanOrEqual,
            DataType::Integer,
            "10",
            DataType::Integer,
            "5",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(10 >= 5));
    }

    #[test]
    fn executes_integer_decimal() {
        let result = result_of_binary_op(
            Operation::GreaterThanOrEqual,
            DataType::Integer,
            "9",
            DataType::Decimal,
            "3.14",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(9.0 >= 3.14));
    }

    #[test]
    fn executes_decimal_integer() {
        let result = result_of_binary_op(
            Operation::GreaterThanOrEqual,
            DataType::Decimal,
            "3.14",
            DataType::Integer,
            "9",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(3.14 >= 9.0));
    }

    #[test]
    fn executes_decimal() {
        let result = result_of_binary_op(
            Operation::GreaterThanOrEqual,
            DataType::Decimal,
            "3.14",
            DataType::Decimal,
            "6.45",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(3.14 >= 6.45));
    }

    #[test]
    fn executes_string() {
        let result = result_of_binary_op(
            Operation::GreaterThanOrEqual,
            DataType::String,
            "Hello",
            DataType::String,
            "World",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some("Hello" >= "World"));
    }

    #[test]
    fn executes_integer_unit() {
        let result = result_of_binary_op(
            Operation::GreaterThanOrEqual,
            DataType::Integer,
            "9",
            DataType::Unit,
            "()",
        );

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }

    #[test]
    fn executes_unit_integer() {
        let result = result_of_binary_op(
            Operation::GreaterThanOrEqual,
            DataType::Unit,
            "()",
            DataType::Integer,
            "9",
        );

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }
}
