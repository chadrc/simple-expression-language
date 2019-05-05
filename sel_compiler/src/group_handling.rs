use crate::change::Change;
use crate::find_root::find_root_index;
use crate::precedence_manager::PrecedenceGroup;
use crate::utils::apply_changes;
use sel_common::{DataType, NodeSide, Operation, SELTreeNode};

pub fn correct_group(
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

    apply_changes(&mut nodes, changes);

    nodes
}

pub fn update_group(
    mut nodes: Vec<SELTreeNode>,
    precedence_group: &PrecedenceGroup,
) -> Vec<SELTreeNode> {
    // get root of group tree
    let root = find_root_index(&nodes, Some(precedence_group.get_first()));

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
        .filter(|left_node| {
            left_node.get_operation() != Operation::CurrentResult
                && left_node.get_data_type() != DataType::Identifier
        })
        .and_then(|_left_node| {
            changes.push(Change {
                index_to_change: precedence_group.get_parent(),
                new_index: None,
                side_to_set: NodeSide::Left,
            });

            Some(true)
        });

    apply_changes(&mut nodes, changes);

    nodes
}

pub fn identifier_call_check(
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

    apply_changes(&mut nodes, changes);

    nodes
}
