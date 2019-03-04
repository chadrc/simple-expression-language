use super::data_type::{get_data_type_for_token, DataType};
use super::operation::{get_operation_type_for_token, Operation};
use super::sel_tree::{NodeSide, SELTree, SELTreeNode};
use sel_tokenizer::Tokenizer;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Copy)]
struct Change {
    index_to_change: usize,
    new_index: Option<usize>,
    side_to_set: NodeSide,
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

                let node_priority = self
                    .operation_priorities
                    .get(&node.get_operation())
                    .unwrap();

                match node.get_left() {
                    None => (),
                    Some(left_index) => {
                        let mut left_node = nodes.get(left_index).unwrap();

                        // walk up tree until no parent
                        loop {
                            match left_node.get_parent() {
                                None => {
                                    break;
                                }
                                Some(parent_index) => {
                                    left_node = nodes.get(parent_index).unwrap();
                                }
                            }
                        }

                        changes.push(Change {
                            index_to_change: node.get_own_index(),
                            new_index: Some(left_node.get_own_index()),
                            side_to_set: NodeSide::Left,
                        });

                        let left_priority = self
                            .operation_priorities
                            .get(&left_node.get_operation())
                            .unwrap();

                        if left_priority < node_priority {
                            // left has higher priority

                            // None left node's left and right
                            // if value precedence
                            if *left_priority == VALUE_PRECEDENCE {
                                changes.push(Change {
                                    index_to_change: left_node.get_own_index(),
                                    new_index: None,
                                    side_to_set: NodeSide::Left,
                                });

                                changes.push(Change {
                                    index_to_change: left_node.get_own_index(),
                                    new_index: None,
                                    side_to_set: NodeSide::Right,
                                });
                            }

                            // Set left node's parent to node
                            changes.push(Change {
                                index_to_change: left_node.get_own_index(),
                                new_index: Some(node.get_own_index()),
                                side_to_set: NodeSide::Parent,
                            });
                        } else if left_priority == node_priority {
                            // same priority

                            // make node, left's parent
                            changes.push(Change {
                                index_to_change: left_node.get_own_index(),
                                new_index: Some(node.get_own_index()),
                                side_to_set: NodeSide::Parent,
                            });
                        } else {
                            // left has lower priority
                        }

                        // check and update next node only if this one was a value node
                        if *left_priority == VALUE_PRECEDENCE {
                            // left node will no longer be pointing toward its left
                            // need to update left's right to point to node
                            match left_node.get_left() {
                                None => (),
                                Some(left_left_index) => {
                                    let left_left_node = nodes.get(left_left_index).unwrap();

                                    // set right to be update by later iteration
                                    changes.push(Change {
                                        index_to_change: left_left_node.get_own_index(),
                                        new_index: Some(node.get_own_index()),
                                        side_to_set: NodeSide::Right,
                                    });
                                }
                            }
                        }
                    }
                }

                match node.get_right() {
                    None => (),
                    Some(right_index) => {
                        let mut right_node = nodes.get(right_index).unwrap();

                        // walk up tree until no parent
                        loop {
                            match right_node.get_parent() {
                                None => {
                                    break;
                                }
                                Some(parent_index) => {
                                    right_node = nodes.get(parent_index).unwrap();
                                }
                            }
                        }

                        changes.push(Change {
                            index_to_change: node.get_own_index(),
                            new_index: Some(right_node.get_own_index()),
                            side_to_set: NodeSide::Right,
                        });

                        let right_priority = self
                            .operation_priorities
                            .get(&right_node.get_operation())
                            .unwrap();

                        if right_priority < node_priority {
                            // right has higher priority

                            // None right node's left and right
                            // if value precedence
                            if *right_priority == VALUE_PRECEDENCE {
                                changes.push(Change {
                                    index_to_change: right_node.get_own_index(),
                                    new_index: None,
                                    side_to_set: NodeSide::Left,
                                });

                                changes.push(Change {
                                    index_to_change: right_node.get_own_index(),
                                    new_index: None,
                                    side_to_set: NodeSide::Right,
                                });
                            }

                            // Set right node's parent to node
                            changes.push(Change {
                                index_to_change: right_node.get_own_index(),
                                new_index: Some(node.get_own_index()),
                                side_to_set: NodeSide::Parent,
                            });
                        } else {
                            // right has lower priority
                        }

                        // check and update next node only if this one was a value node
                        if *right_priority == VALUE_PRECEDENCE {
                            // right node will no longer be pointing toward its right
                            // need to update left's right to point to node
                            match right_node.get_right() {
                                None => (),
                                Some(right_right_index) => {
                                    let right_right_node = nodes.get(right_right_index).unwrap();

                                    changes.push(Change {
                                        index_to_change: right_right_node.get_own_index(),
                                        new_index: Some(node.get_own_index()),
                                        side_to_set: NodeSide::Left,
                                    });
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

        // starting with highest priority operators
        // go through nodes and move pointers to their operands
        // to point at operator
        // let nodes = &mut nodes;

        // skip VALUE_PRECEDENCE
        for (_i, priority) in priority_map.iter().skip(1).enumerate() {
            if priority.len() > 0 {
                // println!("priority {:?}", i);
                nodes = self.resolve_tree(nodes, &priority);
            }
        }

        // next priority
        // for (i, node) in nodes.iter().enumerate() {
        //     if node.get_operation() == Operation::Addition {}
        // }

        let root = Compiler::find_root_index(&nodes);
        // println!("{}", root);

        return SELTree::new(root, nodes);
    }
}
