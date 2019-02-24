use sel_tokenizer;

fn main() {
    let input = String::from("5 + 4 * 8 - 3");
    let tokenizer = sel_tokenizer::Tokenizer::new(&input);
    for token in tokenizer {
        println!("{:?}", token);
    }
}
