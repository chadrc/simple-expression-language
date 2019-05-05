use sel_common::NodeSide;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Change {
    pub index_to_change: usize,
    pub new_index: Option<usize>,
    pub side_to_set: NodeSide,
}

pub fn none_left_right(index: usize) -> Vec<Change> {
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
