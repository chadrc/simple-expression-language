use super::super::context::SELExecutionContext;
use super::SELExecutionResult;
use crate::opexec::utils::match_equality_ops;
use sel_common::{DataType, SELTree, SELTreeNode};

pub fn operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &mut SELExecutionContext,
) -> SELExecutionResult {
    return match_equality_ops(
        tree,
        node,
        context,
        |left, right| left != right,
        |left, right| left != right,
        |left, right| left != right,
        |left_unit, right_unit| (DataType::Boolean, Some(!(left_unit && right_unit))),
    );
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::result_of_binary_op;
    use sel_common::{from_byte_vec, DataType, Operation};

    #[test]
    fn executes_integer() {
        let result = result_of_binary_op(
            Operation::Inequality,
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
        assert_eq!(result_value, Some(10 != 5));
    }

    #[test]
    fn executes_integer_decimal() {
        let result = result_of_binary_op(
            Operation::Inequality,
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
        assert_eq!(result_value, Some(9.0 != 3.14));
    }

    #[test]
    fn executes_decimal_integer() {
        let result = result_of_binary_op(
            Operation::Inequality,
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
        assert_eq!(result_value, Some(3.14 != 9.0));
    }

    #[test]
    fn executes_decimal() {
        let result = result_of_binary_op(
            Operation::Inequality,
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
        assert_eq!(result_value, Some(3.14 != 6.45));
    }

    #[test]
    fn executes_string() {
        let result = result_of_binary_op(
            Operation::Inequality,
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
        assert_eq!(result_value, Some("Hello" != "World"));
    }

    #[test]
    fn executes_integer_unit() {
        let result = result_of_binary_op(
            Operation::Inequality,
            DataType::Integer,
            "9",
            DataType::Unit,
            "()",
        );

        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, true);
    }

    #[test]
    fn executes_unit_integer() {
        let result = result_of_binary_op(
            Operation::Inequality,
            DataType::Unit,
            "()",
            DataType::Integer,
            "9",
        );

        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, true);
    }

    #[test]
    fn executes_unit_unit() {
        let result = result_of_binary_op(
            Operation::Inequality,
            DataType::Unit,
            "()",
            DataType::Unit,
            "()",
        );

        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, false);
    }
}
