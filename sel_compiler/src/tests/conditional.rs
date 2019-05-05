use crate::Compiler;
use sel_common::{DataType, Operation};

#[test]
fn compiles_match_list() {
    let input = String::from("value => 10, value !=> 20");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    let l_left = tree.get_nodes().get(left.get_left().unwrap()).unwrap();
    let l_right = tree.get_nodes().get(left.get_right().unwrap()).unwrap();

    let r_left = tree.get_nodes().get(right.get_left().unwrap()).unwrap();
    let r_right = tree.get_nodes().get(right.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::MatchList);

    assert_eq!(left.get_operation(), Operation::MatchTrue);

    assert_eq!(right.get_operation(), Operation::MatchFalse);

    assert_eq!(l_left.get_operation(), Operation::Touch);
    assert_eq!(l_left.get_data_type(), DataType::Identifier);

    assert_eq!(l_right.get_operation(), Operation::Touch);
    assert_eq!(l_right.get_data_type(), DataType::Integer);

    assert_eq!(r_left.get_operation(), Operation::Touch);
    assert_eq!(r_left.get_data_type(), DataType::Identifier);

    assert_eq!(r_right.get_operation(), Operation::Touch);
    assert_eq!(r_right.get_data_type(), DataType::Integer);
}
