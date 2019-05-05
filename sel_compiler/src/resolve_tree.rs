use crate::precedence_manager::PrecedenceManager;
use crate::resolve_node::resolve_node;
use sel_common::SELTreeNode;

pub fn resolve_tree(
    precedence_manager: &PrecedenceManager,
    mut nodes: Vec<SELTreeNode>,
    indices_to_resolve: &Vec<usize>,
    right_to_left: bool,
) -> Vec<SELTreeNode> {
    if right_to_left {
        for i in indices_to_resolve.iter().rev() {
            nodes = resolve_node(precedence_manager, nodes, *i);
        }
    } else {
        for i in indices_to_resolve {
            nodes = resolve_node(precedence_manager, nodes, *i);
        }
    }

    return nodes;
}
