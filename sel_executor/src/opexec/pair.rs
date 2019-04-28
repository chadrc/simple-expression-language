use sel_common::{to_byte_vec, DataType, Pair, SELTree, SELTreeNode};

use super::utils::{get_left_right_results, get_values_from_results};

use super::super::context::SELExecutionContext;
use super::execution_result::SELExecutionResult;

pub fn operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    let (left_result, right_result) = get_left_right_results(tree, node, context);
    let bytes = to_byte_vec(Pair::new(
        left_result.get_sel_value().clone(),
        right_result.get_sel_value().clone(),
    ));
    return SELExecutionResult::new(DataType::Pair, Some(bytes));
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use sel_common::{
        from_byte_vec, DataHeap, DataType, Operation, SELContext, SELTree, SELTreeNode, Symbol,
        SymbolTable,
    };
    use sel_compiler::Compiler;

    use super::super::get_node_result;
    use super::*;

    #[test]
    fn executes_pair() {
        let compiler = Compiler::new();
        let mut context = SELContext::new();
        let tree = compiler.compile_with_context(&String::from(":value = 10"), context);
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);

        assert_eq!(result.get_type(), DataType::Pair);

        let pair: Pair = from_byte_vec(result.get_value().unwrap());

        let left_value: Symbol = from_byte_vec(pair.get_left().get_value().unwrap());
        let right_value: i64 = from_byte_vec(pair.get_right().get_value().unwrap());

        assert_eq!(pair.get_left().get_type(), DataType::Symbol);
        assert_eq!(left_value.get_identifier(), &String::from("value"));
        assert_eq!(left_value.get_table_index(), 0);
        assert_eq!(pair.get_right().get_type(), DataType::Integer);
        assert_eq!(right_value, 10);
    }
}
