use super::utils::{match_math_ops, MathOps, OptionOr};
use super::SELExecutionResult;
use sel_common::{DataType, SELTree, SELTreeNode};

pub fn division_operation(tree: &SELTree, node: &SELTreeNode) -> SELExecutionResult {
    return match match_math_ops(
        tree,
        node,
        MathOps {
            perform_integer: |left, right| left / right,
            perform_float: |left, right| left / right,
        },
    ) {
        OptionOr::Some(result) => result,
        OptionOr::Or(_) => SELExecutionResult::new(DataType::Unknown, Some(vec![])),
    };
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::result_of_binary_op;
    use sel_common::{from_byte_vec, DataType, Operation};

    #[test]
    fn executes_integer_division() {
        let result = result_of_binary_op(
            Operation::Division,
            DataType::Integer,
            "10",
            DataType::Integer,
            "5",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(result_value, Some(2));
    }

    #[test]
    fn executes_integer_decimal_division() {
        let result = result_of_binary_op(
            Operation::Division,
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
        assert_eq!(result_value, Some(9.0 / 3.14));
    }

    #[test]
    fn executes_decimal_integer_division() {
        let result = result_of_binary_op(
            Operation::Division,
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
        assert_eq!(result_value, Some(3.14 / 9.0));
    }

    #[test]
    fn executes_decimal_division() {
        let result = result_of_binary_op(
            Operation::Division,
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
        assert_eq!(result_value, Some(3.14 / 6.45));
    }

    #[test]
    fn executes_integer_unit_division() {
        let result = result_of_binary_op(
            Operation::Division,
            DataType::Integer,
            "9",
            DataType::Unit,
            "()",
        );

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }

    #[test]
    fn executes_unit_integer_division() {
        let result = result_of_binary_op(
            Operation::Division,
            DataType::Unit,
            "()",
            DataType::Integer,
            "9",
        );

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }
}
