use super::execution_result::SELExecutionResult;
use super::utils::{get_values_from_results, match_math_ops, OptionOr};
use sel_common::{to_byte_vec, DataType, FromByteVec, SELTree, SELTreeNode};

fn concat_results<L: FromByteVec + ToString, R: FromByteVec + ToString>(
    left: &SELExecutionResult,
    right: &SELExecutionResult,
) -> SELExecutionResult {
    let (left_val, right_val) = get_values_from_results::<L, R>(left, right);

    let result = left_val.to_string() + &right_val.to_string();

    SELExecutionResult::new(DataType::String, Some(to_byte_vec(&result)))
}

pub fn addition_operation(tree: &SELTree, node: &SELTreeNode) -> SELExecutionResult {
    return match match_math_ops(
        tree,
        node,
        |left, right| left + right,
        |left, right| left + right,
    ) {
        OptionOr::Some(result) => result,
        OptionOr::Or((left_result, right_result)) => {
            match (left_result.get_type(), right_result.get_type()) {
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
                _ => SELExecutionResult::new(DataType::Unknown, Some(vec![])),
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::result_of_binary_op;
    use sel_common::{from_byte_vec, DataType, Operation};

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
}
