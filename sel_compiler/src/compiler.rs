use super::data_type::{get_data_type_for_token, DataType};
use super::operation::{get_operation_type_for_token, Operation};
use super::sel_tree::{opposite_of_side, NodeSide, SELTree, SELTreeNode};
use sel_tokenizer::Tokenizer;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Copy)]
struct Change {
    index_to_change: usize,
    new_index: Option<usize>,
    side_to_set: NodeSide,
}

fn none_left_right(index: usize) -> Vec<Change> {
    let mut changes: Vec<Change> = vec![];

    changes.push(Change {
        index_to_change: index,
        new_index: None,
        side_to_set: NodeSide::Left,
    });

    changes.push(Change {
        index_to_change: index,
        new_index: None,
        side_to_set: NodeSide::Right,
    });

    return changes;
}

pub struct Compiler {
    operation_priorities: HashMap<Operation, usize>,
}

// lower number means higher priority
const VALUE_PRECEDENCE: usize = 0;
const NOT_PRECEDENCE: usize = VALUE_PRECEDENCE + 1;
const RANGE_PRECEDENCE: usize = NOT_PRECEDENCE + 1;
const EXPONENTIAL_PRECEDENCE: usize = RANGE_PRECEDENCE + 1;
const MULTIPLICATION_PRECEDENCE: usize = EXPONENTIAL_PRECEDENCE + 1;
const ADDITION_PRECEDENCE: usize = MULTIPLICATION_PRECEDENCE + 1;
const RELATIONAL_PRECEDENCE: usize = ADDITION_PRECEDENCE + 1;
const EQUALITY_PRECEDENCE: usize = RELATIONAL_PRECEDENCE + 1;
const AND_PRECEDENCE: usize = EQUALITY_PRECEDENCE + 1;
const OR_PRECEDENCE: usize = AND_PRECEDENCE + 1;

impl Compiler {
    pub fn new() -> Self {
        let mut operation_priorities = HashMap::new();

        operation_priorities.insert(Operation::Touch, VALUE_PRECEDENCE);

        operation_priorities.insert(Operation::ExclusiveRange, RANGE_PRECEDENCE);
        operation_priorities.insert(Operation::InclusiveRange, RANGE_PRECEDENCE);

        operation_priorities.insert(Operation::Exponential, EXPONENTIAL_PRECEDENCE);

        operation_priorities.insert(Operation::Multiplication, MULTIPLICATION_PRECEDENCE);
        operation_priorities.insert(Operation::Division, MULTIPLICATION_PRECEDENCE);
        operation_priorities.insert(Operation::Modulo, MULTIPLICATION_PRECEDENCE);

        operation_priorities.insert(Operation::Addition, ADDITION_PRECEDENCE);
        operation_priorities.insert(Operation::Subtraction, ADDITION_PRECEDENCE);

        operation_priorities.insert(Operation::GreaterThan, RELATIONAL_PRECEDENCE);
        operation_priorities.insert(Operation::GreaterThanOrEqual, RELATIONAL_PRECEDENCE);
        operation_priorities.insert(Operation::LessThan, RELATIONAL_PRECEDENCE);
        operation_priorities.insert(Operation::LessThanOrEqual, RELATIONAL_PRECEDENCE);

        operation_priorities.insert(Operation::Equality, EQUALITY_PRECEDENCE);
        operation_priorities.insert(Operation::Inequality, EQUALITY_PRECEDENCE);

        operation_priorities.insert(Operation::LogicalAnd, AND_PRECEDENCE);

        operation_priorities.insert(Operation::LogicalOr, OR_PRECEDENCE);

        operation_priorities.insert(Operation::LogicalNot, NOT_PRECEDENCE);

        return Compiler {
            operation_priorities: operation_priorities,
        };
    }

    fn make_nodes_from_tokenizer(
        &self,
        tokenizer: &mut Tokenizer,
    ) -> (Vec<SELTreeNode>, Vec<Vec<usize>>) {
        let mut priority_map: Vec<Vec<usize>> = vec![];
        priority_map.push(vec![]); // VALUE_PRECEDENCE
        priority_map.push(vec![]); // NOT_PRECEDENCE
        priority_map.push(vec![]); // RANGE_PRECEDENCE
        priority_map.push(vec![]); // EXPONENTIAL_PRECEDENCE
        priority_map.push(vec![]); // MULTIPLICATION_PRECEDENCE
        priority_map.push(vec![]); // ADDITION_PRECEDENCE
        priority_map.push(vec![]); // RELATIONAL_PRECEDENCE
        priority_map.push(vec![]); // EQUALITY_PRECEDENCE
        priority_map.push(vec![]); // AND_PRECEDENCE
        priority_map.push(vec![]); // OR_PRECEDENCE

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

            // because of starter node, there is always a previous node
            if inserted_index > 0 {
                let previous_index = inserted_index - 1;
                match nodes.get_mut(previous_index) {
                    None => (),
                    Some(previous_node) => {
                        node.set_left(Some(previous_index));
                        previous_node.set_right(Some(inserted_index));
                    }
                }
            }

            nodes.push(node);

            match self.operation_priorities.get(&node.get_operation()) {
                None => (),
                Some(node_priority) => {
                    let v = priority_map.get_mut(*node_priority).unwrap();
                    v.push(node.get_own_index());
                }
            }
        }

