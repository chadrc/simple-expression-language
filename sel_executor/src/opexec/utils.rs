use super::SELExecutionResult;
use sel_common::{from_byte_vec, FromByteVec};

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
