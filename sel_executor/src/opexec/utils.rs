use super::{get_node_result, SELExecutionResult};
use sel_common::{from_byte_vec, to_byte_vec, DataType, FromByteVec, SELTree, SELTreeNode};

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

pub struct MathOps<I, F>
where
    I: Fn(i32, i32) -> i32,
    F: Fn(f64, f64) -> f64,
{
    pub perform_integer: I,
    pub perform_float: F,
}

pub fn match_math_ops<I, F>(
    tree: &SELTree,
    node: &SELTreeNode,
    ops: MathOps<I, F>,
) -> Option<SELExecutionResult>
where
    I: Fn(i32, i32) -> i32,
    F: Fn(f64, f64) -> f64,
{
    let (left_result, right_result) = get_left_right_results(tree, node);

    return match (left_result.get_type(), right_result.get_type()) {
        (DataType::Integer, DataType::Integer) => {
            let (left_val, right_val) =
                get_values_from_results::<i32, i32>(&left_result, &right_result);

            let result = (ops.perform_integer)(left_val, right_val);

            Some(SELExecutionResult::new(
                DataType::Integer,
                Some(to_byte_vec(result)),
            ))
        }
        (DataType::Integer, DataType::Decimal) => {
            let (left_val, right_val) =
                get_values_from_results::<i32, f64>(&left_result, &right_result);

            let result = (ops.perform_float)(f64::from(left_val), right_val);

            Some(SELExecutionResult::new(
                DataType::Decimal,
                Some(to_byte_vec(result)),
            ))
        }
        (DataType::Decimal, DataType::Integer) => {
            let (left_val, right_val) =
                get_values_from_results::<f64, i32>(&left_result, &right_result);

            let result = (ops.perform_float)(left_val, f64::from(right_val));

            Some(SELExecutionResult::new(
                DataType::Decimal,
                Some(to_byte_vec(result)),
            ))
        }
        (DataType::Decimal, DataType::Decimal) => {
            let (left_val, right_val) =
                get_values_from_results::<f64, f64>(&left_result, &right_result);

            let result = (ops.perform_float)(left_val, right_val);

            Some(SELExecutionResult::new(
                DataType::Decimal,
                Some(to_byte_vec(result)),
            ))
        }
        (_, DataType::Unit) | (DataType::Unit, _) => {
            Some(SELExecutionResult::new(DataType::Unit, None))
        }
        _ => None,
    };
}
