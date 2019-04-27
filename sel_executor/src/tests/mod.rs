mod tree_execution {
    use super::super::execute_sel_tree;
    use super::super::SELContext;
    use sel_common::{from_byte_vec, DataHeap, DataType, SELTree, SymbolTable};
    use sel_compiler::Compiler;

    #[test]
    fn executes_empty() {
        let tree = SELTree::new(0, vec![], vec![], DataHeap::new(), SymbolTable::new());

        let context = SELContext::new();

        let results = execute_sel_tree(&tree, &context);

        let result = results.get(0).unwrap();

        assert_eq!(result.get_type(), DataType::Unknown);
        assert_eq!(result.get_value(), None);
    }

    #[test]
    fn multiple_results() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("5 + 10\n15 + 20"));

        let context = SELContext::new();

        let results = execute_sel_tree(&tree, &context);

        let first_result = results.get(0).unwrap();
        let first_result_value = match first_result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(first_result.get_type(), DataType::Integer);
        assert_eq!(first_result_value, Some(15));

        let second_result = results.get(1).unwrap();
        let second_result_value = match second_result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(second_result.get_type(), DataType::Integer);
        assert_eq!(second_result_value, Some(35));
    }

    #[test]
    fn current_result_usage() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("5 + 10\n? + 20"));

        let context = SELContext::new();

        let results = execute_sel_tree(&tree, &context);

        let first_result = results.get(0).unwrap();
        let first_result_value = match first_result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(first_result.get_type(), DataType::Integer);
        assert_eq!(first_result_value, Some(15));

        let second_result = results.get(1).unwrap();
        let second_result_value = match second_result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(second_result.get_type(), DataType::Integer);
        assert_eq!(second_result_value, Some(35));
    }
}
