use crate::change::{none_left_right, Change};
use crate::precedence_manager::PrecedenceManager;
use crate::utils::{apply_changes, loop_max};
use sel_common::{NodeSide, SELTreeNode};

pub fn resolve_node(
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
