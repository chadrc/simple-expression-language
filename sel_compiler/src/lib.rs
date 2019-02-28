use sel_tokenizer::{Token, TokenType, Tokenizer};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Operation {
    Touch,
    Addition,
    Multiplication,
    None,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum DataType {
    Unknown,
    Unit,
    Integer,
    Decimal,
    String,
    Boolean,
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

#[derive(Debug, Clone)]
pub struct SELTreeNode<'a> {
    operation: Operation,
    value: Value,
    left: usize,
    right: usize,
    tree: &'a SELTree<'a>,
}

impl<'a> SELTreeNode<'a> {
    fn new(op: Operation, data_type: DataType, tree: &'a SELTree<'a>) -> Self {
        return SELTreeNode {
            operation: op,
            value: Value {
                data_type: data_type,
            },
            // largest operation has two operands
            left: 0,
            right: 0,
            tree: tree,
        };
    }

    pub fn get_operation(&self) -> Operation {
        return self.operation;
    }

    pub fn get_value(&self) -> Value {
        return self.value;
    }

    pub fn get_left(&self) -> Option<&'a SELTreeNode> {
        return self.tree.nodes.get(self.left);
    }

    pub fn get_right(&self) -> Option<&'a SELTreeNode> {
        return self.tree.nodes.get(self.right);
    }
}

#[derive(Debug, Clone)]
pub struct SELTree<'a> {
    root: Option<SELTreeNode<'a>>,
    nodes: Vec<SELTreeNode<'a>>,
}

impl<'a> SELTree<'a> {
    fn new(tokens: Vec<Token>) -> Self {
        let tree = SELTree {
            root: None,
            nodes: vec![],
        };

        let mut nodes: Vec<SELTreeNode> = vec![];

        // loop trough all tokens
        // convert them to tree nodes
        // and link them together

        for token in tokens {
            let mut node = SELTreeNode::new(
                get_operation_type_for_token(&token),
                get_data_type_for_token(&token),
                &tree,
            );

            let inserted_index = nodes.len() - 1;

            // if we have at least one previous node
            if inserted_index > 0 {
                let previous_index = inserted_index - 1;
                let mut previous_node = nodes.get_mut(previous_index).unwrap();

                node.left = previous_index;
                previous_node.right = inserted_index;
            }

            nodes.push(node);
        }

        return tree;
    }

    pub fn get_root(&self) -> Option<&SELTreeNode> {
        return match &self.root {
            None => None,
            Some(r) => Some(&r),
        };
    }
}

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        return Compiler {};
    }

    pub fn compile(&self, s: &String) -> SELTree {
        let tokens: Vec<Token> = Tokenizer::new(s).collect();
        return SELTree::new(tokens);
    }
}

fn get_data_type_for_token(token: &Token) -> DataType {
    let token_type = token.get_token_type();

    return if token_type == TokenType::Integer {
        DataType::Integer
    } else if token_type == TokenType::Decimal {
        DataType::Decimal
    } else if token_type == TokenType::SingleQuotedString
        || token_type == TokenType::DoubleQuotedString
        || token_type == TokenType::FormattedString
    {
        DataType::String
    } else if token_type == TokenType::Boolean {
        DataType::Boolean
    } else {
        DataType::Unknown
    };
}

