use super::execution_result::SELExecutionResult;
use super::{get_node_result, SELExecutionContext};
use sel_common::sel_types::associative_list::AssociativeList;
use sel_common::sel_types::expression::Expression;
use sel_common::sel_types::list::List;
use sel_common::{from_byte_vec, DataType, Operation, SELTree, SELTreeNode, SELValue};

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
                    .and_then(|left_node| {
                        match left_node.get_operation() {
                            Operation::Touch => {
                                tree.get_usize_value_of(left_node)
                                    // get symbol string
                                    .and_then(|symbol_index| {
                                        tree.get_symbol_table().get_symbol(symbol_index)
                                    })
                                    // get function
                                    // if we have gotten to this point
                                    // the identifier should resolve to a function
                                    .and_then(|symbol| context.get_function(symbol))
                                    // if no function found map directly to a Unit value
                                    // else map using the found function
                                    .map_or(
                                        Some(SELExecutionResult::from(&SELValue::new())),
                                        |func| {
                                            Some(SELExecutionResult::from(&func(
                                                sel_value,
                                                tree.get_symbol_table(),
                                            )))
                                        },
                                    )
                            }
                            _ => {
                                let left_result = get_node_result(tree, left_node, context);
                                match left_result.get_type() {
                                    DataType::Expression => {
                                        let expr: Expression =
                                            from_byte_vec(left_result.get_value().unwrap());

                                        // make new context for expression execution
                                        let mut expr_context = context.clone();
                                        expr_context.set_input(sel_value);

                                        expr.get_root()
                                            .and_then(|expr_root_index| {
                                                tree.get_nodes().get(expr_root_index)
                                            })
                                            .map(|expr_root_node| {
                                                get_node_result(tree, expr_root_node, &expr_context)
                                            })
                                    }
                                    _ => None,
                                }
                            }
                        }
                    })
            }
            // no left means this is just a group op
            // using None to let caller handle things as normal
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

                match result.get_type() {
                    DataType::List => {
                        let mut func_sel_value = result.get_sel_value().clone();
                        let list: List = from_byte_vec(result.get_value().unwrap());
                        // if there are any pairs in list
                        // promote to associative array for function call
                        for value in list.get_values() {
                            if value.get_type() == DataType::Pair {
                                func_sel_value = SELValue::new_from_associative_list(
                                    AssociativeList::from(list),
                                );
                                break;
                            }
                        }

                        println!("result {:?}", result);

                        println!("value {:?}", func_sel_value);

                        call_result(func_sel_value).or(Some(result))
                    }
                    _ => call_result(result.get_sel_value().to_owned()).or(Some(result)),
                }
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
    use sel_common::sel_types::associative_list::AssociativeList;

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
        context.register_function("get_vars", |_sel_value: SELValue, symbol_table| {
            SELValue::new_from_int(10)
        });

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

    #[test]
    fn executes_call_with_single_arg() {
        let compiler = Compiler::new();
        let mut context = SELContext::new();

        context.register_function("get_vars", |sel_value: SELValue, symbol_table| {
            let arg: i64 = sel_value.get_value().map_or(0, |val| from_byte_vec(val));

            SELValue::new_from_int(arg * 10)
        });

        let execution_context = SELExecutionContext::from(&context);

        let tree = compiler.compile_with_context(&String::from("get_vars(10)"), context);

        let results = execute_sel_tree(&tree, &execution_context);

        let first_result = results.get(0).unwrap();
        let first_result_value = match first_result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(first_result.get_type(), DataType::Integer);
        assert_eq!(first_result_value, Some(100));
    }

    #[test]
    fn executes_call_with_single_associative_args() {
        let compiler = Compiler::new();
        let mut context = SELContext::new();

        context.register_function("middle", |sel_value, symbol_table| {
            let args: AssociativeList = from_byte_vec(sel_value.get_value().unwrap());

            let first_value: i64 = from_byte_vec(
                args.get_by_association_index(
                    *symbol_table.get_value(&String::from("lower")).unwrap(),
                )
                .unwrap()
                .get_value()
                .unwrap(),
            );

            let second_value: i64 = from_byte_vec(
                args.get_by_association_index(
                    *symbol_table.get_value(&String::from("upper")).unwrap(),
                )
                .unwrap()
                .get_value()
                .unwrap(),
            );

            let value: i64 = (second_value - first_value) / 2 + first_value;

            SELValue::new_from_int(value)
        });

        let execution_context = SELExecutionContext::from(&context);

        let tree = compiler
            .compile_with_context(&String::from("middle(:lower = 10, :upper = 20)"), context);

        let results = execute_sel_tree(&tree, &execution_context);

        let result = results.get(0).unwrap();
        let result_value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(result_value, 15);
    }

    #[test]
    fn executes_call_with_unregistered_function() {
        let compiler = Compiler::new();

        let execution_context = SELExecutionContext::new();

        let tree = compiler.compile(&String::from("fetch(10)"));

        let results = execute_sel_tree(&tree, &execution_context);

        let first_result = results.get(0).unwrap();

        assert_eq!(first_result.get_type(), DataType::Unit);
        assert_eq!(first_result.get_value(), None);
    }

    #[test]
    fn executes_call_expression() {
        let compiler = Compiler::new();

        let execution_context = SELExecutionContext::new();

        let tree = compiler.compile(&String::from("{ 5 + 10 }\n?()"));

        let results = execute_sel_tree(&tree, &execution_context);

        let result = results.get(1).unwrap();
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 15);
    }

    #[test]
    fn executes_call_expression_with_input() {
        let compiler = Compiler::new();

        let execution_context = SELExecutionContext::new();

        let tree = compiler.compile(&String::from("{ $ + 10 }\n?(5)"));

        let results = execute_sel_tree(&tree, &execution_context);

        let result = results.get(1).unwrap();
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 15);
    }
}
