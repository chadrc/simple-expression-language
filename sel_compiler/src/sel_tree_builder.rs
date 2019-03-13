use super::precedence_manager::PrecedenceManager;
use super::utils::{get_data_type_for_token, get_operation_type_for_token, loop_max};
use sel_common::{DataHeap, DataType, NodeSide, Operation, SELTree, SELTreeNode};
use sel_tokenizer::{TokenType, Tokenizer};

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

fn op_is_terminable(op: Operation) -> bool {
    return op == Operation::Touch || op == Operation::Input;
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

    fn make_nodes_from_tokenizer(
        &mut self,
        tokenizer: &mut Tokenizer,
    ) -> (Vec<SELTreeNode>, DataHeap, Vec<usize>) {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut data: DataHeap = DataHeap::new();
        let mut firsts_of_group: Vec<usize> = vec![];

        // loop trough all tokens
        // convert them to tree nodes
        // and link them together

        let mut last_data_type = DataType::Unknown;
        let mut last_op = Operation::None;
        let mut link_next = true;

        for token in tokenizer {
            let inserted_index = nodes.len();

            if token.get_token_type() == TokenType::LineEnd {
                // set next token to not be linked to previous
                // skip this token, no need to convert LineEnd to a node
                link_next = false;
                continue;
            }

            let mut op = get_operation_type_for_token(&token);
            let data_type = get_data_type_for_token(&token);

            let value = data.insert_from_string(data_type, &token.get_token_str());

            if op == Operation::Subtraction && last_data_type == DataType::Unknown {
                // if previous node is not a value
                // this op is actually a Negation operation
                op = Operation::Negation;
            }

            let mut node = SELTreeNode::new(op, data_type, inserted_index, value);

            if !link_next {
                // check to see if previous node and current node are terminable
                // i.e. a node that can end an expression
                // if not we need to link it

                // right now only value operations can terminate an expression
                // if last op wasn't one of those
                if !op_is_terminable(last_op) || !op_is_terminable(op) {
                    // flip link next so we link previous node with this one
                    link_next = true;
                }
            }

            // because of starter node, there is always a previous node
            if inserted_index > 0 && link_next {
                let previous_index = inserted_index - 1;
                match nodes.get_mut(previous_index) {
                    None => (),
                    Some(previous_node) => {
                        node.set_left(Some(previous_index));
                        previous_node.set_right(Some(inserted_index));
                    }
                }
            }

            // flip back for next node
            if !link_next {
                firsts_of_group.push(inserted_index);
                link_next = true;
            }

            nodes.push(node);

            self.precedence_manager
                .add_index_with_operation(node.get_operation(), node.get_own_index());

            last_data_type = data_type;
            last_op = op;
        }

        // no tokens
        // insert unit node as default
        if nodes.len() == 0 {
            nodes.push(SELTreeNode::new(Operation::None, DataType::Unit, 0, None));
        }

        return (nodes, data, firsts_of_group);
    }

    fn find_root_index(nodes: &Vec<SELTreeNode>, start_index: usize) -> usize {
        // will always have at least one node
        let mut node = nodes.get(start_index).unwrap();

        loop_max(nodes.len(), || match node.get_parent() {
            None => {
                return;
            }
            Some(parent) => {
                node = nodes.get(parent).unwrap();
            }
        });

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
                        None => (),
                        Some(node_index) => {
                            let mut next_node = nodes.get(node_index).unwrap();

                            loop_max(nodes.len(), || match next_node.get_parent() {
                                None => {
                                    return;
                                }
                                Some(parent_index) => {
                                    if parent_index == node.get_own_index() {
                                        return;
                                    }

                                    next_node = nodes.get(parent_index).unwrap();
                                }
                            });

                            let next_is_lower = self
                                .precedence_manager
                                .is_lower(next_node.get_operation(), node.get_operation());

                            if !next_is_lower {
                                changes.push(Change {
                                    index_to_change: node.get_own_index(),
                                    new_index: Some(next_node.get_own_index()),
                                    side_to_set: side,
                                });
                            }

                            let is_value_precedence = self
                                .precedence_manager
                                .is_op_value_precedence(next_node.get_operation());

                            if is_value_precedence {
                                changes.append(&mut none_left_right(next_node.get_own_index()));
                            }

                            if next_is_lower {
                                changes.push(Change {
                                    index_to_change: node.get_own_index(),
                                    new_index: Some(next_node.get_own_index()),
                                    side_to_set: NodeSide::Parent,
                                });

                                // just set the side we're checking to node's parent
                                // set side we're checking to None, since we don't need it anymore

                                changes.push(Change {
                                    index_to_change: node.get_own_index(),
                                    new_index: None,
                                    side_to_set: side,
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
        let (mut nodes, data, firsts_of_group) = self.make_nodes_from_tokenizer(&mut tokenizer);

        // skip VALUE_PRECEDENCE
        for bucket in self.precedence_manager.get_buckets().iter().skip(1) {
            if bucket.len() > 0 {
                nodes = self.resolve_tree(nodes, &bucket);
            }
        }

        // firsts of group doesn't contain very first
        // we find this one by starting at 0
        let root = SELTreeBuilder::find_root_index(&nodes, 0);

        // collect remaining roots by transforming firsts of group
        let sub_roots: Vec<usize> = firsts_of_group
            .iter()
            .map(|first| SELTreeBuilder::find_root_index(&nodes, *first))
            .collect();

        return SELTree::new(root, sub_roots, nodes, data);
    }
}

pub fn build_tree_from_string(s: &String) -> SELTree {
    let mut builder = SELTreeBuilder::new();
    return builder.build(s);
}
