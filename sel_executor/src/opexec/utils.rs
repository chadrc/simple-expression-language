use super::{get_node_result, SELExecutionContext, SELExecutionResult};
use sel_common::{
    from_byte_vec, to_byte_vec, DataType, FromByteVec, SELTree, SELTreeNode, ToByteVec,
};

pub fn get_values_from_results<L: FromByteVec, R: FromByteVec>(
    left: &SELExecutionResult,
    right: &SELExecutionResult,
) -> (L, R) {
    let left_val: L = get_value_from_result(left);
    let right_val: R = get_value_from_result(right);
    return (left_val, right_val);
}

pub fn get_value_from_result<T: FromByteVec>(result: &SELExecutionResult) -> T {
    let val: Option<T> = match result.get_value() {
        Some(value) => Some(from_byte_vec(value)),
        None => None,
    };

    return val.unwrap();
}

pub fn get_left_right_results(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> (SELExecutionResult, SELExecutionResult) {
    let left = tree.get_nodes().get(node.get_left().unwrap()).unwrap();
    let right = tree.get_nodes().get(node.get_right().unwrap()).unwrap();

    return (
        get_node_result(tree, &left, context),
        get_node_result(tree, &right, context),
    );
}

pub enum OptionOr<T, V> {
    Some(T),
    Or(V),
}

fn match_int_dec_ops<FI, FF, RI, RF, FU, RU>(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
    integer_func: FI,
    float_func: FF,
    integer_type: DataType,
    float_type: DataType,
    unit_func: FU,
) -> OptionOr<SELExecutionResult, (SELExecutionResult, SELExecutionResult)>
where
    FI: Fn(i64, i64) -> RI,
    FF: Fn(f64, f64) -> RF,
    RI: ToByteVec,
    RF: ToByteVec,
    FU: Fn(bool, bool) -> (DataType, Option<RU>),
    RU: ToByteVec,
{
    let (left_result, right_result) = get_left_right_results(tree, node, context);

    return match (left_result.get_type(), right_result.get_type()) {
        (DataType::Integer, DataType::Integer) => {
            let (left_val, right_val) =
                get_values_from_results::<i64, i64>(&left_result, &right_result);

            let result = integer_func(left_val, right_val);

            OptionOr::Some(SELExecutionResult::new(
                integer_type,
                Some(to_byte_vec(result)),
            ))
        }
        (DataType::Integer, DataType::Decimal) => {
            let (left_val, right_val) =
                get_values_from_results::<i64, f64>(&left_result, &right_result);

            let result = float_func(left_val as f64, right_val);

            OptionOr::Some(SELExecutionResult::new(
                float_type,
                Some(to_byte_vec(result)),
            ))
        }
        (DataType::Decimal, DataType::Integer) => {
            let (left_val, right_val) =
                get_values_from_results::<f64, i64>(&left_result, &right_result);

            let result = float_func(left_val, right_val as f64);

            OptionOr::Some(SELExecutionResult::new(
                float_type,
                Some(to_byte_vec(result)),
            ))
        }
        (DataType::Decimal, DataType::Decimal) => {
            let (left_val, right_val) =
                get_values_from_results::<f64, f64>(&left_result, &right_result);

            let result = float_func(left_val, right_val);

            OptionOr::Some(SELExecutionResult::new(
                float_type,
                Some(to_byte_vec(result)),
            ))
        }
        (DataType::Unit, DataType::Unit) => {
            let (data_type, value) = unit_func(true, true);

            let bytes = match value {
                Some(v) => Some(to_byte_vec(v)),
                None => None,
            };

            OptionOr::Some(SELExecutionResult::new(data_type, bytes))
        }
        (_, DataType::Unit) => {
            let (data_type, value) = unit_func(false, true);

            let bytes = match value {
                Some(v) => Some(to_byte_vec(v)),
                None => None,
            };

            OptionOr::Some(SELExecutionResult::new(data_type, bytes))
        }
        (DataType::Unit, _) => {
            let (data_type, value) = unit_func(true, false);

            let bytes = match value {
                Some(v) => Some(to_byte_vec(v)),
                None => None,
            };

            OptionOr::Some(SELExecutionResult::new(data_type, bytes))
        }
        _ => OptionOr::Or((left_result, right_result)),
    };
}

pub fn match_math_ops<FI, FF, RI, RF>(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
    integer_func: FI,
    float_func: FF,
) -> OptionOr<SELExecutionResult, (SELExecutionResult, SELExecutionResult)>
where
    FI: Fn(i64, i64) -> RI,
    FF: Fn(f64, f64) -> RF,
    RI: ToByteVec,
    RF: ToByteVec,
{
    return match_int_dec_ops(
        tree,
        node,
        context,
        integer_func,
        float_func,
        DataType::Integer,
        DataType::Decimal,
        |_l, _r| -> (DataType, Option<bool>) { (DataType::Unit, None) },
    );
}

pub fn match_int_math_ops<FI, FF, R>(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
    integer_func: FI,
    float_func: FF,
) -> OptionOr<SELExecutionResult, (SELExecutionResult, SELExecutionResult)>
where
    FI: Fn(i64, i64) -> R,
    FF: Fn(f64, f64) -> R,
    R: ToByteVec,
{
    return match_int_dec_ops(
        tree,
        node,
        context,
        integer_func,
        float_func,
        DataType::Integer,
        DataType::Integer,
        |_l, _r| -> (DataType, Option<bool>) { (DataType::Unit, None) },
    );
}

pub fn match_comparison_ops<FI, FF, FS>(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
    integer_func: FI,
    float_func: FF,
    string_func: FS,
) -> SELExecutionResult
where
    FI: Fn(i64, i64) -> bool,
    FF: Fn(f64, f64) -> bool,
    FS: Fn(&String, &String) -> bool,
{
    return match match_int_dec_ops(
        tree,
        node,
        context,
        integer_func,
        float_func,
        DataType::Boolean,
        DataType::Boolean,
        |_l, _r| -> (DataType, Option<bool>) { (DataType::Unit, None) },
    ) {
        OptionOr::Some(result) => result,
        OptionOr::Or((left, right)) => match (left.get_type(), right.get_type()) {
            (DataType::String, DataType::String) => {
                let (left_val, right_val) =
                    get_values_from_results::<String, String>(&left, &right);

                let result = string_func(&left_val, &right_val);

                SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(result)))
            }
            _ => SELExecutionResult::new(DataType::Unknown, Some(vec![])),
        },
    };
}

pub fn match_equality_ops<FI, FF, FS, FU>(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
    integer_func: FI,
    float_func: FF,
    string_func: FS,
    unit_func: FU,
) -> SELExecutionResult
where
    FI: Fn(i64, i64) -> bool,
    FF: Fn(f64, f64) -> bool,
    FS: Fn(&String, &String) -> bool,
    FU: Fn(bool, bool) -> (DataType, Option<bool>),
{
    return match match_int_dec_ops(
        tree,
        node,
        context,
        integer_func,
        float_func,
        DataType::Boolean,
        DataType::Boolean,
        unit_func,
    ) {
        OptionOr::Some(result) => result,
        OptionOr::Or((left, right)) => match (left.get_type(), right.get_type()) {
            (DataType::String, DataType::String) => {
                let (left_val, right_val) =
                    get_values_from_results::<String, String>(&left, &right);

                let result = string_func(&left_val, &right_val);

                SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(result)))
            }
            _ => SELExecutionResult::new(DataType::Unknown, Some(vec![])),
        },
    };
}
