use super::super::context::SELExecutionContext;
use super::execution_result::SELExecutionResult;
use super::utils::{match_math_ops, OptionOr};
use sel_common::{DataType, SELTree, SELTreeNode};

pub fn operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &mut SELExecutionContext,
) -> SELExecutionResult {
    return match match_math_ops(
        tree,
        node,
        context,
        |left, right| left % right,
        |left, right| left % right,
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
    fn executes_integer_modulo() {
        let result = result_of_binary_op(
            Operation::Modulo,
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
        assert_eq!(result_value, Some(9 % 5));
    }

    #[test]
    fn executes_integer_decimal_modulo() {
        let result = result_of_binary_op(
            Operation::Modulo,
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
        assert_eq!(result_value, Some(9.0 % 3.14));
    }

    #[test]
    fn executes_decimal_integer_modulo() {
        let result = result_of_binary_op(
            Operation::Modulo,
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
        assert_eq!(result_value, Some(3.14 % 9.0));
    }

    #[test]
    fn executes_decimal_modulo() {
        let result = result_of_binary_op(
            Operation::Modulo,
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
        assert_eq!(result_value, Some(3.14 % 6.45));
    }

    #[test]
    fn executes_integer_unit_modulo() {
        let result = result_of_binary_op(
            Operation::Modulo,
            DataType::Integer,
            "9",
            DataType::Unit,
            "()",
        );

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }

    #[test]
    fn executes_unit_integer_modulo() {
        let result = result_of_binary_op(
            Operation::Modulo,
            DataType::Unit,
            "()",
            DataType::Integer,
            "9",
        );

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }
}
