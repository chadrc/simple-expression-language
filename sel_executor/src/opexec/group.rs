use sel_common::{DataType, SELFunction, SELTree, SELTreeNode, SELValue};

use super::execution_result::SELExecutionResult;
use super::{get_node_result, SELExecutionContext};
use crate::opexec::utils::get_value_from_result;

pub fn operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    let call_result = |sel_value: SELValue| -> Option<SELExecutionResult> {
        match node.get_left() {
            // having left index means this is a call operation
            Some(left_index) => {
                tree.get_nodes()
                    .get(left_index)
                    // have a left, get symbol index
                    .and_then(|left_node| tree.get_usize_value_of(left_node))
                    // get symbol string
                    .and_then(|symbol_index| tree.get_symbol_table().get_symbol(symbol_index))
                    // get function
                    // if we have gotten to this point
                    // the identifier should resolve to a function
                    .and_then(|symbol| context.get_function(symbol))
                    // if no function found map directly to a Unit value
                    // else map using the found function
                    .map_or(Some(SELExecutionResult::from(&SELValue::new())), |func| {
                        Some(SELExecutionResult::from(&func(sel_value)))
                    })
            }
            // if no left side
            // simple group operation, return that result
            None => None,
        }
    };

    let result_opt = match node.get_right() {
        Some(right_index) => tree
            .get_nodes()
            .get(right_index)
            // attempt call op with result from right tree
            .and_then(|right_node| {
                let result = get_node_result(tree, right_node, context);
                call_result(result.get_sel_value().to_owned()).or(Some(result))
            }),
        // attempt call op with Unit value
        None => call_result(SELValue::new()),
    };

    return result_opt.unwrap_or(SELExecutionResult::new(DataType::Unknown, None));
}

#[cfg(test)]
mod tests {
    use sel_common::{from_byte_vec, DataType, SELContext, SELValue};
    use sel_compiler::Compiler;

    use super::super::super::execute_sel_tree;
    use super::*;

    #[test]
    fn executes_group() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("5 * (4 + 3)"));

        let context = SELExecutionContext::new();

        let results = execute_sel_tree(&tree, &context);

        let first_result = results.get(0).unwrap();
        let first_result_value = match first_result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(first_result.get_type(), DataType::Integer);
        assert_eq!(first_result_value, Some(35));
    }

    #[test]
    fn executes_call() {
        let compiler = Compiler::new();
        let mut context = SELContext::new();
        context.register_function("get_vars", |sel_value: SELValue| SELValue::new_from_int(10));

        let execution_context = SELExecutionContext::from(&context);

        let tree = compiler.compile_with_context(&String::from("get_vars()"), context);

        let results = execute_sel_tree(&tree, &execution_context);

        let first_result = results.get(0).unwrap();
        let first_result_value = match first_result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(first_result.get_type(), DataType::Integer);
        assert_eq!(first_result_value, Some(10));
    }
}
