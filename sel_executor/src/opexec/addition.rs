use super::execution_result::SELExecutionResult;
use super::get_node_result;
use sel_common::{from_byte_vec, to_byte_vec, DataType, SELTree, SELTreeNode};

pub fn addition_operation(tree: &SELTree, node: &SELTreeNode) -> SELExecutionResult {
    let left = tree.get_nodes().get(node.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(node.get_right().unwrap()).unwrap();

    let left_result = get_node_result(tree, &left);
    let right_result = get_node_result(tree, &right);

    let operand_types = (left_result.get_type(), right_result.get_type());

    let (result, result_type) = match operand_types {
        (DataType::Integer, DataType::Integer) => {
            let left_int: Option<i32> = match left_result.get_value() {
                Some(value) => Some(from_byte_vec(value)),
                None => None,
            };

            let right_int: Option<i32> = match right_result.get_value() {
                Some(value) => Some(from_byte_vec(value)),
                None => None,
            };

            let result = left_int.unwrap() + right_int.unwrap();

            (Some(to_byte_vec(result)), DataType::Integer)
        }
        (DataType::Integer, DataType::Decimal) => {
            let left_int: Option<i32> = match left_result.get_value() {
                Some(value) => Some(from_byte_vec(value)),
                None => None,
            };

            let right_int: Option<f64> = match right_result.get_value() {
                Some(value) => Some(from_byte_vec(value)),
                None => None,
            };

            let result = f64::from(left_int.unwrap()) + right_int.unwrap();

            (Some(to_byte_vec(result)), DataType::Decimal)
        }
        (DataType::Decimal, DataType::Integer) => {
            let left_int: Option<f64> = match left_result.get_value() {
                Some(value) => Some(from_byte_vec(value)),
                None => None,
            };

            let right_int: Option<i32> = match right_result.get_value() {
                Some(value) => Some(from_byte_vec(value)),
                None => None,
            };

            let result = left_int.unwrap() + f64::from(right_int.unwrap());

            (Some(to_byte_vec(result)), DataType::Decimal)
        }
        _ => (Some(vec![]), DataType::Unknown),
    };

    return SELExecutionResult::new(result_type, result);
}
