use sel_common::{from_byte_vec, DataType, SELContext, SELValue};
use sel_compiler;
use sel_executor;
use std::f64::consts::*;
// use sel_tokenizer;

const INPUTS: [&str; 21] = [
    "5 + $ * 8 - 3",
    "3 > 5 || 89 != 43",
    "true ^^ true",
    "10 // 3",
    "3.14 ** 2",
    "'Hello ' + 'World'",
    "5 + 9 * 3 + ()",
    "10 * .5",
    "\
5 + $ * 3 / 7

'Result is: ' + ?
'Input is: ' + $",
    "5.0 * (8 + 2) / (3 + (9 - 4)) * ((4 + 7) * 3)",
    "\
0..100
0...100",
    "E",
    "E + 5",
    ":my_symbol",
    ":email = \"panda@example.com\"",
    ":user = :first_name = \"Panda\"",
    "PI = \"Panda\"",
    "100, true, E, 10 > PI, $, \"pandas\", :my_symbol = 500",
    "\
(0, 0), 
(50, 40), 
(20, 90)

?.2.0
",
    "\
:name = (:first = \"Panda\", :last = \"Bear\")

?.right.1.right
",
    "sin(PI), cos(PI), tan(PI)",
];

fn main() {
    println!("{}", "-".repeat(100));
    for input in INPUTS.iter() {
        let mut context = SELContext::new();
        context.set_decimal_symbol(&String::from("PI"), PI);
        context.set_decimal_symbol(&String::from("E"), E);

        context.register_function("sin", |sel_value| match sel_value.get_type() {
            DataType::Integer => sel_value
                .get_value()
                .map(|bytes| from_byte_vec::<i64>(bytes))
                .map_or(SELValue::new(), |val| {
                    SELValue::new_from_decimal(f64::sin(val as f64))
                }),
            DataType::Decimal => sel_value
                .get_value()
                .map(|bytes| from_byte_vec::<f64>(bytes))
                .map_or(SELValue::new(), |val| {
                    SELValue::new_from_decimal(f64::sin(val))
                }),
            _ => SELValue::new(),
        });

        context.register_function("cos", |sel_value| match sel_value.get_type() {
            DataType::Integer => sel_value
                .get_value()
                .map(|bytes| from_byte_vec::<i64>(bytes))
                .map_or(SELValue::new(), |val| {
                    SELValue::new_from_decimal(f64::cos(val as f64))
                }),
            DataType::Decimal => sel_value
                .get_value()
                .map(|bytes| from_byte_vec::<f64>(bytes))
                .map_or(SELValue::new(), |val| {
                    SELValue::new_from_decimal(f64::cos(val))
                }),
            _ => SELValue::new(),
        });

        context.register_function("tan", |sel_value| match sel_value.get_type() {
            DataType::Integer => sel_value
                .get_value()
                .map(|bytes| from_byte_vec::<i64>(bytes))
                .map_or(SELValue::new(), |val| {
                    SELValue::new_from_decimal(f64::tan(val as f64))
                }),
            DataType::Decimal => sel_value
                .get_value()
                .map(|bytes| from_byte_vec::<f64>(bytes))
                .map_or(SELValue::new(), |val| {
                    SELValue::new_from_decimal(f64::tan(val))
                }),
            _ => SELValue::new(),
        });

        let compiler = sel_compiler::Compiler::new();

        let mut execution_context = sel_executor::SELExecutionContext::from(&context);

        execution_context.set_input(sel_common::SELValue::new_from_int(12345));

        let input_str = String::from(*input);
        let tree = compiler.compile_with_context(&input_str, context);

        let results = sel_executor::execute_sel_tree(&tree, &execution_context);

        println!("{}", input_str);
        for (result_index, result) in results.iter().enumerate() {
            println!("{}: {}", result_index + 1, result);
        }
        println!("{}", "-".repeat(100));
    }
}
