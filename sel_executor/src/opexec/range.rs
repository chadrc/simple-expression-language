use sel_common::{to_byte_vec, DataType, Range, SELTree, SELTreeNode};

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
                get_values_from_results::<i64, i64>(&left_result, &right_result);

            // internally, all ranges will be exclusive
            // so we simply add 1 to give upper bound to make inclusive of that number
            let right_val = if inclusive { right_val + 1 } else { right_val };

            let range = Range::new(left_val, right_val);

            SELExecutionResult::new(DataType::Range, Some(to_byte_vec(range)))
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
        let range: Range = from_byte_vec(first_result.get_value().unwrap());

        assert_eq!(first_result.get_type(), DataType::Range);
        assert_eq!(range.get_lower(), 5);
        assert_eq!(range.get_upper(), 10);
    }

    #[test]
    fn inclusive_range() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("5...10"));

        let context = SELExecutionContext::new();

        let results = execute_sel_tree(&tree, &context);

        let first_result = results.get(0).unwrap();
        let range: Range = from_byte_vec(first_result.get_value().unwrap());

        assert_eq!(first_result.get_type(), DataType::Range);
        assert_eq!(range.get_lower(), 5);
        assert_eq!(range.get_upper(), 11);
    }
}
