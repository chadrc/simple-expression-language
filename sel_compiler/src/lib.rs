mod compiler;
mod precedence_manager;
mod sel_tree_builder;
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
