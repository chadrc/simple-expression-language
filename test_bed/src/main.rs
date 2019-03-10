use sel_common;
use sel_compiler;
use sel_executor;
// use sel_tokenizer;

fn main() {
    let input = String::from("5 + 400 * 8 - 3");
    // let tokenizer = sel_tokenizer::Tokenizer::new(&input);
    // for token in tokenizer {
    //     println!("{:?}", token);
    // }

    let compiler = sel_compiler::Compiler::new();
    let tree = compiler.compile(&input);

    let result = sel_executor::execute_sel_tree(tree);

    let result_val: Option<i32> = match result.get_value() {
        Some(val) => Some(sel_common::from_byte_vec(val)),
        None => None,
    };

    println!("result: {:?}", result.get_type());
    println!("result: {:?}", result_val);
}