fn get_operation_type_for_token(token: &Token) -> Operation {
    return if token.get_token_type() == TokenType::PlusSign {
        Operation::Addition
    } else if token.get_token_type() == TokenType::MultiplicationSign {
        Operation::Multiplication
    } else {
        Operation::None
    };
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

        let root = tree.get_root().unwrap();

        assert_eq!(root.get_operation(), Operation::None);
        assert_eq!(root.get_value().get_type(), DataType::Unit);
    }

    #[test]
    fn compiles_touch_integer() {
        let input = String::from("9");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root().unwrap();

        assert_eq!(root.get_operation(), Operation::Touch);
        assert_eq!(root.get_value().get_type(), DataType::Integer);
    }

    #[test]
    fn compiles_touch_decimal() {
        let input = String::from("3.14");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root().unwrap();

        assert_eq!(root.get_operation(), Operation::Touch);
        assert_eq!(root.get_value().get_type(), DataType::Decimal);
    }

    #[test]
    fn compiles_touch_single_quote_string() {
        let input = String::from("'hello world'");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root().unwrap();

        assert_eq!(root.get_operation(), Operation::Touch);
        assert_eq!(root.get_value().get_type(), DataType::String);
    }

    #[test]
    fn compiles_touch_double_quote_string() {
        let input = String::from("\"hello world\"");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root().unwrap();

        assert_eq!(root.get_operation(), Operation::Touch);
        assert_eq!(root.get_value().get_type(), DataType::String);
    }

    #[test]
    fn compiles_touch_formatted_string() {
        let input = String::from("`hello world`");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root().unwrap();

        assert_eq!(root.get_operation(), Operation::Touch);
        assert_eq!(root.get_value().get_type(), DataType::String);
    }

    #[test]
    fn compiles_touch_boolean() {
        let input = String::from("true");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root().unwrap();

        assert_eq!(root.get_operation(), Operation::Touch);
        assert_eq!(root.get_value().get_type(), DataType::Boolean);
    }

    #[test]
    fn compiles_addition_operation() {
        let input = String::from("5 + 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root().unwrap();

        let left = root.get_left();
        let right = root.get_right();

        assert_eq!(root.get_operation(), Operation::Addition);
        assert_eq!(root.get_value().get_type(), DataType::Unknown);

        assert_eq!(left.unwrap().get_operation(), Operation::Touch);
        assert_eq!(left.unwrap().get_value().get_type(), DataType::Integer);

        assert_eq!(right.unwrap().get_operation(), Operation::Touch);
        assert_eq!(right.unwrap().get_value().get_type(), DataType::Integer);
    }

    #[test]
    fn compiles_multiplication_operation() {
        let input = String::from("5 * 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root().unwrap();

        let left = root.get_left();
        let right = root.get_right();

        assert_eq!(root.get_operation(), Operation::Multiplication);
        assert_eq!(root.get_value().get_type(), DataType::Unknown);

        assert_eq!(left.unwrap().get_operation(), Operation::Touch);
        assert_eq!(left.unwrap().get_value().get_type(), DataType::Integer);

        assert_eq!(right.unwrap().get_operation(), Operation::Touch);
        assert_eq!(right.unwrap().get_value().get_type(), DataType::Integer);
    }

    #[test]
    fn compiles_two_addition_operations() {
        let input = String::from("5 + 10 + 15");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        // tree should look like
        //          +
        //         / \
        //        +   15
        //       / \
        //      5   10

        let root = tree.get_root().unwrap();

        let left = root.get_left().unwrap();
        let right = root.get_right().unwrap();

        let l2_left = left.get_left().unwrap();
        let l2_right = left.get_right().unwrap();

        assert_eq!(root.get_operation(), Operation::Addition);
        assert_eq!(root.get_value().get_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Addition);
        assert_eq!(left.get_value().get_type(), DataType::Unknown);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_value().get_type(), DataType::Integer);

        assert_eq!(l2_left.get_operation(), Operation::Touch);
        assert_eq!(l2_left.get_value().get_type(), DataType::Integer);

        assert_eq!(l2_right.get_operation(), Operation::Touch);
        assert_eq!(l2_right.get_value().get_type(), DataType::Integer);
    }

    #[test]
    fn compiles_addition_multiplication_operations() {
        let input = String::from("5 + 10 * 15");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        // tree should look like
        //          +
        //         / \
        //        5   *
        //           / \
        //         10   15

        let root = tree.get_root().unwrap();

        let left = root.get_left().unwrap();
        let right = root.get_right().unwrap();

        let r2_left = right.get_left().unwrap();
        let r2_right = right.get_right().unwrap();

        assert_eq!(root.get_operation(), Operation::Addition);
        assert_eq!(root.get_value().get_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_value().get_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Multiplication);
        assert_eq!(right.get_value().get_type(), DataType::Unknown);

        assert_eq!(r2_left.get_operation(), Operation::Touch);
        assert_eq!(r2_left.get_value().get_type(), DataType::Integer);

        assert_eq!(r2_right.get_operation(), Operation::Touch);
        assert_eq!(r2_right.get_value().get_type(), DataType::Integer);
    }
    #[test]
    fn compiles_multiplication_addition_operations() {
        let input = String::from("5 * 10 + 15");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        // tree should look like
        //          +
        //         / \
        //        *   15
        //       / \
        //      5   10

        let root = tree.get_root().unwrap();

        let left = root.get_left().unwrap();
        let right = root.get_right().unwrap();

        let l2_left = left.get_left().unwrap();
        let l2_right = left.get_right().unwrap();

        assert_eq!(root.get_operation(), Operation::Addition);
        assert_eq!(root.get_value().get_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Multiplication);
        assert_eq!(left.get_value().get_type(), DataType::Unknown);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_value().get_type(), DataType::Integer);

        assert_eq!(l2_left.get_operation(), Operation::Touch);
        assert_eq!(l2_left.get_value().get_type(), DataType::Integer);

        assert_eq!(l2_right.get_operation(), Operation::Touch);
        assert_eq!(l2_right.get_value().get_type(), DataType::Integer);
    }
}
