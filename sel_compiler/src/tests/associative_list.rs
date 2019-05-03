use super::super::Compiler;
use sel_common::{DataType, Operation};

#[test]
fn single_associative_list() {
    let input = String::from("[5 + 10]");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    // tree should look like
    //         A
    //          \
    //          +
    //         / \
    //        5  10

    let root = tree.get_root();

    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    let r_left = tree.get_nodes().get(right.get_left().unwrap()).unwrap();
    let r_right = tree.get_nodes().get(right.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::AssociativeList);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(right.get_operation(), Operation::Addition);
    assert_eq!(right.get_data_type(), DataType::Unknown);

    assert_eq!(r_left.get_operation(), Operation::Touch);
    assert_eq!(r_left.get_data_type(), DataType::Integer);

    assert_eq!(r_right.get_operation(), Operation::Touch);
    assert_eq!(r_right.get_data_type(), DataType::Integer);
}
