use crate::utils::loop_max;
use sel_common::{Operation, SELTreeNode};

pub fn find_root_index(nodes: &Vec<SELTreeNode>, start_index: Option<usize>) -> usize {
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
