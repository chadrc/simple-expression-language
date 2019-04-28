use sel_common::{to_byte_vec, DataType, SELTree, SELTreeNode};

use crate::opexec::utils::get_values_from_results;

use super::execution_result::SELExecutionResult;
use super::utils::get_left_right_results;
use super::SELExecutionContext;

fn range_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
    inclusive: bool,
) -> SELExecutionResult {
    let (left_result, right_result) = get_left_right_results(tree, node, context);
    return match (left_result.get_type(), right_result.get_type()) {
        (DataType::Integer, DataType::Integer) => {
            let (left_val, right_val) =
                get_values_from_results::<i32, i32>(&left_result, &right_result);

            // internally, all ranges will be exclusive
            // so we simply add 1 to give upper bound to make inclusive of that number
            let right_val = if inclusive { right_val + 1 } else { right_val };

            let mut left_bytes = to_byte_vec(left_val);
            let mut right_bytes = to_byte_vec(right_val);

            left_bytes.append(&mut right_bytes);

            SELExecutionResult::new(DataType::Range, Some(left_bytes))
        }
        _ => SELExecutionResult::new(DataType::Unknown, None),
    };
}

pub fn exclusive_range_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return range_operation(tree, node, context, false);
}

pub fn inclusive_range_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return range_operation(tree, node, context, true);
}

#[cfg(test)]
mod tests {
    use sel_common::{from_byte_vec, DataType};
    use sel_compiler::Compiler;

    use super::super::super::execute_sel_tree;
    use super::*;

    #[test]
    fn exclusive_range() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("5..10"));

        let context = SELExecutionContext::new();

        let results = execute_sel_tree(&tree, &context);

        let first_result = results.get(0).unwrap();
        let first_result_value = match first_result.get_value() {
            Some(value) => {
                let left = from_byte_vec(&Vec::from(&value[0..4]));
                let right = from_byte_vec(&Vec::from(&value[4..]));

                (left, right)
            }
            None => (0, 0),
        };

        assert_eq!(first_result.get_type(), DataType::Range);
        assert_eq!(first_result_value.0, 5);
        assert_eq!(first_result_value.1, 10);
    }

    #[test]
    fn inclusive_range() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("5...10"));

        let context = SELExecutionContext::new();

        let results = execute_sel_tree(&tree, &context);

        let first_result = results.get(0).unwrap();
        let first_result_value = match first_result.get_value() {
            Some(value) => {
                let left = from_byte_vec(&Vec::from(&value[0..4]));
                let right = from_byte_vec(&Vec::from(&value[4..]));

                (left, right)
            }
            None => (0, 0),
        };

        assert_eq!(first_result.get_type(), DataType::Range);
        assert_eq!(first_result_value.0, 5);
        assert_eq!(first_result_value.1, 11);
    }
}
