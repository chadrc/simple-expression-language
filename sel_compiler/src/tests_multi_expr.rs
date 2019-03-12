#[cfg(test)]
mod tests {
    use super::super::compiler::Compiler;
    use sel_common::{DataType, Operation};

    #[test]
    fn two_expressions() {
        let input = String::from("5 + 10\n15 + 20");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        println!("nodes {:?}", tree.get_nodes());
        println!("roots {:?}", tree.get_sub_roots());

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Addition);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);

        let root_2 = tree.get_sub_root(0).unwrap();

        let left_2 = tree.get_nodes().get(root_2.get_left().unwrap()).unwrap();
        let right_2 = tree.get_nodes().get(root_2.get_right().unwrap()).unwrap();

        assert_eq!(root_2.get_operation(), Operation::Addition);
        assert_eq!(root_2.get_data_type(), DataType::Unknown);

        assert_eq!(left_2.get_operation(), Operation::Touch);
        assert_eq!(left_2.get_data_type(), DataType::Integer);

        assert_eq!(right_2.get_operation(), Operation::Touch);
        assert_eq!(right_2.get_data_type(), DataType::Integer);
    }
}
