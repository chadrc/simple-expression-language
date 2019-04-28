use sel_common::SELContext;
use sel_compiler;
use sel_executor;
// use sel_tokenizer;

const INPUTS: [&str; 14] = [
    "5 + $ * 8 - 3",
    "3 > 5 || 89 != 43",
    "true ^^ true",
    "10 // 3",
    "3.14 ** 2",
    "'Hello ' + 'World'",
    "5 + 9 * 3 + ()",
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
];

fn main() {
    // let tokenizer = sel_tokenizer::Tokenizer::new(&input);
    // for token in tokenizer {
    //     println!("{:?}", token);
    // }

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
