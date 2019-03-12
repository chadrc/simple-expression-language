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
    ];
    // let tokenizer = sel_tokenizer::Tokenizer::new(&input);
    // for token in tokenizer {
    //     println!("{:?}", token);
    // }

    let compiler = sel_compiler::Compiler::new();

    let mut context = sel_executor::SELContext::new();

    context.set_input(sel_executor::SELValue::new_from_int(12345));

    for input in inputs.iter() {
        let tree = compiler.compile(&String::from(*input));

        let result = sel_executor::execute_sel_tree(&tree, &context);
        println!("{}", result);
    }
}
