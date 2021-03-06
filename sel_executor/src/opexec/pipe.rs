use super::SELExecutionContext;
use crate::opexec::execution_result::SELExecutionResult;
use crate::opexec::get_node_result;
use crate::opexec::utils::get_value_from_result;
use sel_common::sel_types::associative_list::AssociativeList;
use sel_common::sel_types::list::List;
use sel_common::{from_byte_vec, DataType, Operation, SELTree, SELTreeNode, SELValue};

fn pipe_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    pipe_value_index: Option<usize>,
    expression_index: Option<usize>,
    context: &mut SELExecutionContext,
    first: bool,
) -> SELExecutionResult {
    // first, get value of left
    // this is our value to pipe
    pipe_value_index
        .and_then(|left_index| tree.get_nodes().get(left_index))
        .map(|left_node| get_node_result(tree, left_node, context))
        .map(|left_result| {
            let value = left_result.get_sel_value();

            // get right node
            // should be the root of an expression to execute
            expression_index
                .and_then(|right_index| tree.get_nodes().get(right_index))
                .and_then(|right_node| {
                    // make new context with value to pipe as input
                    let mut pipe_context = context.clone();
                    pipe_context.set_input(value.clone());

                    match (right_node.get_operation(), right_node.get_data_type()) {
                        (Operation::Expression, _) => {
                            // value of expression is sub tree index
                            tree.get_usize_value_of(right_node)
                                .and_then(|sub_tree_index| tree.get_sub_trees().get(sub_tree_index))
                                .and_then(|sub_tree| sub_tree.get_roots().get(0))
                                .and_then(|sub_tree_root_index| {
                                    tree.get_nodes().get(*sub_tree_root_index)
                                })
                                .map(|sub_tree_root| {
                                    get_node_result(tree, sub_tree_root, &mut pipe_context)
                                })
                        }
                        (_, DataType::Identifier) => {
                            // identifier will be of an exposed function
                            // first get node value
                            tree.get_usize_value_of(right_node)
                                .and_then(|function_identifier_index| {
                                    tree.get_symbol_table()
                                        .get_symbol(function_identifier_index)
                                })
                                .and_then(|function_symbol| context.get_function(function_symbol))
                                .map(|func| {
                                    SELExecutionResult::from(&func(
                                        value.clone(),
                                        tree.get_symbol_table(),
                                    ))
                                })
                        }
                        (Operation::Group, _) => {
                            // first get value of group
                            right_node
                                .get_right()
                                // need to go one more right
                                // getting result of group node now would try to execute function
                                .and_then(|arg_node_index| tree.get_nodes().get(arg_node_index))
                                .map(|arg_node| get_node_result(tree, arg_node, context))
                                .map(|arg_result| {
                                    // if value is not a list
                                    // make a list
                                    let mut list: List = match arg_result.get_type() {
                                        DataType::List => get_value_from_result(&arg_result),
                                        _ => {
                                            let mut l = List::new();
                                            l.push(arg_result.get_sel_value().clone());

                                            l
                                        }
                                    };

                                    // inject pipe value into list
                                    // if pipe value is a list
                                    // we will extend it with arg value
                                    match value.get_type() {
                                        DataType::List => {
                                            let mut pipe_list: List =
                                                from_byte_vec(value.get_value().unwrap());

                                            if first {
                                                pipe_list.append(&mut list);
                                                list = pipe_list;
                                            } else {
                                                list.append(&mut pipe_list);
                                            }
                                        }
                                        // else we just insert at beginning
                                        _ => {
                                            if first {
                                                list.insert(0, value.clone())
                                            } else {
                                                list.push(value.clone())
                                            }
                                        }
                                    }
                                    println!("list {:?}", list);

                                    let mut func_sel_value = SELValue::new_from_list(list.clone());
                                    println!("value {:?}", func_sel_value);

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

                                    println!("value {:?}", func_sel_value);

                                    // use list as value to group's left side function
                                    right_node
                                        .get_left()
                                        .and_then(|left_index| tree.get_nodes().get(left_index))
                                        .and_then(|left_node| {
                                            tree.get_usize_value_of(left_node)
                                                .and_then(|function_identifier_index| {
                                                    tree.get_symbol_table()
                                                        .get_symbol(function_identifier_index)
                                                })
                                                .and_then(|function_symbol| {
                                                    context.get_function(function_symbol)
                                                })
                                                .map(|func| {
                                                    SELExecutionResult::from(&func(
                                                        func_sel_value,
                                                        tree.get_symbol_table(),
                                                    ))
                                                })
                                        })
                                        .unwrap_or(SELExecutionResult::new(DataType::Unknown, None))
                                })
                        }
                        _ => {
                            // get result of right node
                            Some(get_node_result(tree, right_node, &mut pipe_context))
                        }
                    }
                })
                .unwrap_or(SELExecutionResult::new(DataType::Unknown, None))
        })
        .unwrap_or(SELExecutionResult::new(DataType::Unknown, None))
}

