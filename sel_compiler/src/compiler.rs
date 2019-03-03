use super::data_type::{get_data_type_for_token, DataType};
use super::operation::{get_operation_type_for_token, Operation};
use super::sel_tree::{NodeSide, SELTree, SELTreeNode};
use sel_tokenizer::Tokenizer;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Copy)]
struct Change {
    index_to_change: usize,
    new_index: usize,
    side_to_set: NodeSide,
}

pub struct Compiler {
    operation_priorities: HashMap<Operation, usize>,
}

// lower number means higher priority
const NOT_PRECEDENCE: usize = 0;
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
                        node.set_left(previous_index);
                        previous_node.set_right(inserted_index);
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
        println!("{:?}", indicies_to_resolve);

        for i in indicies_to_resolve {
            let mut changes: Vec<Change> = vec![];
            {
                let nodes = &nodes;
                let node = nodes.get(*i).unwrap();
                let mut pending_parent = false;

                let my_priority = self
                    .operation_priorities
                    .get(&node.get_operation())
                    .unwrap();

                match node.get_left() {
                    None => (),
                    Some(left_index) => {
                        let left = nodes.get(left_index).unwrap();

                        // only need to update if a value type
                        if left.get_value().get_data_type() != DataType::Unknown {
                            // update operands parent to point to operator
                            changes.push(Change {
                                index_to_change: left.get_own_index(),
                                new_index: node.get_own_index(),
                                side_to_set: NodeSide::Parent,
                            });

                            match left.get_left() {
                                None => (),
                                Some(left_left_index) => {
                                    let left_left = nodes.get(left_left_index).unwrap();

                                    let their_priority = self
                                        .operation_priorities
                                        .get(&left_left.get_operation())
                                        .unwrap();

                                    // lower number is higher priority
                                    // i.e. priority of 0 is higher than 1
                                    // 1 > 0 == true, means their_priority is lower than my_priority
                                    if their_priority > my_priority {
                                        // if lower priority
                                        // make its right point to node
                                        changes.push(Change {
                                            index_to_change: left_left.get_own_index(),
                                            new_index: node.get_own_index(),
                                            side_to_set: NodeSide::Right,
                                        });

                                        changes.push(Change {
                                            index_to_change: node.get_own_index(),
                                            new_index: left_left.get_own_index(),
                                            side_to_set: NodeSide::Parent,
                                        });

                                        pending_parent = true;
                                    } else {
                                        // same or higher priority
                                        // make node's left point to it
                                        changes.push(Change {
                                            index_to_change: node.get_own_index(),
                                            new_index: left_left.get_own_index(),
                                            side_to_set: NodeSide::Left,
                                        });
                                    }
                                }
                            }
                        } else {
                            match left.get_parent() {
                                None => (),
                                Some(parent_index) => {
                                    if parent_index != node.get_own_index() {
                                        let parent = nodes.get(parent_index).unwrap();

                                        let their_priority = self
                                            .operation_priorities
                                            .get(&parent.get_operation())
                                            .unwrap();

                                        if their_priority >= my_priority {
                                            // same or lower
                                            changes.push(Change {
                                                index_to_change: parent.get_own_index(),
                                                new_index: node.get_own_index(),
                                                side_to_set: NodeSide::Parent,
                                            });

                                            changes.push(Change {
                                                index_to_change: node.get_own_index(),
                                                new_index: parent.get_own_index(),
                                                side_to_set: NodeSide::Left,
                                            });
                                        }
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
                        if right.get_value().get_data_type() != DataType::Unknown {
                            changes.push(Change {
                                index_to_change: right.get_own_index(),
                                new_index: node.get_own_index(),
                                side_to_set: NodeSide::Parent,
                            });
                            match right.get_right() {
                                None => (),
                                Some(right_right_index) => {
                                    let right_right = nodes.get(right_right_index).unwrap();

                                    let their_priority = self
                                        .operation_priorities
                                        .get(&right_right.get_operation())
                                        .unwrap();

                                    if their_priority > my_priority {
                                        // lower priority
                                        // make its left point to node
                                        changes.push(Change {
                                            index_to_change: right_right.get_own_index(),
                                            new_index: node.get_own_index(),
                                            side_to_set: NodeSide::Left,
                                        });

                                        // only queue one parent change
                                        // left checked first so it decideds
                                        if !pending_parent {
                                            changes.push(Change {
                                                index_to_change: node.get_own_index(),
                                                new_index: right_right.get_own_index(),
                                                side_to_set: NodeSide::Parent,
                                            });
                                        }
                                    } else {
                                        changes.push(Change {
                                            index_to_change: node.get_own_index(),
                                            new_index: right_right.get_own_index(),
                                            side_to_set: NodeSide::Parent,
                                        });
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
        let (mut nodes, priority_map) = self.make_nodes_from_tokenizer(&mut tokenizer);

        // starting with highest priority operators
        // go through nodes and move pointers to their operands
        // to point at operator
        // let nodes = &mut nodes;

        for priority in priority_map {
            if priority.len() > 0 {
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
