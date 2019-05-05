use super::super::context::SELExecutionContext;
use crate::opexec::execution_result::SELExecutionResult;
use crate::opexec::get_node_result;
use crate::opexec::utils::{
    get_left_right_results, get_value_from_result, get_values_from_results, match_equality_ops,
};
use sel_common::{to_byte_vec, DataType, Operation, SELTree, SELTreeNode};

fn run_match(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
    invert: bool,
) -> bool {
    return node
        .get_left()
        .and_then(|left_index| tree.get_nodes().get(left_index))
        .map(|left_node| get_node_result(tree, left_node, context))
        .map(|left_result| {
            let run = get_value_from_result::<bool>(&left_result);
            if invert {
                !run
            } else {
                run
            }
        })
        .unwrap_or(false);
}

fn match_bool(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
    invert: bool,
) -> SELExecutionResult {
    let run = run_match(tree, node, context, invert);

    let result_opt = if run {
        node.get_right()
            .and_then(|right_index| tree.get_nodes().get(right_index))
            .map(|right_node| get_node_result(tree, right_node, context))
    } else {
        if context.get_results().len() > 0 {
            context
                .get_results()
                .get(context.get_results().len() - 1)
                .map(|result| result.to_owned())
        } else {
            match context.get_input() {
                Some(input) => Some(SELExecutionResult::new(
                    input.get_type(),
                    match input.get_value() {
                        Some(value) => Some(std::vec::Vec::from(value.as_slice())),
                        None => None,
                    },
                )),
                None => None,
            }
        }
    };

    return result_opt.unwrap_or(SELExecutionResult::new(DataType::Unknown, None));
}

pub fn match_true(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return match_bool(tree, node, context, false);
}

pub fn match_false(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return match_bool(tree, node, context, true);
}

pub fn match_list(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    let mut match_stack: Vec<&SELTreeNode> = vec![];
    let mut current_node = node;

    let mut it_count = 0;
    while current_node.get_operation() == Operation::MatchList {
        match current_node
            .get_right()
            .and_then(|right_index| tree.get_nodes().get(right_index))
        {
            Some(right_node) => {
                // push right to stack
                match_stack.push(right_node);
            }
            None => (),
        }

        // set current node to left node
        match current_node
            .get_left()
            .and_then(|left_index| tree.get_nodes().get(left_index))
        {
            Some(left_node) => {
                current_node = left_node;
            }
            None => (),
        }

        // fail safe
        // node count is more than maximum amount this op should take
        // but is only value that will scale with any tree
        it_count += 1;
        if it_count > tree.get_nodes().len() {
            break;
        }
    }

    match_stack.push(current_node);

    let mut final_result: Option<SELExecutionResult> = None;
    // iterate top down
    for node in match_stack.iter().rev() {
        let run = run_match(
            tree,
            node,
            context,
            node.get_operation() == Operation::MatchFalse,
        );

        if run {
            // return left nodes right side
            node.get_right()
                .and_then(|right_index| tree.get_nodes().get(right_index))
                .and_then(|right_node| {
                    // only get a result here if the arm was run
                    // so we can break on first result
                    final_result = Some(get_node_result(tree, right_node, context));

                    Some(true)
                });
        }

        if final_result.is_some() {
            break;
        }
    }

    return final_result.unwrap_or(SELExecutionResult::new(DataType::Unknown, None));
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::result_of_binary_op;
    use crate::opexec::get_node_result;
    use crate::SELExecutionContext;
    use sel_common::{from_byte_vec, DataType, Operation, SELValue};
    use sel_compiler::Compiler;

    #[test]
    fn executes_match_true_right_result() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("true => 100"));

        let mut execution_context = SELExecutionContext::new();
        execution_context.set_input(SELValue::new_from_int(200));

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 100);
    }

    #[test]
    fn executes_match_true_current_result() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("false => 100"));

        let mut execution_context = SELExecutionContext::new();
        execution_context.set_input(SELValue::new_from_int(200));

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 200);
    }

    #[test]
    fn executes_match_false_current_result() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("true !=> 100"));

        let mut execution_context = SELExecutionContext::new();
        execution_context.set_input(SELValue::new_from_int(200));

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 200);
    }

    #[test]
    fn executes_match_false_right_result() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("false !=> 100"));

        let mut execution_context = SELExecutionContext::new();
        execution_context.set_input(SELValue::new_from_int(200));

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: i64 = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(value, 100);
    }

    #[test]
    fn executes_match_list() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("? => \"true 1\", ? => \"true 2\""));

        let mut execution_context = SELExecutionContext::new();
        execution_context.set_input(SELValue::new_from_boolean(true));

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: String = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::String);
        assert_eq!(value, String::from("true 1"));
    }

    #[test]
    fn executes_match_list_three_arms() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(
            "? => \"true 1\", ? !=> \"false\", ? => \"true 2\"",
        ));

        let mut execution_context = SELExecutionContext::new();
        execution_context.set_input(SELValue::new_from_boolean(false));

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: String = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::String);
        assert_eq!(value, String::from("false"));
    }
}
