use super::SELExecutionContext;
use crate::opexec::execution_result::SELExecutionResult;
use crate::opexec::get_node_result;
use crate::opexec::utils::get_value_from_result;
use sel_common::{DataType, Operation, SELTree, SELTreeNode, SELValue};

pub fn pipe_first_right_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    // first, get value of left
    // this is our value to pipe
    node.get_left()
        .and_then(|left_index| tree.get_nodes().get(left_index))
        .map(|left_node| get_node_result(tree, left_node, context))
        .map(|left_result| {
            let value = left_result.get_sel_value();

            // get right node
            // should be the root of an expression to execute
            node.get_right()
                .and_then(|right_index| tree.get_nodes().get(right_index))
                .and_then(|right_node| {
                    // make new context with value to pipe as input
                    let mut pipe_context = context.clone();
                    pipe_context.set_input(value.clone());

                    match right_node.get_operation() {
                        Operation::Expression => {
                            // value of expression is sub tree index
                            tree.get_usize_value_of(right_node)
                                .and_then(|sub_tree_index| tree.get_sub_trees().get(sub_tree_index))
                                .and_then(|sub_tree| sub_tree.get_roots().get(0))
                                .and_then(|sub_tree_root_index| {
                                    tree.get_nodes().get(*sub_tree_root_index)
                                })
                                .map(|sub_tree_root| {
                                    get_node_result(tree, sub_tree_root, &pipe_context)
                                })
                        }
                        _ => {
                            // get result of right node
                            Some(get_node_result(tree, right_node, &pipe_context))
                        }
                    }
                })
                .unwrap_or(SELExecutionResult::new(DataType::Unknown, None))
        })
        .unwrap_or(SELExecutionResult::new(DataType::Unknown, None))
}

pub fn pipe_first_left_operation(
    _tree: &SELTree,
    _node: &SELTreeNode,
    _context: &SELExecutionContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

pub fn pipe_last_right_operation(
    _tree: &SELTree,
    _node: &SELTreeNode,
    _context: &SELExecutionContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

pub fn pipe_last_left_operation(
    _tree: &SELTree,
    _node: &SELTreeNode,
    _context: &SELExecutionContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

#[cfg(test)]
mod tests {
    use crate::opexec::get_node_result;
    use crate::SELExecutionContext;
    use sel_common::{from_byte_vec, DataType, SELContext};
    use sel_compiler::Compiler;

    #[test]
    fn executes_pipe_first_right_raw_expresssion() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("10 -> $ * 10"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 100);
    }

    #[test]
    fn executes_pipe_first_right_nested_expression() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("10 -> { $ * 10 }"));

        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 100);
    }
}
