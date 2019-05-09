use super::super::compiler::Compiler;
use sel_common::named_expression::NamedExpression;
use sel_common::{DataType, Operation};
use std::collections::HashMap;

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
fn compiles_integer_division_operation() {
    let input = String::from("5 // 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::IntegerDivision);
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
    let input = String::from("5**10");
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
fn compiles_keys_equal() {
    let input = String::from("(,) := 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    println!("{:?}", tree);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::KeysEqual);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Group);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_keys_not_equal() {
    let input = String::from("(,) :!= 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::KeysNotEqual);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Group);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_values_equal() {
    let input = String::from("(,) $= 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::ValuesEqual);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Group);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_values_not_equal() {
    let input = String::from("(,) $!= 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::ValuesNotEqual);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Group);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_contains() {
    let input = String::from("(,) ~= 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Contains);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Group);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_not_contains() {
    let input = String::from("(,) ~!= 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::NotContains);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Group);

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
fn compiles_logical_xor_operation() {
    let input = String::from("5 ^^ 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::LogicalXOR);
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

    assert_eq!(root.get_operation(), Operation::Not);
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

#[test]
fn compiles_bitwise_or_operation() {
    let input = String::from("5 | 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::BitwiseOr);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_bitwise_and_operation() {
    let input = String::from("5 & 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::BitwiseAnd);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_bitwise_xor_operation() {
    let input = String::from("5 ^ 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::BitwiseXOR);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_bitwise_left_shift_operation() {
    let input = String::from("5 << 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::BitwiseLeftShift);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_bitwise_right_shift_operation() {
    let input = String::from("5 >> 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::BitwiseRightShift);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_bitwise_not_operation() {
    let input = String::from("!4");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Not);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_pair_operation() {
    let input = String::from("value = 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Pair);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Identifier);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_dot_access_operation() {
    let input = String::from("value.field");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::DotAccess);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Identifier);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Identifier);
}

#[test]
fn compiles_interpreted_access_operation() {
    let input = String::from("value[\"field\"]");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::InterpretedAccess);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Identifier);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::String);
}

#[test]
fn compiles_list_operation() {
    let input = String::from("value, 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::List);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Identifier);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_pipe_first_right() {
    let input = String::from("10 -> func");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::PipeFirstRight);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Identifier);
}

#[test]
fn compiles_partial_arguments() {
    let input = String::from("func ~ 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::PartialApplication);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Identifier);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_infix_function() {
    let input = String::from("5 `infix` 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let root_value = tree.get_usize_value_of(root);
    let symbol = tree.get_symbol_table().get_symbol(root_value.unwrap());

    assert_eq!(root_value, Some(0));
    assert_eq!(symbol, Some(&String::from("infix")));

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::InfixCall);
    assert_eq!(root.get_data_type(), DataType::Identifier);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_named_expression() {
    let input = String::from("#my_expression $ + 5");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let named_expressions = tree.get_named_expressions();

    assert_eq!(named_expressions.len(), 1);

    let symbol = tree
        .get_symbol_table()
        .get_value(&String::from("my_expression"))
        .unwrap();

    let named_expression = named_expressions.get(symbol).unwrap();

    assert_eq!(named_expression.get_symbol(), 0);
    assert_eq!(named_expression.get_root(), 2);

    let root = tree.get_nodes().get(named_expression.get_root()).unwrap();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Addition);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Input);
    assert_eq!(left.get_data_type(), DataType::Unknown);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_pipe_first_left() {
    let input = String::from("func <- 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::PipeFirstLeft);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Identifier);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_pipe_last_right() {
    let input = String::from("10 |> func");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::PipeLastRight);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Integer);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Identifier);
}

#[test]
fn compiles_pipe_last_left() {
    let input = String::from("func <| 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::PipeLastLeft);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Identifier);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_match_true() {
    let input = String::from("value => 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::MatchTrue);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Identifier);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_match_false() {
    let input = String::from("value !=> 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::MatchFalse);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Identifier);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_stream() {
    let input = String::from("value >>> 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Stream);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Identifier);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_collect() {
    let input = String::from("value >- 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::Collect);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Identifier);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}

#[test]
fn compiles_collect_init() {
    let input = String::from("value -< 10");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let left = tree.get_nodes().get(root.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    assert_eq!(root.get_operation(), Operation::CollectInit);
    assert_eq!(root.get_data_type(), DataType::Unknown);

    assert_eq!(left.get_operation(), Operation::Touch);
    assert_eq!(left.get_data_type(), DataType::Identifier);

    assert_eq!(right.get_operation(), Operation::Touch);
    assert_eq!(right.get_data_type(), DataType::Integer);
}
