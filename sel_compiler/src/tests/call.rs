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
