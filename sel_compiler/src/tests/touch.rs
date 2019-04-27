use super::super::compiler::Compiler;
use sel_common::{DataType, Operation};

#[test]
fn compiles_empty() {
    let input = String::from("");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    assert_eq!(root.get_operation(), Operation::None);
    assert_eq!(root.get_value(), None);
    assert_eq!(root.get_data_type(), DataType::Unit);
}

#[test]
fn compiles_unit() {
    let input = String::from("()");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    assert_eq!(root.get_operation(), Operation::Touch);
    assert_eq!(root.get_value(), None);
    assert_eq!(root.get_data_type(), DataType::Unit);
}

#[test]
fn compiles_input() {
    let input = String::from("$");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    assert_eq!(root.get_operation(), Operation::Input);
    assert_eq!(root.get_value(), None);
    assert_eq!(root.get_data_type(), DataType::Unknown);
}

#[test]
fn compiles_last_result() {
    let input = String::from("?");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    assert_eq!(root.get_operation(), Operation::CurrentResult);
    assert_eq!(root.get_value(), None);
    assert_eq!(root.get_data_type(), DataType::Unknown);
}

#[test]
fn compiles_touch_integer() {
    let input = String::from("9");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();
    let root_value: i64 = tree.get_integer_value_of(&root).unwrap();

    assert_eq!(root.get_operation(), Operation::Touch);
    assert_eq!(root.get_data_type(), DataType::Integer);
    assert_eq!(root_value, 9);
}

#[test]
fn compiles_touch_decimal() {
    let input = String::from("3.14");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let root_value: f64 = tree.get_decimal_value_of(&root).unwrap();

    assert_eq!(root.get_operation(), Operation::Touch);
    assert_eq!(root.get_data_type(), DataType::Decimal);
    assert_eq!(root_value, 3.14);
}

#[test]
fn compiles_touch_single_quote_string() {
    let input = String::from("'hello world'");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let root_value: String = tree.get_string_value_of(&root).unwrap();

    assert_eq!(root.get_operation(), Operation::Touch);
    assert_eq!(root.get_data_type(), DataType::String);
    assert_eq!(root_value, "hello world");
}

#[test]
fn compiles_touch_double_quote_string() {
    let input = String::from("\"hello world\"");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let root_value: String = tree.get_string_value_of(&root).unwrap();

    assert_eq!(root.get_operation(), Operation::Touch);
    assert_eq!(root.get_data_type(), DataType::String);
    assert_eq!(root_value, "hello world");
}

#[test]
fn compiles_touch_boolean() {
    let input = String::from("true");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let root_value: bool = tree.get_boolean_value_of(&root).unwrap();

    assert_eq!(root.get_operation(), Operation::Touch);
    assert_eq!(root.get_data_type(), DataType::Boolean);
    assert_eq!(root_value, true);
}

#[test]
fn compiles_touch_identifier() {
    let input = String::from("value");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let root_value = tree.get_value_bytes_of(&root);

    assert_eq!(root.get_operation(), Operation::Touch);
    assert_eq!(root.get_data_type(), DataType::Identifier);
    assert_eq!(root_value, None);
}

#[test]
fn compiles_symbol_operation() {
    let input = String::from(":value");
    let compiler = Compiler::new();

    let tree = compiler.compile(&input);

    let root = tree.get_root();

    let right = tree.get_nodes().get(root.get_right().unwrap()).unwrap();

    let root_value = tree.get_integer_value_of(&root);

    let symbol = tree.get_symbol_table().get(root_value.unwrap());

    assert_eq!(root.get_operation(), Operation::Touch);
    assert_eq!(root.get_data_type(), DataType::Symbol);
    assert_eq!(root_value, Some(0));
    assert_eq!(symbol, "value");
}
