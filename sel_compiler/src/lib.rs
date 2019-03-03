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
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

impl SELTreeNode {
    fn new(op: Operation, data_type: DataType, own_index: usize) -> Self {
        return SELTreeNode {
            operation: op,
            value: Value {
                data_type: data_type,
            },
            // largest operation has two operands
            left: None,
            right: None,
            parent: None,
            own_index: own_index,
        };
    }

    pub fn get_operation(&self) -> Operation {
        return self.operation;
    }

    pub fn get_value(&self) -> Value {
        return self.value;
    }

    pub fn get_left(&self) -> Option<usize> {
        return self.left;
    }

    pub fn get_right(&self) -> Option<usize> {
        return self.right;
    }

    pub fn get_parent(&self) -> Option<usize> {
        return self.parent;
    }

    fn set_left(&mut self, left: usize) {
        self.left = Some(left)
    }

    fn set_right(&mut self, right: usize) {
        self.right = Some(right);
    }

    fn set_parent(&mut self, parent: usize) {
        self.parent = Some(parent);
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

#[derive(PartialEq, Debug, Clone, Copy)]
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

        // loop trough all tokens
        // convert them to tree nodes
        // and link them together
        for token in tokenizer {
            let inserted_index = nodes.len();

            let mut node = SELTreeNode::new(
                get_operation_type_for_token(&token),
                get_data_type_for_token(&token),
                inserted_index,
            );

            node.own_index = inserted_index;

            // because of starter node, there is always a previous node
            if inserted_index > 0 {
                let previous_index = inserted_index - 1;
                match nodes.get_mut(previous_index) {
                    None => (),
                    Some(previous_node) => {
                        node.set_left(previous_index);
                        previous_node.set_right(inserted_index);
                    }
                }
            }

            nodes.push(node);
        }

        // no tokens
        // insert unit node as default
        if nodes.len() == 0 {
            nodes.push(SELTreeNode::new(Operation::None, DataType::Unit, 0));
        }

        return nodes;
    }

    fn find_root_index(nodes: &Vec<SELTreeNode>) -> usize {
        // will always have at least one node
        let mut node = nodes.get(0).unwrap();
        let mut count = 0;

        println!("finding root {:?}", node);

        loop {
            match node.parent {
                None => {
                    break;
                }
                Some(parent) => {
                    node = nodes.get(parent).unwrap();

                    println!("finding root {:?}", node);

                    // fail safe
                    // stop after checking all nodes
                    count += 1;
                    if count > nodes.len() {
                        break;
                    }
                }
            }
        }

        return node.own_index;
    }

    fn resolve_tree(nodes_len: usize, mut nodes: Vec<SELTreeNode>) -> Vec<SELTreeNode> {
        for i in 0..nodes_len {
            let mut changes: Vec<Change> = vec![];
            {
                let nodes = &nodes;
                let node = nodes.get(i).unwrap();

                if node.get_operation() == Operation::Addition
                    || node.get_operation() == Operation::Multiplication
                {
                    match node.get_left() {
                        None => (),
                        Some(left_index) => {
                            let left = nodes.get(left_index).unwrap();
                            // only need to update if a value type
                            if left.value.data_type != DataType::Unknown {
                                // update operands parent to point to operator
                                changes.push(Change {
                                    index_to_change: left.own_index,
                                    new_index: node.own_index,
                                    side_to_set: NodeSide::Parent,
                                });

                                match left.get_left() {
                                    None => (),
                                    Some(left_left_index) => {
                                        let left_left = nodes.get(left_left_index).unwrap();
                                        if left_left.get_operation() != Operation::Addition {
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
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // might not have right
                    match node.get_right() {
                        None => (),
                        Some(right_index) => {
                            let right = nodes.get(right_index).unwrap();
                            // only need to update if a value type
                            if right.value.data_type != DataType::Unknown {
                                changes.push(Change {
                                    index_to_change: right.own_index,
                                    new_index: node.own_index,
                                    side_to_set: NodeSide::Parent,
                                });
                                match right.get_right() {
                                    None => (),
                                    Some(right_right_index) => {
                                        let right_right = nodes.get(right_right_index).unwrap();

                                        if right_right.get_operation() != Operation::Addition {
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

                    println!("performing change {:?}", change);

                    match change.side_to_set {
                        NodeSide::Left => node.set_left(change.new_index),
                        NodeSide::Right => node.set_right(change.new_index),
                        NodeSide::Parent => node.set_parent(change.new_index),
                    }
                }
            }
        }

        return nodes;
    }

    pub fn compile(&self, s: &String) -> SELTree {
        let mut tokenizer = Tokenizer::new(s);
        let nodes = Compiler::make_nodes_from_tokenizer(&mut tokenizer);

        // starting with highest priority operators
        // go through nodes and move pointers to their operands
        // to point at operator
        // let nodes = &mut nodes;

        let nodes = Compiler::resolve_tree(nodes.len(), nodes);

        // next priority
        // for (i, node) in nodes.iter().enumerate() {
        //     if node.get_operation() == Operation::Addition {}
        // }

        let root = Compiler::find_root_index(&nodes);
        println!("{}", root);

        return SELTree {
            root: root,
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

    #[test]
    fn compiles_addition_operation() {
        let input = String::from("5 + 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        println!("{:?}", tree.nodes);

        let root = tree.get_root();

        let left = tree.nodes.get(root.get_left().unwrap()).unwrap();
        let right = tree.nodes.get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Addition);
        assert_eq!(root.get_value().get_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_value().get_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_value().get_type(), DataType::Integer);
    }

    #[test]
    fn compiles_multiplication_operation() {
        let input = String::from("5 * 10");
        let compiler = Compiler::new();

        let tree = compiler.compile(&input);

        let root = tree.get_root();

        let left = tree.nodes.get(root.get_left().unwrap()).unwrap();
        let right = tree.nodes.get(root.get_right().unwrap()).unwrap();

        assert_eq!(root.get_operation(), Operation::Multiplication);
        assert_eq!(root.get_value().get_type(), DataType::Unknown);

        assert_eq!(left.get_operation(), Operation::Touch);
        assert_eq!(left.get_value().get_type(), DataType::Integer);

        assert_eq!(right.get_operation(), Operation::Touch);
        assert_eq!(right.get_value().get_type(), DataType::Integer);
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

        print_nodes(&tree.nodes);

        let root = tree.get_root();

        let left = tree.nodes.get(root.get_left().unwrap()).unwrap();
        let right = tree.nodes.get(root.get_right().unwrap()).unwrap();

        let l2_left = tree.nodes.get(left.get_left().unwrap()).unwrap();
        let l2_right = tree.nodes.get(left.get_right().unwrap()).unwrap();

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

        let root = tree.get_root();

        let left = tree.nodes.get(root.get_left().unwrap()).unwrap();
        let right = tree.nodes.get(root.get_right().unwrap()).unwrap();

        let r2_left = tree.nodes.get(right.get_left().unwrap()).unwrap();
        let r2_right = tree.nodes.get(right.get_right().unwrap()).unwrap();

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

    fn print_nodes(nodes: &Vec<SELTreeNode>) {
        for node in nodes {
            println!("{:?}", node);
        }
    }
}
