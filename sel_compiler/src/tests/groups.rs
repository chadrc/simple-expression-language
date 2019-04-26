use super::super::Compiler;
use sel_common::{DataType, Operation};

#[test]
fn single_group() {
    let input = String::from("(5 + 10)");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    // tree should look like
    //         G
    //          \
    //          +
    //         / \
    //        5  10

    let root = tree.get_root();

    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    let r_left = tree.get_nodes().get(right.get_left().unwrap()).unwrap();
    let r_right = tree.get_nodes().get(right.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Group);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(right.get_operation(), Operation::Addition);
    assert_eq!(right.get_data_type(), DataType::Unknown);

    assert_eq!(r_left.get_operation(), Operation::Touch);
    assert_eq!(r_left.get_data_type(), DataType::Integer);

    assert_eq!(r_right.get_operation(), Operation::Touch);
    assert_eq!(r_right.get_data_type(), DataType::Integer);
}

#[test]
fn single_group_end() {
    let input = String::from("5 * (10 + 15)");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    // tree should look like
    //          *
    //         / \
    //        5   G
    //             \
    //             +
    //            / \
    //          15  10

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    let r_right = tree.get_nodes().get(right.get_right().unwrap()).unwrap();

    let rr_left = tree.get_nodes().get(r_right.get_left().unwrap()).unwrap();
    let rr_right = tree.get_nodes().get(r_right.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Multiplication);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Group);
    assert_eq!(right.get_data_type(), DataType::Unknown);

    assert_eq!(r_right.get_operation(), Operation::Addition);
    assert_eq!(r_right.get_data_type(), DataType::Unknown);

    assert_eq!(rr_left.get_operation(), Operation::Touch);
    assert_eq!(rr_left.get_data_type(), DataType::Integer);

    assert_eq!(rr_right.get_operation(), Operation::Touch);
    assert_eq!(rr_right.get_data_type(), DataType::Integer);
}

#[test]
fn single_group_begin() {
    let input = String::from("(5 + 10) * 15");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    // tree should look like
    //            *
    //           / \
    //          G  15
    //          \
    //          +
    //         / \
    //        5  10

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    let l_right = tree.get_nodes().get(left.get_right().unwrap()).unwrap();

    let ll_left = tree.get_nodes().get(l_right.get_left().unwrap()).unwrap();
    let ll_right = tree.get_nodes().get(l_right.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Multiplication);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Group);
    assert_eq!(left.get_data_type(), DataType::Unknown);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);

    assert_eq!(l_right.get_operation(), Operation::Addition);
    assert_eq!(l_right.get_data_type(), DataType::Unknown);

    assert_eq!(ll_left.get_operation(), Operation::Touch);
    assert_eq!(ll_left.get_data_type(), DataType::Integer);

    assert_eq!(ll_right.get_operation(), Operation::Touch);
    assert_eq!(ll_right.get_data_type(), DataType::Integer);
}

#[test]
fn single_group_begin_end() {
    let input = String::from("5 * (5 + 10) * 15");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    // tree should look like
    //            *
    //           / \
    //          *  15
    //         / \
    //        5   G
    //             \
    //             +
    //            / \
    //           5  10

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    let l_left = tree.get_nodes().get(left.get_left().unwrap()).unwrap();
    let l_right = tree.get_nodes().get(left.get_right().unwrap()).unwrap();

    let lr_right = tree.get_nodes().get(l_right.get_right().unwrap()).unwrap();

    let lrr_left = tree.get_nodes().get(lr_right.get_left().unwrap()).unwrap();
    let lrr_right = tree.get_nodes().get(lr_right.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Multiplication);

    assert_eq!(left.get_operation(), Operation::Multiplication);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);

    assert_eq!(l_left.get_operation(), Operation::Touch);
    assert_eq!(l_left.get_data_type(), DataType::Integer);

    assert_eq!(l_right.get_operation(), Operation::Group);

    assert_eq!(lr_right.get_operation(), Operation::Addition);

    assert_eq!(lrr_left.get_operation(), Operation::Touch);
    assert_eq!(lrr_left.get_data_type(), DataType::Integer);

    assert_eq!(lrr_right.get_operation(), Operation::Touch);
    assert_eq!(lrr_right.get_data_type(), DataType::Integer);
}

#[test]
fn single_group_begin_end_lower_begin() {
    let input = String::from("5 + (5 + 10) * 15");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    // tree should look like
    //            +
    //           / \
    //          5   *
    //             / \
    //            G  15
    //            \
    //            +
    //           / \
    //          5  10

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    let r_left = tree.get_nodes().get(right.get_left().unwrap()).unwrap();
    let r_right = tree.get_nodes().get(right.get_right().unwrap()).unwrap();

    let rl_right = tree.get_nodes().get(r_left.get_right().unwrap()).unwrap();

    let rlr_left = tree.get_nodes().get(rl_right.get_left().unwrap()).unwrap();
    let rlr_right = tree.get_nodes().get(rl_right.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Addition);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Multiplication);

    assert_eq!(r_left.get_operation(), Operation::Group);

    assert_eq!(rl_right.get_operation(), Operation::Addition);

    assert_eq!(r_right.get_operation(), Operation::Touch);
    assert_eq!(r_right.get_data_type(), DataType::Integer);

    assert_eq!(rlr_left.get_operation(), Operation::Touch);
    assert_eq!(rlr_left.get_data_type(), DataType::Integer);

    assert_eq!(rlr_right.get_operation(), Operation::Touch);
    assert_eq!(rlr_right.get_data_type(), DataType::Integer);
}

