pub struct Compiler {}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Operation {
    Touch,
    None,
}

#[derive(Debug, Clone, Copy)]
pub struct SELTreeNode {
    operation: Operation,
}

impl SELTreeNode {
    pub fn get_operation(&self) -> Operation {
        return self.operation;
    }
}

#[derive(Debug, Clone)]
pub struct SELTree {
    root: SELTreeNode,
}

impl SELTree {
    pub fn get_root(&self) -> SELTreeNode {
        return self.root;
    }
}

impl Compiler {
    pub fn new() -> Self {
        return Compiler {};
    }

    pub fn compile(&self, _s: &String) -> SELTree {
        return SELTree {
            root: SELTreeNode {
                operation: Operation::None,
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_compiler() {
        Compiler::new();
    }

    #[test]
    fn compiles_empty() {
        let input = String::from("");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        assert_eq!(root.get_operation(), Operation::None);
    }
}
