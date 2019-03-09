use super::{get_node_result, SELExecutionResult};
use sel_common::{from_byte_vec, FromByteVec, SELTree, SELTreeNode};

pub fn get_values_from_results<L: FromByteVec, R: FromByteVec>(
    left: &SELExecutionResult,
    right: &SELExecutionResult,
) -> (L, R) {
    let left_val: Option<L> = match left.get_value() {
        Some(value) => Some(from_byte_vec(value)),
        None => None,
    };

    let right_val: Option<R> = match right.get_value() {
        Some(value) => Some(from_byte_vec(value)),
        None => None,
    };

    return (left_val.unwrap(), right_val.unwrap());
}

pub fn get_left_right_results(
    tree: &SELTree,
    node: &SELTreeNode,
) -> (SELExecutionResult, SELExecutionResult) {
    let left = tree.get_nodes().get(node.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(node.get_right().unwrap()).unwrap();

    return (get_node_result(tree, &left), get_node_result(tree, &right));
}
