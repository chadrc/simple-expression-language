mod compiler;
mod data_type;
mod operation;
mod sel_tree;

mod tests_single_op;
mod tests_touch;

pub use compiler::Compiler;

#[cfg(test)]
mod tests {
    use super::compiler::Compiler;

    #[test]
    fn create_compiler() {
        Compiler::new();
    }
}
