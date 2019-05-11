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
        |left, right| left.pow(right as u32),
        |left, right| left.powf(right),
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
    fn executes_integer_exponential() {
        let result = result_of_binary_op(
            Operation::Exponential,
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
        assert_eq!(result_value, Some(9i32.pow(5)));
    }

    #[test]
    fn executes_integer_decimal_exponential() {
        let result = result_of_binary_op(
            Operation::Exponential,
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
        assert_eq!(result_value, Some(9.0f64.powf(3.14)));
    }

    #[test]
    fn executes_decimal_integer_exponential() {
        let result = result_of_binary_op(
            Operation::Exponential,
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
        assert_eq!(result_value, Some(3.14f64.powf(9.0)));
    }

    #[test]
    fn executes_decimal_exponential() {
        let result = result_of_binary_op(
            Operation::Exponential,
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
        assert_eq!(result_value, Some(3.14f64.powf(6.45)));
    }

    #[test]
    fn executes_integer_unit_exponential() {
        let result = result_of_binary_op(
            Operation::Exponential,
            DataType::Integer,
            "9",
            DataType::Unit,
            "()",
        );

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }

    #[test]
    fn executes_unit_integer_exponential() {
        let result = result_of_binary_op(
            Operation::Exponential,
            DataType::Unit,
            "()",
            DataType::Integer,
            "9",
        );

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }
}
