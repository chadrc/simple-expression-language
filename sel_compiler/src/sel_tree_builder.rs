use super::precedence_manager::{PrecedenceGroup, PrecedenceManager};
use super::utils::{get_data_type_for_token, get_operation_type_for_token, loop_max};
use crate::precedence_manager::RIGHT_TO_LEFT_PRECEDENCES;
use sel_common::{DataHeap, DataType, NodeSide, Operation, SELContext, SELTree, SELTreeNode};
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
    return op == Operation::Touch || op == Operation::Input || op == Operation::CurrentResult;
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
        context: &mut SELContext,
    ) -> (Vec<SELTreeNode>, DataHeap, Vec<usize>) {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut data = DataHeap::new();
        let mut firsts_of_expression: Vec<usize> = vec![];

        // loop trough all tokens
        // convert them to tree nodes
        // and link them together

        let mut last_data_type = DataType::Unknown;
        let mut last_op = Operation::None;
        let mut link_next = true;
        let mut symbol_next = false;
        let mut empty_group = false;

        for token in tokenizer {
            let inserted_index = nodes.len();
            let previous_index = if nodes.len() > 0 {
                inserted_index - 1
            } else {
                0
            };

            if token.get_token_type() == TokenType::Comment {
                // will store later for meta data
                // for now, just drop the token
                continue;
            }

            if token.get_token_type() == TokenType::LineEnd {
                // set next token to not be linked to previous
                // skip this token, no need to convert LineEnd to a node
                link_next = false;
                continue;
            }

            if token.get_token_type() == TokenType::EndGroup
                || token.get_token_type() == TokenType::EndAssociativeList
                || token.get_token_type() == TokenType::EndExpressionBlock
            {
                // end current group
                // and drop token
                self.precedence_manager.end_group();
                continue;
            }

            if symbol_next {
                let symbol_value = context.add_symbol(&token.get_token_str());
                nodes
                    .get_mut(previous_index)
                    .and_then(|previous_node| -> Option<usize> {
                        previous_node.set_value(data.insert_integer(symbol_value as i64));
                        None
                    });
                symbol_next = false;
                continue;
            }

            let mut op = get_operation_type_for_token(&token);
            let mut data_type = get_data_type_for_token(&token);

            if data_type == DataType::Symbol {
                symbol_next = true;
            } else if data_type == DataType::Unit {
                // if Unit symbol immediately follows an identifier
                // it is an empty argument list
                if last_data_type == DataType::Identifier {
                    op = Operation::Group;
                    data_type = DataType::Unknown;
                    empty_group = true; // so we don't start a group later
                }
            }

            let value = if token.get_token_type() == TokenType::Identifier {
                let symbol_value = context.add_symbol(&token.get_token_str());
                data.insert_integer(symbol_value as i64)
            } else {
                data.insert_from_string(data_type, &token.get_token_str())
            };

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

            if inserted_index > 0 && link_next {
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
                firsts_of_expression.push(inserted_index);
                link_next = true;
            }

            nodes.push(node);

            self.precedence_manager
                .add_index_with_operation(node.get_operation(), node.get_own_index());

            if (node.get_operation() == Operation::Group && !empty_group)
                || node.get_operation() == Operation::AssociativeList
                || node.get_operation() == Operation::Expression
            {
                self.precedence_manager.start_group();
            }

            last_data_type = data_type;
            last_op = op;
        }

        // no tokens
        // insert unit node as default
        if nodes.len() == 0 {
            nodes.push(SELTreeNode::new(Operation::None, DataType::Unit, 0, None));
        }

        return (nodes, data, firsts_of_expression);
    }

    fn find_root_index(nodes: &Vec<SELTreeNode>, start_index: Option<usize>) -> usize {
        let mut node = match start_index {
            Some(index) => nodes.get(index).unwrap(),
            None => {
                // find first node in tree
                let start_index = 0;

                let mut node = nodes.get(start_index).unwrap();

                // find first node in tree
                // need this until we clean up orphan nodes
                while node.get_operation() == Operation::Group {
                    node = nodes.get(node.get_own_index() + 1).unwrap();
                }

                node
            }
        };

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

    fn resolve_node(&self, mut nodes: Vec<SELTreeNode>, index: usize) -> Vec<SELTreeNode> {
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
            SELTreeBuilder::apply_changes(&mut nodes, changes)
        }

        return nodes;
    }

    fn resolve_tree(
        &self,
        mut nodes: Vec<SELTreeNode>,
        indices_to_resolve: &Vec<usize>,
        right_to_left: bool,
    ) -> Vec<SELTreeNode> {
        if right_to_left {
            for i in indices_to_resolve.iter().rev() {
                nodes = self.resolve_node(nodes, *i);
            }
        } else {
            for i in indices_to_resolve {
                nodes = self.resolve_node(nodes, *i);
            }
        }

        return nodes;
    }

    fn correct_group(
        &self,
        mut nodes: Vec<SELTreeNode>,
        precedence_group: &PrecedenceGroup,
    ) -> Vec<SELTreeNode> {
        let mut changes: Vec<Change> = vec![];

        // each group will be made into a tree to begin with
        // in order to do this, we'll set the
        // first node's left and the last node's right
        // to None

        changes.push(Change {
            index_to_change: precedence_group.get_first(),
            new_index: None,
            side_to_set: NodeSide::Left,
        });

        // take last node in group and make its right's left the group node
        nodes
            // get last node in group
            .get(precedence_group.get_last())
            // only update if not a group operation
            // group operations are evaluated in order
            // so a group operation's right is the sub tree for that group
            .filter(|last| {
                last.get_operation() != Operation::Group
                    && last.get_operation() != Operation::AssociativeList
            })
            // check if last node has a right
            .and_then(|last| {
                // set node's right to none
                // marking it as end of sequence
                changes.push(Change {
                    index_to_change: precedence_group.get_last(),
                    new_index: None,
                    side_to_set: NodeSide::Right,
                });

                last.get_right()
            })
            // if it does, update its left to be the groups parent node
            .and_then(|lasts_right_index| -> Option<usize> {
                changes.push(Change {
                    index_to_change: lasts_right_index,
                    new_index: Some(precedence_group.get_parent()),
                    side_to_set: NodeSide::Left,
                });

                None
            });

        SELTreeBuilder::apply_changes(&mut nodes, changes);

        nodes
    }

    fn update_group(
        &self,
        mut nodes: Vec<SELTreeNode>,
        precedence_group: &PrecedenceGroup,
    ) -> Vec<SELTreeNode> {
        // get root of group tree
        let root = SELTreeBuilder::find_root_index(&nodes, Some(precedence_group.get_first()));

        // update root's parent to be the group parent
        // and group parent's right to be the root
        let mut changes: Vec<Change> = vec![];

        // if root is the parent root
        // we would end up root pointing to itself
        // we don't want that, and this allows empty groups
        if root != precedence_group.get_parent() {
            changes.push(Change {
                index_to_change: root,
                new_index: Some(precedence_group.get_parent()),
                side_to_set: NodeSide::Parent,
            });

            changes.push(Change {
                index_to_change: precedence_group.get_parent(),
                new_index: Some(root),
                side_to_set: NodeSide::Right,
            });
        }

        // Check group's left side
        // if anything but an identifier
        // set to none
        nodes
            .get(precedence_group.get_parent())
            .and_then(|parent_node| parent_node.get_left())
            .and_then(|left_index| nodes.get(left_index))
            .filter(|left_node| left_node.get_data_type() != DataType::Identifier)
            .and_then(|_left_node| {
                changes.push(Change {
                    index_to_change: precedence_group.get_parent(),
                    new_index: None,
                    side_to_set: NodeSide::Left,
                });

                Some(true)
            });

        SELTreeBuilder::apply_changes(&mut nodes, changes);

        nodes
    }

    fn identifier_call_check(
        &self,
        mut nodes: Vec<SELTreeNode>,
        indices_to_resolve: &Vec<usize>,
    ) -> Vec<SELTreeNode> {
        let mut changes: Vec<Change> = vec![];

        // if an identifier is followed by a group operation
        // it is a call operation
        // make the group the parent of the identifier
        // set identifier's right to None
        // also, make the identifier's left's right point to the group

        for index in indices_to_resolve {
            nodes.get(*index).map(|node| {
                node.get_right()
                    .and_then(|right_index| nodes.get(right_index))
                    .and_then(|right_node| {
                        if right_node.get_operation() == Operation::Group {
                            changes.push(Change {
                                index_to_change: *index,
                                new_index: Some(right_node.get_own_index()),
                                side_to_set: NodeSide::Parent,
                            });

                            changes.push(Change {
                                index_to_change: *index,
                                new_index: None,
                                side_to_set: NodeSide::Right,
                            });

                            node.get_left().and_then(|left_index| {
                                changes.push(Change {
                                    index_to_change: left_index,
                                    new_index: Some(right_node.get_own_index()),
                                    side_to_set: NodeSide::Right,
                                });
                                Some(true)
                            });
                        }

                        Some(true)
                    })
            });
        }

        SELTreeBuilder::apply_changes(&mut nodes, changes);

        nodes
    }

    fn apply_changes(nodes: &mut Vec<SELTreeNode>, changes: Vec<Change>) {
        for change in changes {
            let node = nodes.get_mut(change.index_to_change).unwrap();

            match change.side_to_set {
                NodeSide::Left => node.set_left(change.new_index),
                NodeSide::Right => node.set_right(change.new_index),
                NodeSide::Parent => node.set_parent(change.new_index),
            }
        }
    }

    fn build(&mut self, s: &String, context: SELContext) -> SELTree {
        let mut context = context;
        let mut tokenizer = Tokenizer::new(s);
        let (mut nodes, data, firsts_of_expression) =
            self.make_nodes_from_tokenizer(&mut tokenizer, &mut context);

        let precedence_groups = self.precedence_manager.get_group_tiers();

        for (index, tier) in precedence_groups.iter().enumerate().rev() {
            for group in tier.iter().rev() {
                // base tier doesn't need any correction
                // before or after creating the tree

                if index != 0 {
                    nodes = self.correct_group(nodes, group);
                }

                // check identifiers first
                nodes = self.identifier_call_check(nodes, group.get_members().get(0).unwrap());

                // skip value and group precedences
                for (i, bucket) in group.get_members().iter().skip(2).enumerate() {
                    // plus two to compensate for skipped precedences
                    let right_to_left = RIGHT_TO_LEFT_PRECEDENCES.contains(&(i + 2));
                    nodes = self.resolve_tree(nodes, &bucket, right_to_left);
                }

                if index != 0 {
                    nodes = self.update_group(nodes, group);
                }
            }
        }

        // firsts of group doesn't contain very first
        // we find this one by starting at 0
        let root = SELTreeBuilder::find_root_index(&nodes, None);

        // collect remaining roots by transforming firsts of group
        let sub_roots: Vec<usize> = firsts_of_expression
            .iter()
            .map(|first| SELTreeBuilder::find_root_index(&nodes, Some(*first)))
            .collect();

        return SELTree::new(root, sub_roots, nodes, data, context);
    }
}

pub fn build_tree_from_string(s: &String, context: SELContext) -> SELTree {
    let mut builder = SELTreeBuilder::new();
    return builder.build(s, context);
}
