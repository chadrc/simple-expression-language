use sel_tokenizer::{Token, TokenType, Tokenizer};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Operation {
    Touch,
    Addition,
    Multiplication,
    None,
    Start,
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

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct SELTreeNode {
    operation: Operation,
    value: Value,
    own_index: usize,
    parent: usize,
    left: usize,
    right: usize,
}

impl SELTreeNode {
    fn new(op: Operation, data_type: DataType) -> Self {
        return SELTreeNode {
            operation: op,
            value: Value {
                data_type: data_type,
            },
            // largest operation has two operands
            left: 0,
            right: 0,
            parent: 0,
            own_index: 0,
        };
    }

    pub fn get_operation(&self) -> Operation {
        return self.operation;
    }

    pub fn get_value(&self) -> Value {
        return self.value;
    }

    pub fn get_left(&self) -> usize {
        return self.left;
    }

    pub fn get_right(&self) -> usize {
        return self.right;
    }

    fn set_left(&mut self, left: usize) {
        self.left = left;
    }

    fn set_right(&mut self, right: usize) {
        self.right = right;
    }

    fn set_parent(&mut self, parent: usize) {
        self.parent = parent;
    }
}

#[derive(Debug, Clone)]
pub struct SELTree {
    root: usize,
    nodes: Vec<SELTreeNode>,
}

impl SELTree {
    pub fn get_root(&self) -> &SELTreeNode {
        return &self.nodes.get(self.root).unwrap();
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum NodeSide {
    Left,
    Right,
    Parent,
}

struct Change {
    index_to_change: usize,
    new_index: usize,
    side_to_set: NodeSide,
}

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        return Compiler {};
    }

    fn make_nodes_from_tokenizer(tokenizer: &mut Tokenizer) -> Vec<SELTreeNode> {
        let mut nodes: Vec<SELTreeNode> = vec![];

        // insert starter node to avoid later checks for length and end of list
        nodes.push(SELTreeNode::new(Operation::Start, DataType::Unknown));

        // loop trough all tokens
        // convert them to tree nodes
        // and link them together
        for token in tokenizer {
            let mut node = SELTreeNode::new(
                get_operation_type_for_token(&token),
                get_data_type_for_token(&token),
            );

            let inserted_index = nodes.len();
            node.own_index = inserted_index;

            // because of starter node, there is always a previous node
            let previous_index = inserted_index - 1;
            let mut previous_node = nodes.get_mut(previous_index).unwrap();

            node.left = previous_index;
            previous_node.right = inserted_index;

            nodes.push(node);
        }

        // no tokens
        // insert unit node as default
        if nodes.len() == 1 {
            let mut default_node = SELTreeNode::new(Operation::None, DataType::Unit);
            default_node.own_index = 1;
            nodes.push(default_node);
        }

        return nodes;
    }

    fn find_root_index(nodes: &Vec<SELTreeNode>) -> usize {
        // first node is a placeholer
        // so start with second node
        let mut node = nodes.get(1).unwrap();
        let mut count = 0;

        while node.parent != 0 {
            node = nodes.get(node.parent).unwrap();

            // fail safe
            // stop after checking all nodes
            count += 1;
            if count > nodes.len() {
                break;
            }
        }

        return node.own_index;
    }

    pub fn compile(&self, s: &String) -> SELTree {
        let mut tokenizer = Tokenizer::new(s);
        let mut nodes = Compiler::make_nodes_from_tokenizer(&mut tokenizer);

        // starting with highest priority operators
        // go through nodes and move pointers to their operands
        // to point at operator
        // let nodes = &mut nodes;

        let mut changes: Vec<Change> = vec![];

        {
            let nodes = &nodes;

            for node in nodes.iter() {
                if node.get_operation() == Operation::Multiplication {
                    // will always have left, because of start node
                    let left_operand = nodes.get(node.get_left()).unwrap();

                    // although left might not always have a left
                    match nodes.get(left_operand.get_left()) {
                        None => (),
                        Some(left_left) => {
                            if left_left.get_operation() == Operation::Addition {
                                // if lower priority
                                // make its right point to node
                                changes.push(Change {
                                    index_to_change: left_left.own_index,
                                    new_index: node.own_index,
                                    side_to_set: NodeSide::Right,
                                });

                                changes.push(Change {
                                    index_to_change: node.own_index,
                                    new_index: left_left.own_index,
                                    side_to_set: NodeSide::Parent,
                                });
                            } else {
                                // same or higher priority
                                // make node's left point to it
                                changes.push(Change {
                                    index_to_change: node.own_index,
                                    new_index: left_left.own_index,
                                    side_to_set: NodeSide::Left,
                                });

                                changes.push(Change {
                                    index_to_change: left_left.own_index,
                                    new_index: node.own_index,
                                    side_to_set: NodeSide::Parent,
                                });
                            }
                        }
                    }

                    // might not have right
                    match nodes.get(node.get_right()) {
                        None => (),
                        Some(right) => {
                            // still might not have a node to right of right
                            match nodes.get(right.get_right()) {
                                None => (),
                                Some(right_right) => {
                                    if right_right.get_operation() == Operation::Addition {
                                        // lower priority
                                        // make its left point to node
                                        changes.push(Change {
                                            index_to_change: right_right.own_index,
                                            new_index: node.own_index,
                                            side_to_set: NodeSide::Left,
                                        });

                                        changes.push(Change {
                                            index_to_change: node.own_index,
                                            new_index: right_right.own_index,
                                            side_to_set: NodeSide::Parent,
                                        });
                                    } else {
                                        changes.push(Change {
                                            index_to_change: node.own_index,
                                            new_index: right_right.own_index,
                                            side_to_set: NodeSide::Right,
                                        });

                                        changes.push(Change {
                                            index_to_change: right_right.own_index,
                                            new_index: node.own_index,
                                            side_to_set: NodeSide::Parent,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        {
            let nodes = &mut nodes;

            for change in changes {
                let node = nodes.get_mut(change.index_to_change).unwrap();

                match change.side_to_set {
                    NodeSide::Left => node.set_left(change.new_index),
                    NodeSide::Right => node.set_right(change.new_index),
                    NodeSide::Parent => node.set_parent(change.new_index),
                }
            }
        }

        // next priority
        // for (i, node) in nodes.iter().enumerate() {
        //     if node.get_operation() == Operation::Addition {}
        // }

        println!("{:?}", nodes);

        return SELTree {
            root: Compiler::find_root_index(&nodes),
            nodes: nodes,
        };
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
    } else if token.get_token_type() == TokenType::Boolean
        || token.get_token_type() == TokenType::Integer
        || token.get_token_type() == TokenType::Decimal
        || token.get_token_type() == TokenType::SingleQuotedString
        || token.get_token_type() == TokenType::DoubleQuotedString
        || token.get_token_type() == TokenType::FormattedString
    {
        // all value tokens result in a touch operation
        Operation::Touch
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

    // #[test]
    // fn compiles_addition_operation() {
    //     let input = String::from("5 + 10");
    //     let compiler = Compiler::new();

    //     let tree = compiler.compile(&input);

    //     let root = tree.get_root();

    //     let left = root.get_left();
    //     let right = root.get_right();

    //     assert_eq!(root.get_operation(), Operation::Addition);
    //     assert_eq!(root.get_value().get_type(), DataType::Unknown);

    //     assert_eq!(left.unwrap().get_operation(), Operation::Touch);
    //     assert_eq!(left.unwrap().get_value().get_type(), DataType::Integer);

    //     assert_eq!(right.unwrap().get_operation(), Operation::Touch);
    //     assert_eq!(right.unwrap().get_value().get_type(), DataType::Integer);
    // }

    // #[test]
    // fn compiles_multiplication_operation() {
    //     let input = String::from("5 * 10");
    //     let compiler = Compiler::new();

    //     let tree = compiler.compile(&input);

    //     let root = tree.get_root();

    //     let left = root.get_left();
    //     let right = root.get_right();

    //     assert_eq!(root.get_operation(), Operation::Multiplication);
    //     assert_eq!(root.get_value().get_type(), DataType::Unknown);

    //     assert_eq!(left.unwrap().get_operation(), Operation::Touch);
    //     assert_eq!(left.unwrap().get_value().get_type(), DataType::Integer);

    //     assert_eq!(right.unwrap().get_operation(), Operation::Touch);
    //     assert_eq!(right.unwrap().get_value().get_type(), DataType::Integer);
    // }

    // #[test]
    // fn compiles_two_addition_operations() {
    //     let input = String::from("5 + 10 + 15");
    //     let compiler = Compiler::new();

    //     let tree = compiler.compile(&input);

    //     // tree should look like
    //     //          +
    //     //         / \
    //     //        +   15
    //     //       / \
    //     //      5   10

    //     let root = tree.get_root();

    //     let left = root.get_left().unwrap();
    //     let right = root.get_right().unwrap();

    //     let l2_left = left.get_left().unwrap();
    //     let l2_right = left.get_right().unwrap();

    //     assert_eq!(root.get_operation(), Operation::Addition);
    //     assert_eq!(root.get_value().get_type(), DataType::Unknown);

    //     assert_eq!(left.get_operation(), Operation::Addition);
    //     assert_eq!(left.get_value().get_type(), DataType::Unknown);

    //     assert_eq!(right.get_operation(), Operation::Touch);
    //     assert_eq!(right.get_value().get_type(), DataType::Integer);

    //     assert_eq!(l2_left.get_operation(), Operation::Touch);
    //     assert_eq!(l2_left.get_value().get_type(), DataType::Integer);

    //     assert_eq!(l2_right.get_operation(), Operation::Touch);
    //     assert_eq!(l2_right.get_value().get_type(), DataType::Integer);
    // }

    // #[test]
    // fn compiles_addition_multiplication_operations() {
    //     let input = String::from("5 + 10 * 15");
    //     let compiler = Compiler::new();

    //     let tree = compiler.compile(&input);

    //     // tree should look like
    //     //          +
    //     //         / \
    //     //        5   *
    //     //           / \
    //     //         10   15

    //     let root = tree.get_root();

    //     let left = root.get_left().unwrap();
    //     let right = root.get_right().unwrap();

    //     let r2_left = right.get_left().unwrap();
    //     let r2_right = right.get_right().unwrap();

    //     assert_eq!(root.get_operation(), Operation::Addition);
    //     assert_eq!(root.get_value().get_type(), DataType::Unknown);

    //     assert_eq!(left.get_operation(), Operation::Touch);
    //     assert_eq!(left.get_value().get_type(), DataType::Integer);

    //     assert_eq!(right.get_operation(), Operation::Multiplication);
    //     assert_eq!(right.get_value().get_type(), DataType::Unknown);

    //     assert_eq!(r2_left.get_operation(), Operation::Touch);
    //     assert_eq!(r2_left.get_value().get_type(), DataType::Integer);

    //     assert_eq!(r2_right.get_operation(), Operation::Touch);
    //     assert_eq!(r2_right.get_value().get_type(), DataType::Integer);
    // }
    // #[test]
    // fn compiles_multiplication_addition_operations() {
    //     let input = String::from("5 * 10 + 15");
    //     let compiler = Compiler::new();

    //     let tree = compiler.compile(&input);

    //     // tree should look like
    //     //          +
    //     //         / \
    //     //        *   15
    //     //       / \
    //     //      5   10

    //     let root = tree.get_root();

    //     let left = root.get_left().unwrap();
    //     let right = root.get_right().unwrap();

    //     let l2_left = left.get_left().unwrap();
    //     let l2_right = left.get_right().unwrap();

    //     assert_eq!(root.get_operation(), Operation::Addition);
    //     assert_eq!(root.get_value().get_type(), DataType::Unknown);

    //     assert_eq!(left.get_operation(), Operation::Multiplication);
    //     assert_eq!(left.get_value().get_type(), DataType::Unknown);

    //     assert_eq!(right.get_operation(), Operation::Touch);
    //     assert_eq!(right.get_value().get_type(), DataType::Integer);

    //     assert_eq!(l2_left.get_operation(), Operation::Touch);
    //     assert_eq!(l2_left.get_value().get_type(), DataType::Integer);

    //     assert_eq!(l2_right.get_operation(), Operation::Touch);
    //     assert_eq!(l2_right.get_value().get_type(), DataType::Integer);
    // }
}
