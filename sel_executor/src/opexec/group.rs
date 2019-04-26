use super::execution_result::SELExecutionResult;
use super::{get_node_result, SELContext};
use sel_common::{DataType, SELTree, SELTreeNode};

pub fn operation(tree: &SELTree, node: &SELTreeNode, context: &SELContext) -> SELExecutionResult {
    return match node
        .get_right()
        .and_then(|index| tree.get_nodes().get(index))
    {
        Some(right_node) => get_node_result(tree, right_node, context),
        None => SELExecutionResult::new(DataType::Unknown, None),
    };
}

#[cfg(test)]
mod tests {
    use super::super::super::execute_sel_tree;
    use super::super::get_node_result;
    use super::*;
    use sel_common::{from_byte_vec, DataHeap, DataType, Operation, SELTree, SELTreeNode};
    use sel_compiler::Compiler;

    #[test]
    fn executes_group() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("5 * (4 + 3)"));

        let context = SELContext::new();

        let results = execute_sel_tree(&tree, &context);

        let first_result = results.get(0).unwrap();
        let first_result_value = match first_result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(first_result.get_type(), DataType::Integer);
        assert_eq!(first_result_value, Some(35));
    }
}
