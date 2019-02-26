pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        return Compiler {};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_compiler() {
        Compiler::new();
    }
}
