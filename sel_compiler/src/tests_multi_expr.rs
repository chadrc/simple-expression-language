#[cfg(test)]
mod tests {
    use super::super::compiler::Compiler;
    use sel_common::{DataType, Operation};

    #[test]
    fn two_expressions() {
        assert_two_expressions("5 + 10\n15 + 20");
    }

    #[test]
    fn leading_newlines_are_dropped() {
        assert_two_expressions("\n\n\n\n5 + 10\n15 + 20");
    }

    #[test]
    fn tailing_newlines_are_dropped() {
        assert_two_expressions("5 + 10\n15 + 20\n\n\n\n\n");
    }

    #[test]
    fn extra_newlines_are_dropped() {
        assert_two_expressions("5 + 10\n\n\n\n15 + 20");
    }

    #[test]
    fn two_expressions_with_terminable_nodes() {
        assert_two_expressions("5 +\n 10\n15 +\n 20");
    }

    #[test]
    fn two_expressions_with_non_terminable_nodes() {
        assert_two_expressions("5\n+ 10\n15\n+ 20");
    }

    #[test]
    fn two_expressions_with_terminable_nodes_and_input() {
        assert_two_expressions("5 +\n 10\n$ +\n 20");
    }

    #[test]
    fn two_expressions_with_non_terminable_nodes_and_input() {
        assert_two_expressions("5\n+ 10\n$\n+ 20");
    }

    // TODO: finish when result is converted to an operation
    // #[test]
    // fn two_expressions_with_terminable_nodes_with_result() {
    //     assert_two_expressions("5 +\n 10\n? +\n 20");
    // }

    // #[test]
    // fn two_expressions_with_non_terminable_nodes_with_result() {
    //     assert_two_expressions("5\n+ 10\n$\n+ 20");
    // }

    fn assert_two_expressions(s: &str) {
        let input = String::from(s);
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        assert!(root.get_left() != None);
        assert!(root.get_right() != None);

        assert_eq!(root.get_operation(), Operation::Addition);

        let root_2 = tree.get_sub_root(0).unwrap();

        assert!(root_2.get_left() != None);
        assert!(root_2.get_right() != None);

        assert_eq!(root_2.get_operation(), Operation::Addition);
    }
}
