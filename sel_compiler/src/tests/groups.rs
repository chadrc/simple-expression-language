use super::super::Compiler;
use sel_common::{DataType, Operation};

#[test]
fn single_group_end() {
    let input = String::from("5 * (10 + 15)");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    // tree should look like
    //          *
    //         / \
    //        5   +
    //           / \
    //         15   10

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    let r2_left = tree.get_nodes().get(right.get_left().unwrap()).unwrap();
    let r2_right = tree.get_nodes().get(right.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Multiplication);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Addition);
    assert_eq!(right.get_data_type(), DataType::Unknown);

    assert_eq!(r2_left.get_operation(), Operation::Touch);
    assert_eq!(r2_left.get_data_type(), DataType::Integer);

    assert_eq!(r2_right.get_operation(), Operation::Touch);
    assert_eq!(r2_right.get_data_type(), DataType::Integer);
}

#[test]
fn single_group_begin() {
    let input = String::from("(5 + 10) * 15");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    // tree should look like
    //          *
    //         / \
    //        +   15
    //       / \
    //      5   10

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    let l2_left = tree.get_nodes().get(left.get_left().unwrap()).unwrap();
    let l2_right = tree.get_nodes().get(left.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Multiplication);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Addition);
    assert_eq!(left.get_data_type(), DataType::Unknown);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);

    assert_eq!(l2_left.get_operation(), Operation::Touch);
    assert_eq!(l2_left.get_data_type(), DataType::Integer);

    assert_eq!(l2_right.get_operation(), Operation::Touch);
    assert_eq!(l2_right.get_data_type(), DataType::Integer);
}
