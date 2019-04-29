use super::SELExecutionContext;
use crate::opexec::execution_result::SELExecutionResult;
use crate::opexec::get_node_result;
use crate::opexec::utils::get_left_right_results;
use sel_common::{
    from_byte_vec, to_byte_vec, DataType, Operation, Pair, SELTree, SELTreeNode, SELValue,
};

pub fn dot_access_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    // get value of left
    // this is the value we will attempt to access
    let left_result: SELExecutionResult = node
        .get_left()
        .and_then(|index| tree.get_nodes().get(index))
        .map(|node| get_node_result(tree, node, context))
        .unwrap_or(SELExecutionResult::new(DataType::Unknown, None));

    // get type and raw value of right
    // should be an identifier
    let identifier: String = node
        .get_right()
        .and_then(|index| tree.get_nodes().get(index))
        .and_then(|node| {
            if node.get_data_type() == DataType::Identifier {
                tree.get_usize_value_of(node)
                    .and_then(|symbol_index| tree.get_symbol_table().get_symbol(symbol_index))
            } else {
                None
            }
        })
        .map_or(String::from(""), |s| s.clone());

    return match (left_result.get_type(), identifier.as_ref()) {
        (DataType::Pair, "left") => {
            let pair: Pair = from_byte_vec(left_result.get_value().unwrap());
            SELExecutionResult::from(pair.get_left())
        }
        (DataType::Pair, "right") => {
            let pair: Pair = from_byte_vec(left_result.get_value().unwrap());
            SELExecutionResult::from(pair.get_right())
        }
        _ => SELExecutionResult::new(DataType::Unknown, None),
    };
}

#[cfg(test)]
mod tests {
    use super::super::{get_node_result, SELExecutionContext};
    use sel_common::{
        from_byte_vec, DataHeap, DataType, Operation, SELContext, SELTree, SELTreeNode, SELValue,
        Symbol, SymbolTable,
    };
    use sel_compiler::Compiler;

    #[test]
    fn executes_pair_left_access() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("(:my_value = 100).left"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let symbol: Symbol = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Symbol);
        assert_eq!(symbol.get_identifier(), &String::from("my_value"));
    }

    #[test]
    fn executes_pair_right_access() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("(:my_value = 100).right"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 100);
    }
}
