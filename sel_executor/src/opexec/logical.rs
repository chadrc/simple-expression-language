use super::super::context::SELContext;
use super::utils::{get_left_right_results, get_value_from_result, get_values_from_results};
use super::SELExecutionResult;
use sel_common::{to_byte_vec, DataType, SELTree, SELTreeNode};

fn logical_xor(left: bool, right: bool) -> bool {
    return (left || right) && left != right;
}

fn match_logical<F>(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELContext,
    f: F,
) -> SELExecutionResult
where
    F: Fn(bool, bool) -> bool,
{
    let (left_result, right_result) = get_left_right_results(tree, node, context);

    return match (left_result.get_type(), right_result.get_type()) {
        (DataType::Boolean, DataType::Boolean) => {
            let (left_val, right_val) =
                get_values_from_results::<bool, bool>(&left_result, &right_result);

            let result = f(left_val, right_val);

            SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(result)))
        }
        (DataType::Unit, DataType::Boolean) => {
            let right_val: bool = get_value_from_result(&right_result);

            let result = f(false, right_val);

            SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(result)))
        }
        (DataType::Boolean, DataType::Unit) => {
            let left_val: bool = get_value_from_result(&left_result);

            let result = f(left_val, false);

            SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(result)))
        }
        _ => SELExecutionResult::new(DataType::Unknown, Some(vec![])),
    };
}

pub fn xor_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELContext,
) -> SELExecutionResult {
    return match_logical(tree, node, context, logical_xor);
}

pub fn or_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELContext,
) -> SELExecutionResult {
    return match_logical(tree, node, context, |left, right| left || right);
}

pub fn and_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELContext,
) -> SELExecutionResult {
    return match_logical(tree, node, context, |left, right| left && right);
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::result_of_binary_op;
    use crate::opexec::logical::logical_xor;
    use sel_common::{from_byte_vec, DataType, Operation};

    #[test]
    fn executes_logical_or() {
        let result = result_of_binary_op(
            Operation::LogicalOr,
            DataType::Boolean,
            "false",
            DataType::Boolean,
            "true",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(true));
    }

    #[test]
    fn executes_logical_and() {
        let result = result_of_binary_op(
            Operation::LogicalAnd,
            DataType::Boolean,
            "false",
            DataType::Boolean,
            "true",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(false));
    }

    #[test]
    fn executes_logical_or_unit_bool() {
        let result = result_of_binary_op(
            Operation::LogicalOr,
            DataType::Unit,
            "()",
            DataType::Boolean,
            "true",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(true));
    }

    #[test]
    fn executes_logical_or_bool_unit() {
        let result = result_of_binary_op(
            Operation::LogicalOr,
            DataType::Boolean,
            "true",
            DataType::Unit,
            "()",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(true));
    }

    #[test]
    fn executes_logical_and_unit_bool() {
        let result = result_of_binary_op(
            Operation::LogicalAnd,
            DataType::Unit,
            "()",
            DataType::Boolean,
            "true",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(false));
    }

    #[test]
    fn executes_logical_and_bool_unit() {
        let result = result_of_binary_op(
            Operation::LogicalAnd,
            DataType::Boolean,
            "true",
            DataType::Unit,
            "()",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(false));
    }

    #[test]
    fn executes_logical_xor() {
        let result = result_of_binary_op(
            Operation::LogicalXOR,
            DataType::Boolean,
            "true",
            DataType::Boolean,
            "true",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(false));
    }

    #[test]
    fn xor_true_false() {
        assert_eq!(logical_xor(true, false), true);
    }

    #[test]
    fn xor_false_true() {
        assert_eq!(logical_xor(false, true), true);
    }

    #[test]
    fn xor_true_true() {
        assert_eq!(logical_xor(true, true), false);
    }

    #[test]
    fn xor_false_false() {
        assert_eq!(logical_xor(false, false), false);
    }
}
