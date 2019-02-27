use sel_tokenizer::{Token, TokenType, Tokenizer};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Operation {
    Touch,
    Addition,
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
pub struct SELTreeNode {
    operation: Operation,
    value: Value,
    left: Box<Option<SELTreeNode>>,
    right: Box<Option<SELTreeNode>>,
}

const LEFT_OP_INDEX: usize = 0;
const RIGHT_OP_INDEX: usize = 1;

impl SELTreeNode {
    fn new(op: Operation, data_type: DataType) -> Self {
        return SELTreeNode {
            operation: op,
            value: Value {
                data_type: data_type,
            },
            // largest operation has two operands
            left: Box::new(None),
            right: Box::new(None),
        };
    }

    pub fn get_operation(&self) -> Operation {
        return self.operation;
    }

    pub fn get_value(&self) -> Value {
        return self.value;
    }

    pub fn get_left(&self) -> Option<&SELTreeNode> {
        return match &*self.left {
            None => None,
            Some(n) => Some(&n),
        };
    }

    pub fn get_right(&self) -> Option<&SELTreeNode> {
        return match &*self.right {
            None => None,
            Some(n) => Some(&n),
        };
    }
}

#[derive(Debug, Clone)]
pub struct SELTree {
    root: SELTreeNode,
}

impl SELTree {
    pub fn get_root(&self) -> &SELTreeNode {
        return &self.root;
    }
}

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        return Compiler {};
    }

    pub fn compile(&self, s: &String) -> SELTree {
        let mut tokenizer = Tokenizer::new(s);

        let mut last_node: Option<&SELTreeNode> = None;
        let mut last_op_node: Option<SELTreeNode> = None;

        // process first token
        let token = match tokenizer.next() {
            None => {
                return SELTree {
                    root: SELTreeNode::new(Operation::None, DataType::Unit),
                };
            }
            Some(t) => t,
        };

        // first token should be a value token
        let op = Operation::Touch;
        let data_type = get_value_type_for_token(token);

        let first_node = SELTreeNode::new(op, data_type);

        // second token should be a operation token
        let token = match tokenizer.next() {
            None => {
                // no other operations
                // return the touch operation
                return SELTree { root: first_node };
            }
            Some(t) => t,
        };

        let op = get_operation_type_for_token(token);
        let data_type = DataType::Unknown;

        let mut op_node = SELTreeNode::new(op, data_type);

        // place first token as left operand

        op_node.left = Box::new(Some(first_node));

        // get next token and place it as right operand of op_node
        let token = match tokenizer.next() {
            None => {
                // no second operand
                // return incomplete operation
                return SELTree { root: op_node };
            }
            Some(t) => t,
        };

        // first token should be a value token
        let op = Operation::Touch;
        let data_type = get_value_type_for_token(token);

        let second_node = SELTreeNode::new(op, data_type);

        op_node.right = Box::new(Some(second_node));

        return SELTree { root: op_node };
    }
}

fn get_value_type_for_token(token: Token) -> DataType {
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

fn get_operation_type_for_token(token: Token) -> Operation {
    return if token.get_token_type() == TokenType::PlusSign {
        Operation::Addition
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

        let root = tree.get_root();

        assert_eq!(root.get_operation(), Operation::None);
        assert_eq!(root.get_value().get_type(), DataType::Unit);
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

    #[test]
    fn compiles_touch_decimal() {
        let input = String::from("3.14");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        assert_eq!(root.get_operation(), Operation::Touch);
        assert_eq!(root.get_value().get_type(), DataType::Decimal);
    }

    #[test]
    fn compiles_touch_single_quote_string() {
        let input = String::from("'hello world'");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        assert_eq!(root.get_operation(), Operation::Touch);
        assert_eq!(root.get_value().get_type(), DataType::String);
    }

    #[test]
    fn compiles_touch_double_quote_string() {
        let input = String::from("\"hello world\"");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        assert_eq!(root.get_operation(), Operation::Touch);
        assert_eq!(root.get_value().get_type(), DataType::String);
    }

    #[test]
    fn compiles_touch_formatted_string() {
        let input = String::from("`hello world`");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        assert_eq!(root.get_operation(), Operation::Touch);
        assert_eq!(root.get_value().get_type(), DataType::String);
    }

    #[test]
    fn compiles_touch_boolean() {
        let input = String::from("true");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        assert_eq!(root.get_operation(), Operation::Touch);
        assert_eq!(root.get_value().get_type(), DataType::Boolean);
    }

    #[test]
    fn compiles_addition_operation() {
        let input = String::from("5 + 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

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

        let root = tree.get_root();

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
}
