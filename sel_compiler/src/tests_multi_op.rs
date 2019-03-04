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

        // tree should look like
        //          +
        //         / \
        //        +   15
        //       / \
        //      5   10

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        let l_left = tree.get_nodes().get(left.get_left().unwrap()).unwrap();
        let l_right = tree.get_nodes().get(left.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Addition);
        assert_eq!(root.get_value().get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Addition);
        assert_eq!(left.get_value().get_data_type(), DataType::Unknown);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_value().get_data_type(), DataType::Integer);

        assert_eq!(l_left.get_operation(), Operation::Touch);
        assert_eq!(l_left.get_value().get_data_type(), DataType::Integer);

        assert_eq!(l_right.get_operation(), Operation::Touch);
        assert_eq!(l_right.get_value().get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_addition_multiplication_operations() {
        let input = String::from("5 + 10 * 15");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        for node in tree.get_nodes() {
            println!("{:?}", node);
        }

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

    #[test]
    fn logical_operations_1() {
        let input = String::from("true && false || true");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        // tree should look like
        //             _ "||" _
        //            /        \
        //        _ "&&" _    true
        //       /        \
        //     true      false

        let root = tree.get_root();

        assert_eq!(root.get_operation(), Operation::LogicalOr);

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(left.get_operation(), Operation::LogicalAnd);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_value().get_data_type(), DataType::Boolean);

        let l_left = tree.get_nodes().get(left.get_left().unwrap()).unwrap();
        let l_right = tree.get_nodes().get(left.get_right().unwrap()).unwrap();

        assert_eq!(l_left.get_operation(), Operation::Touch);
        assert_eq!(l_left.get_value().get_data_type(), DataType::Boolean);

        assert_eq!(l_right.get_operation(), Operation::Touch);
        assert_eq!(l_right.get_value().get_data_type(), DataType::Boolean);
    }

    #[test]
    fn logical_operations_2() {
        let input = String::from("5 + 3 == 4 && 10 >= 8 || 15 < 3 - 20");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        // tree should look like
        //                      ___ "||" ___
        //                     /            \
        //                 _ "&&" _         "<"
        //                /        \        / \
        //              "=="      ">="     15 "-"
        //              /  \      /  \        / \
        //            "+"   4    15   2      3  20
        //            / \
        //           5   3

        let root = tree.get_root();

        assert_eq!(root.get_operation(), Operation::LogicalOr);

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(left.get_operation(), Operation::LogicalAnd);
        assert_eq!(right.get_operation(), Operation::LessThan);

        let r_left = tree.get_nodes().get(right.get_left().unwrap()).unwrap();
        let r_right = tree.get_nodes().get(right.get_right().unwrap()).unwrap();

        assert_eq!(r_left.get_operation(), Operation::Touch);
        assert_eq!(r_left.get_value().get_data_type(), DataType::Integer);

        assert_eq!(r_right.get_operation(), Operation::Subtraction);

        let rr_left = tree.get_nodes().get(r_right.get_left().unwrap()).unwrap();
        let rr_right = tree.get_nodes().get(r_right.get_right().unwrap()).unwrap();

        assert_eq!(rr_left.get_operation(), Operation::Touch);
        assert_eq!(rr_left.get_value().get_data_type(), DataType::Integer);

        assert_eq!(rr_right.get_operation(), Operation::Touch);
        assert_eq!(rr_right.get_value().get_data_type(), DataType::Integer);

        let l_left = tree.get_nodes().get(left.get_left().unwrap()).unwrap();
        let l_right = tree.get_nodes().get(left.get_right().unwrap()).unwrap();

        assert_eq!(l_left.get_operation(), Operation::Equality);
        assert_eq!(l_right.get_operation(), Operation::GreaterThanOrEqual);

        let lr_left = tree.get_nodes().get(l_right.get_left().unwrap()).unwrap();
        let lr_right = tree.get_nodes().get(l_right.get_right().unwrap()).unwrap();

        assert_eq!(lr_left.get_operation(), Operation::Touch);
        assert_eq!(lr_left.get_value().get_data_type(), DataType::Integer);

        assert_eq!(lr_right.get_operation(), Operation::Touch);
        assert_eq!(lr_right.get_value().get_data_type(), DataType::Integer);

        let ll_left = tree.get_nodes().get(l_left.get_left().unwrap()).unwrap();
        let ll_right = tree.get_nodes().get(l_left.get_right().unwrap()).unwrap();

        assert_eq!(ll_left.get_operation(), Operation::Addition);

        assert_eq!(ll_right.get_operation(), Operation::Touch);
        assert_eq!(ll_right.get_value().get_data_type(), DataType::Integer);

        let lll_left = tree.get_nodes().get(ll_left.get_left().unwrap()).unwrap();
        let lll_right = tree.get_nodes().get(ll_left.get_right().unwrap()).unwrap();

        assert_eq!(lll_left.get_operation(), Operation::Touch);
        assert_eq!(lll_left.get_value().get_data_type(), DataType::Integer);

        assert_eq!(lll_right.get_operation(), Operation::Touch);
        assert_eq!(lll_right.get_value().get_data_type(), DataType::Integer);
    }

    #[test]
    fn logical_operations_3() {
        let input = String::from("true || 5 + 3 == 4 && 10 >= 8 || false");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        // tree should look like
        //               ___ "||" ___
        //              /            \
        //        ___ "||" ___      false
        //       /            \
        //     true        _ "&&" _
        //                /        \
        //              "=="      ">="
        //              /  \      /  \
        //            "+"   4    10   8
        //            / \
        //           5   3

        let root = tree.get_root();

        // for node in tree.get_nodes() {
        //     println!("{:?}", node);
        // }

        assert_eq!(root.get_operation(), Operation::LogicalOr);

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(left.get_operation(), Operation::LogicalOr);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_value().get_data_type(), DataType::Boolean);

        let l_left = tree.get_nodes().get(left.get_left().unwrap()).unwrap();
        let l_right = tree.get_nodes().get(left.get_right().unwrap()).unwrap();

        assert_eq!(l_left.get_operation(), Operation::Touch);
        assert_eq!(l_left.get_value().get_data_type(), DataType::Boolean);

        assert_eq!(l_right.get_operation(), Operation::LogicalAnd);

        let lr_left = tree.get_nodes().get(l_right.get_left().unwrap()).unwrap();
        let lr_right = tree.get_nodes().get(l_right.get_right().unwrap()).unwrap();

        assert_eq!(lr_left.get_operation(), Operation::Equality);
        assert_eq!(lr_right.get_operation(), Operation::GreaterThanOrEqual);

        let lrr_left = tree.get_nodes().get(lr_right.get_left().unwrap()).unwrap();
        let lrr_right = tree.get_nodes().get(lr_right.get_right().unwrap()).unwrap();

        assert_eq!(lrr_left.get_operation(), Operation::Touch);
        assert_eq!(lrr_left.get_value().get_data_type(), DataType::Integer);

        assert_eq!(lrr_right.get_operation(), Operation::Touch);
        assert_eq!(lrr_right.get_value().get_data_type(), DataType::Integer);

        let lrl_left = tree.get_nodes().get(lr_left.get_left().unwrap()).unwrap();
        let lrl_right = tree.get_nodes().get(lr_left.get_right().unwrap()).unwrap();

        assert_eq!(lrl_left.get_operation(), Operation::Addition);

        assert_eq!(lrl_right.get_operation(), Operation::Touch);
        assert_eq!(lrl_right.get_value().get_data_type(), DataType::Integer);

        let lrll_left = tree.get_nodes().get(lrl_left.get_left().unwrap()).unwrap();
        let lrll_right = tree.get_nodes().get(lrl_left.get_right().unwrap()).unwrap();

        assert_eq!(lrll_left.get_operation(), Operation::Touch);
        assert_eq!(lrll_left.get_value().get_data_type(), DataType::Integer);

        assert_eq!(lrll_right.get_operation(), Operation::Touch);
        assert_eq!(lrll_right.get_value().get_data_type(), DataType::Integer);
    }
}
