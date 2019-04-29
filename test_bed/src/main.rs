use sel_common::SELContext;
use sel_compiler;
use sel_executor;
// use sel_tokenizer;

const INPUTS: [&str; 20] = [
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
    "count",
    "count + 5",
    ":my_symbol",
    ":email = \"panda@example.com\"",
    ":user = :first_name = \"Panda\"",
    "count = \"Panda\"",
    "100, true, count, 10 > count, $, \"pandas\", :my_symbol = 500",
    "\
(0, 0), (50, 40), (20, 90)

?.2.0
",
    "\
:name = (:first = \"Panda\", :last = \"Bear\")

?.right.1.right
",
];

fn main() {
    println!("{}", "-".repeat(100));
    for input in INPUTS.iter() {
        let mut context = SELContext::new();
        context.set_integer_symbol(&String::from("count"), 100);

        let compiler = sel_compiler::Compiler::new();

        let mut execution_context = sel_executor::SELExecutionContext::new();

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
