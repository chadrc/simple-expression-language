use sel_tokenizer::{TokenType, Tokenizer};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Operation {
    Touch,
    None,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum DataType {
    Integer,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Value {
    data_type: DataType,
}

impl Value {
    pub fn get_type(&self) -> DataType {
        return self.data_type;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SELTreeNode {
    operation: Operation,
    value: Value,
}

impl SELTreeNode {
    pub fn get_operation(&self) -> Operation {
        return self.operation;
    }

    pub fn get_value(&self) -> Value {
        return self.value;
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

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        return Compiler {};
    }

    pub fn compile(&self, s: &String) -> SELTree {
        let tokenizer = Tokenizer::new(s);

        // assume some defaults
        let mut op = Operation::None;
        let mut data_type = DataType::Integer;

        for token in tokenizer {
            if token.get_token_type() == TokenType::Integer {
                op = Operation::Touch;
                data_type = DataType::Integer;
            }
        }

        return SELTree {
            root: SELTreeNode {
                operation: op,
                value: Value {
                    data_type: data_type,
                },
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

    #[test]
    fn compiles_touch_integer() {
        let input = String::from("9");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        assert_eq!(root.get_operation(), Operation::Touch);
        assert_eq!(root.get_value().get_type(), DataType::Integer);
    }
}
