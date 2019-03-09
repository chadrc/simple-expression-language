#[cfg(test)]
mod tests {
    use super::super::compiler::Compiler;
    use sel_common::{DataType, Operation};

    #[test]
    fn compiles_addition_operation() {
        let input = String::from("5 + 10");
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
    fn compiles_multiplication_operation() {
        let input = String::from("5 * 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Multiplication);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_subtraction_operation() {
        let input = String::from("5 - 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Subtraction);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_division_operation() {
        let input = String::from("5 / 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Division);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_modulus_operation() {
        let input = String::from("5 % 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Modulo);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_exponential_operation() {
        let input = String::from("5^10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Exponential);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_exclusive_range_operation() {
        let input = String::from("5..10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::ExclusiveRange);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_inclusive_range_operation() {
        let input = String::from("5...10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::InclusiveRange);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_greater_than_operation() {
        let input = String::from("5 > 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::GreaterThan);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_greater_than_equal_operation() {
        let input = String::from("5 >= 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::GreaterThanOrEqual);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_less_than_operation() {
        let input = String::from("5 < 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::LessThan);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_less_than_equal_to_operation() {
        let input = String::from("5 <= 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::LessThanOrEqual);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_equality_operation() {
        let input = String::from("5 == 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Equality);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_inequality_operation() {
        let input = String::from("5 != 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Inequality);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_logical_and_operation() {
        let input = String::from("5 && 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::LogicalAnd);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_logical_or_operation() {
        let input = String::from("5 || 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::LogicalOr);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_data_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }

    #[test]
    fn compiles_logical_not_operation() {
        let input = String::from("!true");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::LogicalNot);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Boolean);
    }

    #[test]
    fn compiles_negation_operation() {
        let input = String::from("-4");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Negation);
        assert_eq!(root.get_data_type(), DataType::Unknown);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_data_type(), DataType::Integer);
    }
}
