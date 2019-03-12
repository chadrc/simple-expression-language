mod compiler;
mod precedence_manager;
mod sel_tree_builder;
mod utils;

mod tests_multi_op;
mod tests_single_op;
mod tests_touch;
mod tests_multi_expr;

pub use compiler::Compiler;

#[cfg(test)]
mod tests {
    use super::compiler::Compiler;

    #[test]
    fn create_compiler() {
        Compiler::new();
    }
}
