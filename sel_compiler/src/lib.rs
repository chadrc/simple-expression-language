mod build;
mod change;
mod compiler;
mod find_root;
mod group_handling;
mod precedence_manager;
mod process_tokens;
mod resolve_node;
mod resolve_tree;
mod tokenizer;
mod utils;

#[cfg(test)]
mod tests;

pub use compiler::Compiler;

#[cfg(test)]
mod compiler_tests {
    use super::compiler::Compiler;

    #[test]
    fn create_compiler() {
        Compiler::new();
    }
}
