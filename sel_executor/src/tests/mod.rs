#[cfg(test)]
mod tree_execution {
    use super::super::execute_sel_tree;
    use super::super::SELContext;
    use sel_common::{DataHeap, DataType, SELTree};
    use sel_compiler::Compiler;

    #[test]
    fn executes_empty() {
        let tree = SELTree::new(0, vec![], vec![], DataHeap::new());

        let context = SELContext::new();

        let result = execute_sel_tree(&tree, &context);

        assert_eq!(result.get_type(), DataType::Unknown);
        assert_eq!(result.get_value(), None);
    }

    #[test]
    fn multiple_results() {}
}
