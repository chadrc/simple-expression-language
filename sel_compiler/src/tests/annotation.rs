use super::super::Compiler;
use sel_common::{DataType, Operation};

#[test]
fn only_comment_empty_tree() {
    let input = String::from("@ this is a comment");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    assert_eq!(root.get_operation(), Operation::None);
    assert_eq!(root.get_value(), None);
    assert_eq!(root.get_data_type(), DataType::Unit);
}

#[test]
fn only_comment_with_following_expression() {
    let input = String::from("@ this is a comment\n5 + 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Addition);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn expression_with_following_comment() {
    let input = String::from("5 + 10 @ this is a comment");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Addition);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn expression_with_annotation_block() {
    let input = String::from("@@ this is a comment\n@@ with a second line\n5 + 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Addition);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}