        // no tokens
        // insert unit node as default
        if nodes.len() == 0 {
            nodes.push(SELTreeNode::new(Operation::None, DataType::Unit, 0));
        }

        return (nodes, priority_map);
    }

    fn find_root_index(nodes: &Vec<SELTreeNode>) -> usize {
        // will always have at least one node
        let mut node = nodes.get(0).unwrap();
        let mut count = 0;

        // println!("finding root {:?}", node);

        loop {
            match node.get_parent() {
                None => {
                    break;
                }
                Some(parent) => {
                    node = nodes.get(parent).unwrap();

                    // println!("finding root {:?}", node);

                    // fail safe
                    // stop after checking all nodes
                    count += 1;
                    if count > nodes.len() {
                        break;
                    }
                }
            }
        }

        return node.get_own_index();
    }

    fn resolve_tree(
        &self,
        mut nodes: Vec<SELTreeNode>,
        indicies_to_resolve: &Vec<usize>,
    ) -> Vec<SELTreeNode> {
        // println!("{:?}", indicies_to_resolve);

        for i in indicies_to_resolve {
            let mut changes: Vec<Change> = vec![];
            {
                let nodes = &nodes;
                let node = nodes.get(*i).unwrap();

                let parent_of_index = |start_index: usize| -> &SELTreeNode {
                    let mut node = nodes.get(start_index).unwrap();

                    // walk up tree until no parent
                    loop {
                        match node.get_parent() {
                            None => {
                                break;
                            }
                            Some(parent_index) => {
                                node = nodes.get(parent_index).unwrap();
                            }
                        }
                    }

                    return node;
                };

                let change_to_node_if_value =
                    |priority: usize, index: Option<usize>, side: NodeSide| -> Option<Change> {
                        if priority == VALUE_PRECEDENCE {
                            // left node will no longer be pointing toward its left
                            // need to update left's right to point to node
                            return match index {
                                None => None,
                                Some(new_index) => {
                                    // set right to be update by later iteration
                                    return Some(Change {
                                        index_to_change: new_index,
                                        new_index: Some(node.get_own_index()),
                                        side_to_set: side,
                                    });
                                }
                            };
                        }

                        return None;
                    };

                let index_for_side = |side: NodeSide| match side {
                    NodeSide::Parent => panic!("NodeSide::Parent can't be updated."),
                    NodeSide::Right => node.get_right(),
                    NodeSide::Left => node.get_left(),
                };

                let mut update_node_side = |side: NodeSide| {
                    let start_index = index_for_side(side);

                    match start_index {
                        None => (), // shouldn't happen
                        Some(node_index) => {
                            let next_node = parent_of_index(node_index);

                            changes.push(Change {
                                index_to_change: node.get_own_index(),
                                new_index: Some(next_node.get_own_index()),
                                side_to_set: side,
                            });

                            let priority = self
                                .operation_priorities
                                .get(&next_node.get_operation())
                                .unwrap();

                            if *priority == VALUE_PRECEDENCE {
                                changes.append(&mut none_left_right(next_node.get_own_index()));
                            }

                            changes.push(Change {
                                index_to_change: next_node.get_own_index(),
                                new_index: Some(node.get_own_index()),
                                side_to_set: NodeSide::Parent,
                            });

                            let opposite_side = opposite_of_side(side);

                            match change_to_node_if_value(
                                *priority,
                                index_for_side(opposite_side),
                                opposite_side,
                            ) {
                                None => (),
                                Some(change) => changes.push(change),
                            }
                        }
                    }
                };

                update_node_side(NodeSide::Left);
                update_node_side(NodeSide::Right);
            }

            {
                let nodes = &mut nodes;

                for change in changes {
                    let node = nodes.get_mut(change.index_to_change).unwrap();

                    // println!("performing change {:?}", change);

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
        let (mut nodes, priority_map) = self.make_nodes_from_tokenizer(&mut tokenizer);

        // skip VALUE_PRECEDENCE
        for (_i, priority) in priority_map.iter().skip(1).enumerate() {
            if priority.len() > 0 {
                // println!("priority {:?}", i);
                nodes = self.resolve_tree(nodes, &priority);
            }
        }

        let root = Compiler::find_root_index(&nodes);
        // println!("{}", root);

        return SELTree::new(root, nodes);
    }
}
