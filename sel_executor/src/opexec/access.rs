use super::SELExecutionContext;
use crate::opexec::execution_result::SELExecutionResult;
use crate::opexec::get_node_result;
use crate::opexec::utils::get_left_right_results;
use sel_common::{
    from_byte_vec, to_byte_vec, DataType, List, Operation, Pair, SELTree, SELTreeNode, SELValue,
};

fn get_identifier(node: &SELTreeNode, tree: &SELTree) -> String {
    node.get_right()
        .and_then(|index| tree.get_nodes().get(index))
        .and_then(|node| {
            if node.get_data_type() == DataType::Identifier {
                tree.get_usize_value_of(node)
                    .and_then(|symbol_index| tree.get_symbol_table().get_symbol(symbol_index))
            } else {
                None
            }
        })
        .map_or(String::from(""), |s| s.clone())
}

fn get_index(node: &SELTreeNode, tree: &SELTree) -> Option<usize> {
    node.get_right()
        .and_then(|index| tree.get_nodes().get(index))
        .and_then(|node| {
            if node.get_data_type() == DataType::Integer {
                tree.get_usize_value_of(node)
            } else {
                None
            }
        })
}

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

    return match left_result.get_type() {
        DataType::Pair => {
            // get type and raw value of right
            // should be an identifier
            let identifier: String = get_identifier(node, tree);
            let pair: Pair = from_byte_vec(left_result.get_value().unwrap());
            match identifier.as_ref() {
                "left" => SELExecutionResult::from(pair.get_left()),
                "right" => SELExecutionResult::from(pair.get_right()),
                _ => SELExecutionResult::new(DataType::Unit, None),
            }
        }
        DataType::List => match get_index(node, tree) {
            Some(index) => {
                let list: List = from_byte_vec(left_result.get_value().unwrap());
                SELExecutionResult::from(list.get_values().get(index).unwrap_or(&SELValue::new()))
            }
            None => SELExecutionResult::new(DataType::Unit, None),
        },
        _ => SELExecutionResult::new(DataType::Unit, None),
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

    #[test]
    fn executes_access_of_non_existent_identifier() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("(:my_value = 100).center"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);

        assert_eq!(result.get_type(), DataType::Unit);
    }

    #[test]
    fn executes_access_of_non_existent_value() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("uninitialized.field"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);

        assert_eq!(result.get_type(), DataType::Unit);
    }

    #[test]
    fn executes_access_of_non_existent_chain() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("uninitialized.next.field"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);

        assert_eq!(result.get_type(), DataType::Unit);
    }

    #[test]
    fn executes_chain_access() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("(:top = :next = 100).right.right"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 100);
    }

    #[test]
    fn executes_list_index_access() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("(100, 200, 300).1"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 200);
    }
}
