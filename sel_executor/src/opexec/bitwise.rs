use super::{SELExecutionContext, SELExecutionResult};
use crate::opexec::get_node_result;
use crate::opexec::utils::{get_left_right_results, get_values_from_results};
use sel_common::{from_byte_vec, to_byte_vec, DataType, SELTree, SELTreeNode};

fn match_bitwise_op<F>(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
    f: F,
) -> SELExecutionResult
where
    F: Fn(i64, i64) -> i64,
{
    let (left_result, right_result) = get_left_right_results(tree, node, context);

    return match (left_result.get_type(), right_result.get_type()) {
        (DataType::Integer, DataType::Integer) => {
            let (left_value, right_value) =
                get_values_from_results::<i64, i64>(&left_result, &right_result);

            let value = f(left_value, right_value);

            return SELExecutionResult::new(DataType::Integer, Some(to_byte_vec(value)));
        }
        _ => SELExecutionResult::new(DataType::Unknown, None),
    };
}

pub fn or_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return match_bitwise_op(tree, node, context, |left, right| left | right);
}

pub fn and_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return match_bitwise_op(tree, node, context, |left, right| left & right);
}

pub fn xor_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return match_bitwise_op(tree, node, context, |left, right| left ^ right);
}

pub fn left_shift_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return match_bitwise_op(tree, node, context, |left, right| left << right);
}

pub fn right_shift_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return match_bitwise_op(tree, node, context, |left, right| left >> right);
}

#[cfg(test)]
mod tests {
    use super::super::{get_node_result, SELExecutionContext};
    use sel_common::{
        from_byte_vec, DataHeap, DataType, Operation, SELContext, SELTree, SELTreeNode, SELValue,
        SymbolTable,
    };
    use sel_compiler::Compiler;

    #[test]
    fn executes_bitwise_or() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("250 | 10928"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 250 | 10928);
    }

    #[test]
    fn executes_bitwise_and() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("250 & 10928"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 250 & 10928);
    }

    #[test]
    fn executes_bitwise_xor() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("250 ^ 10928"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 250 ^ 10928);
    }

    #[test]
    fn executes_bitwise_left_shift() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("250 << 2"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 250 << 2);
    }

    #[test]
    fn executes_bitwise_right_shift() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("250 >> 2"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 250 >> 2);
    }
}