pub fn pipe_first_right_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &mut SELExecutionContext,
) -> SELExecutionResult {
    return pipe_operation(tree, node, node.get_left(), node.get_right(), context, true);
}

pub fn pipe_first_left_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &mut SELExecutionContext,
) -> SELExecutionResult {
    return pipe_operation(tree, node, node.get_right(), node.get_left(), context, true);
}

pub fn pipe_last_right_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &mut SELExecutionContext,
) -> SELExecutionResult {
    return pipe_operation(
        tree,
        node,
        node.get_left(),
        node.get_right(),
        context,
        false,
    );
}

pub fn pipe_last_left_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &mut SELExecutionContext,
) -> SELExecutionResult {
    return pipe_operation(
        tree,
        node,
        node.get_right(),
        node.get_left(),
        context,
        false,
    );
}

#[cfg(test)]
mod tests {
    use crate::opexec::execution_result::SELExecutionResult;
    use crate::opexec::get_node_result;
    use crate::SELExecutionContext;
    use sel_common::sel_types::associative_list::AssociativeList;
    use sel_common::sel_types::list::List;
    use sel_common::{from_byte_vec, to_byte_vec, DataType, SELContext, SELValue};
    use sel_compiler::Compiler;

    #[test]
    fn executes_pipe_first_right_raw_expression() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("10 -> $ * 10"));
        let mut execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 100);
    }

    #[test]
    fn executes_pipe_first_right_nested_expression() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("10 -> { $ * 10 }"));

        let mut execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 100);
    }

    #[test]
    fn executes_pipe_first_right_exposed_function() {
        let compiler = Compiler::new();
        let mut context = SELContext::new();

        context.register_function("is_even", |sel_value, symbol_table| {
            match sel_value.get_type() {
                DataType::Integer => {
                    let value: i64 = from_byte_vec(sel_value.get_value().unwrap());
                    let result = value % 2 == 0;

                    SELValue::new_from_boolean(result)
                }
                _ => SELValue::new(),
            }
        });

        let mut execution_context = SELExecutionContext::from(&context);

        let tree = compiler.compile_with_context(&String::from("10 -> is_even"), context);

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, true);
    }

    #[test]
    fn executes_pipe_first_right_exposed_function_with_arg() {
        let compiler = Compiler::new();
        let mut context = SELContext::new();

        context.register_function("middle", |sel_value, symbol_table| {
            match sel_value.get_type() {
                DataType::List => {
                    let list: List = from_byte_vec(sel_value.get_value().unwrap());

                    let first_value: i64 =
                        from_byte_vec(list.get_values().get(0).unwrap().get_value().unwrap());

                    let second_value: i64 =
                        from_byte_vec(list.get_values().get(1).unwrap().get_value().unwrap());

                    let value: i64 = (second_value - first_value) / 2 + first_value;

                    SELValue::new_from_int(value)
                }
                _ => SELValue::new(),
            }
        });

        let mut execution_context = SELExecutionContext::from(&context);

        let tree = compiler.compile_with_context(&String::from("10 -> middle(20)"), context);

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 15);
    }

    #[test]
    fn executes_pipe_first_right_exposed_function_with_multiple_args() {
        let compiler = Compiler::new();
        let mut context = SELContext::new();

        context.register_function("avg", |sel_value, symbol_table| {
            match sel_value.get_type() {
                DataType::List => {
                    let list: List = from_byte_vec(sel_value.get_value().unwrap());

                    let mut result = 1;

                    let total = list
                        .get_values()
                        .iter()
                        .map(|sel_value| from_byte_vec::<i64>(sel_value.get_value().unwrap()))
                        .fold(0 as i64, |result, i| i + result);

                    let avg = total / list.get_values().len() as i64;

                    SELValue::new_from_int(avg)
                }
                _ => SELValue::new(),
            }
        });

        let mut execution_context = SELExecutionContext::from(&context);

        let tree = compiler.compile_with_context(&String::from("10 -> avg(20, 30)"), context);

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 20);
    }

    #[test]
    fn executes_pipe_first_right_exposed_function_with_multiple_pipe_values() {
        let compiler = Compiler::new();
        let mut context = SELContext::new();

        context.register_function("avg", |sel_value, symbol_table| {
            match sel_value.get_type() {
                DataType::List => {
                    let list: List = from_byte_vec(sel_value.get_value().unwrap());

                    let total = list
                        .get_values()
                        .iter()
                        .map(|sel_value| from_byte_vec::<i64>(sel_value.get_value().unwrap()))
                        .fold(0 as i64, |result, i| i + result);

                    let avg = total / list.get_values().len() as i64;

                    SELValue::new_from_int(avg)
                }
                _ => SELValue::new(),
            }
        });

        let mut execution_context = SELExecutionContext::from(&context);

        let tree = compiler.compile_with_context(&String::from("10, 20 -> avg(30)"), context);

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 20);
    }

    #[test]
    fn executes_pipe_first_right_exposed_function_with_pair_pipe_value() {
        let compiler = Compiler::new();
        let mut context = SELContext::new();

        context.register_function("middle", |sel_value, symbol_table| {
            match sel_value.get_type() {
                DataType::AssociativeList => {
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
                }
                _ => SELValue::new(),
            }
        });

        let mut execution_context = SELExecutionContext::from(&context);

        let tree = compiler
            .compile_with_context(&String::from(":lower = 10 -> middle(:upper = 20)"), context);

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 15);
    }

    #[test]
    fn executes_pipe_last_right_exposed_function_with_arg() {
        let compiler = Compiler::new();
        let mut context = SELContext::new();

        context.register_function("middle", |sel_value, symbol_table| {
            match sel_value.get_type() {
                DataType::List => {
                    let list: List = from_byte_vec(sel_value.get_value().unwrap());

                    let first_value: i64 =
                        from_byte_vec(list.get_values().get(0).unwrap().get_value().unwrap());

                    let second_value: i64 =
                        from_byte_vec(list.get_values().get(1).unwrap().get_value().unwrap());

                    assert_eq!(first_value, 10);
                    assert_eq!(second_value, 20);

                    let value: i64 = (second_value - first_value) / 2 + first_value;

                    SELValue::new_from_int(value)
                }
                _ => SELValue::new(),
            }
        });

        let mut execution_context = SELExecutionContext::from(&context);

        let tree = compiler.compile_with_context(&String::from("20 |> middle(10)"), context);

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 15);
    }

    #[test]
    fn executes_pipe_last_right_exposed_function_with_multiple_pipe_values() {
        let compiler = Compiler::new();
        let mut context = SELContext::new();

        context.register_function("avg", |sel_value, symbol_table| {
            match sel_value.get_type() {
                DataType::List => {
                    let list: List = from_byte_vec(sel_value.get_value().unwrap());

                    let ints: Vec<i64> = list
                        .get_values()
                        .iter()
                        .map(|sel_value| from_byte_vec::<i64>(sel_value.get_value().unwrap()))
                        .collect();

                    assert_eq!(*ints.get(0).unwrap(), 10);
                    assert_eq!(*ints.get(1).unwrap(), 20);
                    assert_eq!(*ints.get(2).unwrap(), 30);

                    let total = ints.iter().fold(0 as i64, |result, i| i + result);

                    let avg = total / list.get_values().len() as i64;

                    SELValue::new_from_int(avg)
                }
                _ => SELValue::new(),
            }
        });

        let mut execution_context = SELExecutionContext::from(&context);

        let tree = compiler.compile_with_context(&String::from("20, 30 |> avg(10)"), context);

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 20);
    }

    #[test]
    fn executes_pipe_first_left_exposed_function_with_multiple_pipe_values() {
        let compiler = Compiler::new();
        let mut context = SELContext::new();

        context.register_function("avg", |sel_value, symbol_table| {
            match sel_value.get_type() {
                DataType::List => {
                    let list: List = from_byte_vec(sel_value.get_value().unwrap());

                    let ints: Vec<i64> = list
                        .get_values()
                        .iter()
                        .map(|sel_value| from_byte_vec::<i64>(sel_value.get_value().unwrap()))
                        .collect();

                    assert_eq!(*ints.get(0).unwrap(), 10);
                    assert_eq!(*ints.get(1).unwrap(), 20);
                    assert_eq!(*ints.get(2).unwrap(), 30);

                    let total = ints.iter().fold(0 as i64, |result, i| i + result);

                    let avg = total / list.get_values().len() as i64;

                    SELValue::new_from_int(avg)
                }
                _ => SELValue::new(),
            }
        });

        let mut execution_context = SELExecutionContext::from(&context);

        let tree = compiler.compile_with_context(&String::from("avg(30) <- 10, 20"), context);

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 20);
    }

    #[test]
    fn executes_pipe_last_left_exposed_function_with_multiple_pipe_values() {
        let compiler = Compiler::new();
        let mut context = SELContext::new();

        context.register_function("avg", |sel_value, symbol_table| {
            match sel_value.get_type() {
                DataType::List => {
                    let list: List = from_byte_vec(sel_value.get_value().unwrap());

                    let ints: Vec<i64> = list
                        .get_values()
                        .iter()
                        .map(|sel_value| from_byte_vec::<i64>(sel_value.get_value().unwrap()))
                        .collect();

                    assert_eq!(*ints.get(0).unwrap(), 10);
                    assert_eq!(*ints.get(1).unwrap(), 20);
                    assert_eq!(*ints.get(2).unwrap(), 30);

                    let total = ints.iter().fold(0 as i64, |result, i| i + result);

                    let avg = total / list.get_values().len() as i64;

                    SELValue::new_from_int(avg)
                }
                _ => SELValue::new(),
            }
        });

        let mut execution_context = SELExecutionContext::from(&context);

        let tree = compiler.compile_with_context(&String::from("avg(10) <| 20, 30"), context);

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 20);
    }
}
