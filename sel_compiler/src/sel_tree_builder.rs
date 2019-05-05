use super::precedence_manager::{PrecedenceGroup, PrecedenceManager};
use super::utils::{get_data_type_for_token, get_operation_type_for_token, loop_max};
use crate::change::{none_left_right, Change};
use crate::find_root::find_root_index;
use crate::group_handling::{correct_group, identifier_call_check, update_group};
use crate::precedence_manager::RIGHT_TO_LEFT_PRECEDENCES;
use crate::process_tokens::make_nodes_from_tokenizer;
use crate::utils::apply_changes;
use sel_common::{DataHeap, DataType, NodeSide, Operation, SELContext, SELTree, SELTreeNode};
use sel_tokenizer::{TokenType, Tokenizer};

struct SELTreeBuilder {}

impl SELTreeBuilder {
    pub fn new() -> Self {
        return SELTreeBuilder {};
    }

    fn resolve_node(
        &self,
        precedence_manager: &PrecedenceManager,
        mut nodes: Vec<SELTreeNode>,
        index: usize,
    ) -> Vec<SELTreeNode> {
        let mut changes: Vec<Change> = vec![];
        {
            let nodes = &nodes;
            let node = nodes.get(index).unwrap();

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

                        let next_is_lower = precedence_manager
                            .is_lower(next_node.get_operation(), node.get_operation());

                        if !next_is_lower {
                            changes.push(Change {
                                index_to_change: node.get_own_index(),
                                new_index: Some(next_node.get_own_index()),
                                side_to_set: side,
                            });
                        }

                        let is_value_precedence =
                            precedence_manager.is_op_value_precedence(next_node.get_operation());

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
            apply_changes(&mut nodes, changes)
        }

        return nodes;
    }

    fn resolve_tree(
        &self,
        precedence_manager: &PrecedenceManager,
        mut nodes: Vec<SELTreeNode>,
        indices_to_resolve: &Vec<usize>,
        right_to_left: bool,
    ) -> Vec<SELTreeNode> {
        if right_to_left {
            for i in indices_to_resolve.iter().rev() {
                nodes = self.resolve_node(precedence_manager, nodes, *i);
            }
        } else {
            for i in indices_to_resolve {
                nodes = self.resolve_node(precedence_manager, nodes, *i);
            }
        }

        return nodes;
    }

    fn build(&mut self, s: &String, context: SELContext) -> SELTree {
        let mut context = context;
        let mut precedence_manager = PrecedenceManager::new();
        let mut tokenizer = Tokenizer::new(s);
        let (mut nodes, data, firsts_of_expression) =
            make_nodes_from_tokenizer(&mut precedence_manager, &mut tokenizer, &mut context);

        let precedence_groups = precedence_manager.get_group_tiers();

        for (index, tier) in precedence_groups.iter().enumerate().rev() {
            for group in tier.iter().rev() {
                // base tier doesn't need any correction
                // before or after creating the tree

                if index != 0 {
                    nodes = correct_group(nodes, group);
                }

                // check identifiers first
                nodes = identifier_call_check(nodes, group.get_members().get(0).unwrap());

                // skip value and group precedences
                for (i, bucket) in group.get_members().iter().skip(2).enumerate() {
                    // plus two to compensate for skipped precedences
                    let right_to_left = RIGHT_TO_LEFT_PRECEDENCES.contains(&(i + 2));
                    nodes = self.resolve_tree(&precedence_manager, nodes, &bucket, right_to_left);
                }

                if index != 0 {
                    nodes = update_group(nodes, group);
                }
            }
        }

        // firsts of group doesn't contain very first
        // we find this one by starting at 0
        let root = find_root_index(&nodes, None);

        // collect remaining roots by transforming firsts of group
        let sub_roots: Vec<usize> = firsts_of_expression
            .iter()
            .map(|first| find_root_index(&nodes, Some(*first)))
            .collect();

        return SELTree::new(root, sub_roots, nodes, data, context);
    }
}

pub fn build_tree_from_string(s: &String, context: SELContext) -> SELTree {
    let mut builder = SELTreeBuilder::new();
    return builder.build(s, context);
}
