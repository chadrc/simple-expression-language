use super::data_type::{get_data_type_for_token, DataType};
use super::operation::{get_operation_type_for_token, Operation};
use super::precedence_manager::PrecedenceManager;
use super::sel_tree::{NodeSide, SELTree, SELTreeNode};
use sel_tokenizer::Tokenizer;

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

struct SELTreeBuilder {
    precedence_manager: PrecedenceManager,
}

impl SELTreeBuilder {
    pub fn new() -> Self {
        return SELTreeBuilder {
            precedence_manager: PrecedenceManager::new(),
        };
    }

    fn make_nodes_from_tokenizer(&mut self, tokenizer: &mut Tokenizer) -> Vec<SELTreeNode> {
        let mut nodes: Vec<SELTreeNode> = vec![];

        // loop trough all tokens
        // convert them to tree nodes
        // and link them together

        let mut last_data_type = DataType::Unknown;

        for token in tokenizer {
            let inserted_index = nodes.len();

            let mut op = get_operation_type_for_token(&token);
            let data_type = get_data_type_for_token(&token);

            if op == Operation::Subtraction && last_data_type == DataType::Unknown {
                // if previous node is not a value
                // this op is actually a Negation operation
                op = Operation::Negation;
            }

            let mut node = SELTreeNode::new(op, data_type, inserted_index);

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

            self.precedence_manager
                .add_index_with_operation(node.get_operation(), node.get_own_index());

            last_data_type = data_type;
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

        loop {
            match node.get_parent() {
                None => {
                    break;
                }
                Some(parent) => {
                    node = nodes.get(parent).unwrap();

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
        for i in indicies_to_resolve {
            let mut changes: Vec<Change> = vec![];
            {
                let nodes = &nodes;
                let node = nodes.get(*i).unwrap();

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
                            let mut next_node = nodes.get(node_index).unwrap();

                            // walk up tree until no parent
                            loop {
                                match next_node.get_parent() {
                                    None => {
                                        break;
                                    }
                                    Some(parent_index) => {
                                        next_node = nodes.get(parent_index).unwrap();
                                    }
                                }
                            }

                            changes.push(Change {
                                index_to_change: node.get_own_index(),
                                new_index: Some(next_node.get_own_index()),
                                side_to_set: side,
                            });

                            let is_value_precedence = self
                                .precedence_manager
                                .is_op_value_precedence(next_node.get_operation());

                            if is_value_precedence {
                                changes.append(&mut none_left_right(next_node.get_own_index()));
                            }

                            if self
                                .precedence_manager
                                .is_lower(next_node.get_operation(), node.get_operation())
                            {
                                changes.push(Change {
                                    index_to_change: node.get_own_index(),
                                    new_index: Some(next_node.get_own_index()),
                                    side_to_set: NodeSide::Parent,
                                });
                            } else {
                                changes.push(Change {
                                    index_to_change: next_node.get_own_index(),
                                    new_index: Some(node.get_own_index()),
                                    side_to_set: NodeSide::Parent,
                                });
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

    fn build(&mut self, s: &String) -> SELTree {
        let mut tokenizer = Tokenizer::new(s);
        let mut nodes = self.make_nodes_from_tokenizer(&mut tokenizer);

        // skip VALUE_PRECEDENCE
        for bucket in self.precedence_manager.get_buckets().iter().skip(1) {
            if bucket.len() > 0 {
                nodes = self.resolve_tree(nodes, &bucket);
            }
        }

        let root = SELTreeBuilder::find_root_index(&nodes);

        return SELTree::new(root, nodes);
    }
}

pub fn build_tree_from_string(s: &String) -> SELTree {
    let mut builder = SELTreeBuilder::new();
    return builder.build(s);
}
