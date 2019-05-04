use super::super::context::SELExecutionContext;
use crate::opexec::execution_result::SELExecutionResult;
use crate::opexec::utils::{get_left_right_results, get_values_from_results, match_equality_ops};
use sel_common::{to_byte_vec, AssociativeList, DataType, SELTree, SELTreeNode};

pub fn match_true(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

pub fn match_false(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::result_of_binary_op;
    use crate::opexec::get_node_result;
    use crate::SELExecutionContext;
    use sel_common::{from_byte_vec, DataType, Operation, SELValue};
    use sel_compiler::Compiler;

    #[test]
    fn executes_match_true_right_result() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("true => 100"));

        let mut execution_context = SELExecutionContext::new();
        execution_context.set_input(SELValue::new_from_int(200));

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 100);
    }

    #[test]
    fn executes_match_true_current_result() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("false => 100"));

        let mut execution_context = SELExecutionContext::new();
        execution_context.set_input(SELValue::new_from_int(200));

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 200);
    }

    #[test]
    fn executes_match_false_current_result() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("true =!> 100"));

        let mut execution_context = SELExecutionContext::new();
        execution_context.set_input(SELValue::new_from_int(200));

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 200);
    }

    #[test]
    fn executes_match_false_right_result() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("false =!> 100"));

        let mut execution_context = SELExecutionContext::new();
        execution_context.set_input(SELValue::new_from_int(200));

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 100);
    }
}
