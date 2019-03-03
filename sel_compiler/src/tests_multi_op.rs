#[cfg(test)]
mod tests {
    use super::super::compiler::Compiler;
    use super::super::data_type::DataType;
    use super::super::operation::Operation;

    #[test]
    fn compiles_two_addition_operations() {
        let input = String::from("5 + 10 + 15");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        for node in tree.get_nodes() {
            println!("{:?}", node);
        }

        // tree should look like
        //          +
        //         / \
        //        +   15
        //       / \
        //      5   10

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        let l2_left = tree.get_nodes().get(left.get_left().unwrap()).unwrap();
        let l2_right = tree.get_nodes().get(left.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Addition);
        assert_eq!(root.get_value().get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Addition);
        assert_eq!(left.get_value().get_data_type(), DataType::Unknown);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_value().get_data_type(), DataType::Integer);

        assert_eq!(l2_left.get_operation(), Operation::Touch);
        assert_eq!(l2_left.get_value().get_data_type(), DataType::Integer);

        assert_eq!(l2_right.get_operation(), Operation::Touch);
        assert_eq!(l2_right.get_value().get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_addition_multiplication_operations() {
        let input = String::from("5 + 10 * 15");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        // tree should look like
        //          +
        //         / \
        //        5   *
        //           / \
        //         10   15

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        let r2_left = tree.get_nodes().get(right.get_left().unwrap()).unwrap();
        let r2_right = tree.get_nodes().get(right.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Addition);
        assert_eq!(root.get_value().get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_value().get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Multiplication);
        assert_eq!(right.get_value().get_data_type(), DataType::Unknown);

        assert_eq!(r2_left.get_operation(), Operation::Touch);
        assert_eq!(r2_left.get_value().get_data_type(), DataType::Integer);

        assert_eq!(r2_right.get_operation(), Operation::Touch);
        assert_eq!(r2_right.get_value().get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_multiplication_addition_operations() {
        let input = String::from("5 * 10 + 15");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        // tree should look like
        //          +
        //         / \
        //        *   15
        //       / \
        //      5   10

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        let l2_left = tree.get_nodes().get(left.get_left().unwrap()).unwrap();
        let l2_right = tree.get_nodes().get(left.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Addition);
        assert_eq!(root.get_value().get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Multiplication);
        assert_eq!(left.get_value().get_data_type(), DataType::Unknown);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_value().get_data_type(), DataType::Integer);

        assert_eq!(l2_left.get_operation(), Operation::Touch);
        assert_eq!(l2_left.get_value().get_data_type(), DataType::Integer);

        assert_eq!(l2_right.get_operation(), Operation::Touch);
        assert_eq!(l2_right.get_value().get_data_type(), DataType::Integer);
    }

    #[test]
    fn arithmetic_operations_2() {
        let input = String::from("5 * 10 + 15 / 2 - 5 % 3 + 4 - 3");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        // tree should look like
        //                           __ "-" __
        //                          /         \
        //                     __ "+" __       3
        //                    /         \
        //               __ "-" __      4
        //              /         \
        //          _ "+" _       "%"
        //         /       \      / \
        //       "*"       "/"   5   3
        //       / \       / \
        //      5   10   15   2

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Subtraction);

        assert_eq!(left.get_operation(), Operation::Addition);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_value().get_data_type(), DataType::Integer);

        let l_left = tree.get_nodes().get(left.get_left().unwrap()).unwrap();
        let l_right = tree.get_nodes().get(left.get_right().unwrap()).unwrap();

        assert_eq!(l_left.get_operation(), Operation::Subtraction);

        assert_eq!(l_right.get_operation(), Operation::Touch);
        assert_eq!(l_right.get_value().get_data_type(), DataType::Integer);

        let ll_left = tree.get_nodes().get(l_left.get_left().unwrap()).unwrap();
        let ll_right = tree.get_nodes().get(l_left.get_right().unwrap()).unwrap();

        assert_eq!(ll_left.get_operation(), Operation::Addition);
        assert_eq!(ll_right.get_operation(), Operation::Modulo);

        let llr_left = tree.get_nodes().get(ll_right.get_left().unwrap()).unwrap();
        let llr_right = tree.get_nodes().get(ll_right.get_right().unwrap()).unwrap();

        assert_eq!(llr_left.get_operation(), Operation::Touch);
        assert_eq!(llr_left.get_value().get_data_type(), DataType::Integer);

        assert_eq!(llr_right.get_operation(), Operation::Touch);
        assert_eq!(llr_right.get_value().get_data_type(), DataType::Integer);

        let lll_left = tree.get_nodes().get(ll_left.get_left().unwrap()).unwrap();
        let lll_right = tree.get_nodes().get(ll_left.get_right().unwrap()).unwrap();

        assert_eq!(lll_left.get_operation(), Operation::Multiplication);
        assert_eq!(lll_right.get_operation(), Operation::Division);

        let lllr_left = tree.get_nodes().get(lll_right.get_left().unwrap()).unwrap();
        let lllr_right = tree
            .get_nodes()
            .get(lll_right.get_right().unwrap())
            .unwrap();

        assert_eq!(lllr_left.get_operation(), Operation::Touch);
        assert_eq!(lllr_left.get_value().get_data_type(), DataType::Integer);

        assert_eq!(lllr_right.get_operation(), Operation::Touch);
        assert_eq!(lllr_right.get_value().get_data_type(), DataType::Integer);

        let llll_left = tree.get_nodes().get(lll_left.get_left().unwrap()).unwrap();
        let llll_right = tree.get_nodes().get(lll_left.get_right().unwrap()).unwrap();

        assert_eq!(llll_left.get_operation(), Operation::Touch);
        assert_eq!(llll_left.get_value().get_data_type(), DataType::Integer);

        assert_eq!(llll_right.get_operation(), Operation::Touch);
        assert_eq!(llll_right.get_value().get_data_type(), DataType::Integer);
    }
}
