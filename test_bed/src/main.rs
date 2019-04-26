use sel_compiler;
use sel_executor;
// use sel_tokenizer;

fn main() {
    let inputs = [
        "5 + $ * 8 - 3",
        "3 > 5 || 89 != 43",
        "$ * 3.14^2",
        "'Hello ' + 'World'",
        "5 + 9 * 3 + ()",
        "
        5 + $ * 3 / 7

        'Result is: ' + ?
        'Input is: ' + $
        ",
        "5.0 * (8 + 2) / (3 + (9 - 4)) * ((4 + 7) * 3)",
        "5..100",
    ];
    // let tokenizer = sel_tokenizer::Tokenizer::new(&input);
    // for token in tokenizer {
    //     println!("{:?}", token);
    // }

    let compiler = sel_compiler::Compiler::new();

    let mut context = sel_executor::SELContext::new();

    context.set_input(sel_executor::SELValue::new_from_int(12345));

    for (index, input) in inputs.iter().enumerate() {
        let tree = compiler.compile(&String::from(*input));

        let results = sel_executor::execute_sel_tree(&tree, &context);

        for (result_index, result) in results.iter().enumerate() {
            println!("({}:{}) {}", index, result_index, result);
        }
    }
}