#[test]
fn multiple_groups_siblings() {
    let input = String::from("5 + (5 + 10) * 15 - (10 - 4) + 7");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    // tree should look like
    //                       + ____
    //                     /       \
    //               ____ - ____   7
    //             /            \
    //            +             G
    //           / \             \
    //          5   *            -
    //             / \          / \
    //            G  15       10  4
    //            \
    //            +
    //           / \
    //          5  10

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    let l_left = tree.get_nodes().get(left.get_left().unwrap()).unwrap();
    let l_right = tree.get_nodes().get(left.get_right().unwrap()).unwrap();

    let ll_left = tree.get_nodes().get(l_left.get_left().unwrap()).unwrap();
    let ll_right = tree.get_nodes().get(l_left.get_right().unwrap()).unwrap();

    let llr_left = tree.get_nodes().get(ll_right.get_left().unwrap()).unwrap();
    let llr_right = tree.get_nodes().get(ll_right.get_right().unwrap()).unwrap();

    let llrl_right = tree.get_nodes().get(llr_left.get_right().unwrap()).unwrap();

    let llrlr_left = tree
        .get_nodes()
        .get(llrl_right.get_left().unwrap())
        .unwrap();
    let llrlr_right = tree
        .get_nodes()
        .get(llrl_right.get_right().unwrap())
        .unwrap();

    let lr_right = tree.get_nodes().get(l_right.get_right().unwrap()).unwrap();

    let lrr_left = tree.get_nodes().get(lr_right.get_left().unwrap()).unwrap();
    let lrr_right = tree.get_nodes().get(lr_right.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Addition);

    assert_eq!(left.get_operation(), Operation::Subtraction);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);

    assert_eq!(l_left.get_operation(), Operation::Addition);
    assert_eq!(l_right.get_operation(), Operation::Group);

    assert_eq!(ll_left.get_operation(), Operation::Touch);
    assert_eq!(ll_left.get_data_type(), DataType::Integer);

    assert_eq!(ll_right.get_operation(), Operation::Multiplication);

    assert_eq!(llr_left.get_operation(), Operation::Group);

    assert_eq!(llr_right.get_operation(), Operation::Touch);
    assert_eq!(llr_right.get_data_type(), DataType::Integer);

    assert_eq!(llrl_right.get_operation(), Operation::Addition);

    assert_eq!(llrlr_left.get_operation(), Operation::Touch);
    assert_eq!(llrlr_left.get_data_type(), DataType::Integer);

    assert_eq!(llrlr_right.get_operation(), Operation::Touch);
    assert_eq!(llrlr_right.get_data_type(), DataType::Integer);

    assert_eq!(lr_right.get_operation(), Operation::Subtraction);

    assert_eq!(lrr_left.get_operation(), Operation::Touch);
    assert_eq!(lrr_left.get_data_type(), DataType::Integer);

    assert_eq!(lrr_right.get_operation(), Operation::Touch);
    assert_eq!(lrr_right.get_data_type(), DataType::Integer);
}

#[test]
fn multiple_groups_nested() {
    let input = String::from("5 + (3 * (5 + 3))");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    println!("{:?}", tree);

    // tree should look like
    //              +
    //            /  \
    //           5    G
    //                 \
    //                 *
    //                / \
    //               3   G
    //                    \
    //                    +
    //                   / \
    //                  5   3

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    let r_right = tree.get_nodes().get(right.get_right().unwrap()).unwrap();

    let rr_left = tree.get_nodes().get(r_right.get_left().unwrap()).unwrap();
    let rr_right = tree.get_nodes().get(r_right.get_right().unwrap()).unwrap();

    let rrr_right = tree.get_nodes().get(rr_right.get_right().unwrap()).unwrap();

    let rrrr_left = tree.get_nodes().get(rrr_right.get_left().unwrap()).unwrap();
    let rrrr_right = tree
        .get_nodes()
        .get(rrr_right.get_right().unwrap())
        .unwrap();

    assert_eq!(root.get_operation(), Operation::Addition);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Group);

    assert_eq!(r_right.get_operation(), Operation::Multiplication);

    assert_eq!(rr_left.get_operation(), Operation::Touch);
    assert_eq!(rr_left.get_data_type(), DataType::Integer);

    assert_eq!(rr_right.get_operation(), Operation::Group);

    assert_eq!(rrr_right.get_operation(), Operation::Addition);

    assert_eq!(rrrr_left.get_operation(), Operation::Touch);
    assert_eq!(rrrr_left.get_data_type(), DataType::Integer);

    assert_eq!(rrrr_right.get_operation(), Operation::Touch);
    assert_eq!(rrrr_right.get_data_type(), DataType::Integer);
}
