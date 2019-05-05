use super::super::Compiler;
use sel_common::{DataType, Operation};

#[test]
fn single_empty_expression_block() {
    let input = String::from("{}");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    // tree should look like
    //         E

    let root = tree.get_root();

    assert_eq!(root.get_operation(), Operation::Expression);
    assert_eq!(root.get_data_type(), DataType::Unknown);
    assert_eq!(root.get_parent(), None);
    assert_eq!(root.get_right(), None);
}

#[test]
fn single_expression_block() {
    let input = String::from("{5 + 10}");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    // tree should look like
    //         E
    //          \
    //          +
    //         / \
    //        5  10

    let root = tree.get_root();

    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    let r_left = tree.get_nodes().get(right.get_left().unwrap()).unwrap();
    let r_right = tree.get_nodes().get(right.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Expression);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(right.get_operation(), Operation::Addition);
    assert_eq!(right.get_data_type(), DataType::Unknown);

    assert_eq!(r_left.get_operation(), Operation::Touch);
    assert_eq!(r_left.get_data_type(), DataType::Integer);

    assert_eq!(r_right.get_operation(), Operation::Touch);
    assert_eq!(r_right.get_data_type(), DataType::Integer);
}

#[test]
fn multi_line_nested_expression_main_roots() {
    let input = String::from(
        "\
1

{
    2

    {
        3

        4
    }

    5
}

6
",
    );

    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    assert_eq!(tree.get_root().get_own_index(), 0);
    assert_eq!(tree.get_sub_roots().len(), 2);
    assert_eq!(*tree.get_sub_roots().get(0).unwrap(), 1);
    assert_eq!(*tree.get_sub_roots().get(1).unwrap(), 7);
}

#[test]
fn multi_line_nested_expression_nested_expression_roots() {
    let input = String::from(
        "\
1

{
    2

    {
        3

        4
    }

    5
}

6
",
    );

    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    assert_eq!(tree.get_sub_trees().len(), 2);

    let sub_tree_1 = tree.get_sub_trees().get(0).unwrap();
    let sub_tree_2 = tree.get_sub_trees().get(1).unwrap();

    // sub trees get inserted in evaluation order
    // meaning the inner most expression blocks appear first

    assert_eq!(sub_tree_1.get_roots().len(), 2);
    assert_eq!(*sub_tree_1.get_roots().get(0).unwrap(), 4);
    assert_eq!(*sub_tree_1.get_roots().get(1).unwrap(), 5);

    assert_eq!(sub_tree_2.get_roots().len(), 3);
    assert_eq!(*sub_tree_2.get_roots().get(0).unwrap(), 2);
    assert_eq!(*sub_tree_2.get_roots().get(1).unwrap(), 3);
    assert_eq!(*sub_tree_2.get_roots().get(2).unwrap(), 6);
}

#[test]
fn multi_line_nested_expression_expression_value_is_sub_tree_index() {
    let input = String::from(
        "\
1

{
    2

    {
        3

        4
    }

    5
}

6
",
    );

    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let first_expr = tree.get_nodes().get(1).unwrap();
    let second_expr = tree.get_nodes().get(3).unwrap();

    let first_expr_value = tree.get_usize_value_of(first_expr).unwrap();
    let second_expr_value = tree.get_usize_value_of(second_expr).unwrap();

    // sub trees get inserted in evaluation order
    // meaning the inner most expression blocks appear first
    assert_eq!(first_expr_value, 1);
    assert_eq!(second_expr_value, 0);
}
#[test]
fn single_line_expression_roots() {
    let input = String::from("10 -> { $ * 10 }");

    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let sub_tree = tree.get_sub_trees().get(0).unwrap();

    assert_eq!(*sub_tree.get_roots().get(0).unwrap(), 4);
}
