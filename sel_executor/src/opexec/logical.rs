use super::utils::{get_left_right_results, get_values_from_results};
use super::SELExecutionResult;
use sel_common::{to_byte_vec, DataType, SELTree, SELTreeNode};

fn match_logical<F>(tree: &SELTree, node: &SELTreeNode, f: F) -> SELExecutionResult
where
    F: Fn(bool, bool) -> bool,
{
    let (left_result, right_result) = get_left_right_results(tree, node);

    return match (left_result.get_type(), right_result.get_type()) {
        (DataType::Boolean, DataType::Boolean) => {
            let (left_val, right_val) =
                get_values_from_results::<bool, bool>(&left_result, &right_result);

            let result = f(left_val, right_val);

            SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(result)))
        }
        (_, DataType::Unit) | (DataType::Unit, _) => SELExecutionResult::new(DataType::Unit, None),
        _ => SELExecutionResult::new(DataType::Unknown, Some(vec![])),
    };
}

pub fn logical_or_operation(tree: &SELTree, node: &SELTreeNode) -> SELExecutionResult {
    return match_logical(tree, node, |left, right| left || right);
}

pub fn logical_and_operation(tree: &SELTree, node: &SELTreeNode) -> SELExecutionResult {
    return match_logical(tree, node, |left, right| left && right);
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::result_of_binary_op;
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
}
