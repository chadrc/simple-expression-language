use super::super::Compiler;
use sel_common::{DataType, Operation};

#[test]
fn no_argument_with_parenthesis() {
    let input = String::from("get_vars()");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    // tree should look like
    //         _ G _
    //        /     \
    //   get_vars

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Group);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Identifier);
}

#[test]
fn single_argument_with_parenthesis() {
    let input = String::from("get_vars(5)");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    // tree should look like
    //         _ G _
    //        /     \
    //   get_vars   5

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Group);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Identifier);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn single_argument_call_preceding_op() {
    let input = String::from("5 + get_vars(5)");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    // tree should look like
    //    __ + __
    //   /       \
    //  5      _ G _
    //        /     \
    //   get_vars   5

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    let r_left = tree.get_nodes().get(right.get_left().unwrap()).unwrap();
    let r_right = tree.get_nodes().get(right.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Addition);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Group);

    assert_eq!(r_left.get_operation(), Operation::Touch);
    assert_eq!(r_left.get_data_type(), DataType::Identifier);

    assert_eq!(r_right.get_operation(), Operation::Touch);
    assert_eq!(r_right.get_data_type(), DataType::Integer);
}
