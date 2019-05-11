use super::SELExecutionContext;
use crate::opexec::execution_result::SELExecutionResult;
use crate::opexec::get_node_result;
use sel_common::sel_types::list::List;
use sel_common::{from_byte_vec, to_byte_vec, DataType, Operation, SELTree, SELTreeNode, SELValue};

fn add_value_to_list(value: SELValue, list: &mut List) {
    if value.get_type() == DataType::List {
        let sub_list = value
            .get_value()
            .map_or(List::new(), |value| from_byte_vec(value));

        for item in sub_list.get_values() {
            list.push(item.to_owned());
        }
    } else {
        list.push(value);
    }
}

fn add_if_exists(
    index: Option<usize>,
    tree: &SELTree,
    context: &mut SELExecutionContext,
    list: &mut List,
) {
    let result_info: Option<(SELExecutionResult, bool)> = index
        .and_then(|index| tree.get_nodes().get(index))
        .map(|node| {
            let result: SELExecutionResult = get_node_result(tree, node, context);
            let nested =
                result.get_type() == DataType::List && node.get_operation() == Operation::Group;

            (result, nested)
        });

    if result_info.is_some() {
        let (result, nested) = result_info.unwrap();
        if nested {
            // already checked that value type is list
            // and marked as nested
            // push directly to main list
            list.push(result.get_sel_value().to_owned());
        } else {
            add_value_to_list(result.get_sel_value().to_owned(), list);
        }
    }
}

pub fn operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &mut SELExecutionContext,
) -> SELExecutionResult {
    let mut list = List::new();

    add_if_exists(node.get_left(), tree, context, &mut list);
    add_if_exists(node.get_right(), tree, context, &mut list);

    return SELExecutionResult::new(DataType::List, Some(to_byte_vec(list)));
}

#[cfg(test)]
mod tests {
    use super::super::{get_node_result, SELExecutionContext};
    use sel_common::sel_types::list::List;
    use sel_common::{from_byte_vec, DataType, SELValue};
    use sel_compiler::Compiler;

    #[test]
    fn executes_two_member_list() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("100, true"));
        let mut execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let list: List = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::List);

        let values = list.get_values();

        assert_eq!(values.len(), 2);

        let first_value: &SELValue = values.get(0).unwrap();
        let second_value: &SELValue = values.get(1).unwrap();

        assert_eq!(first_value.get_type(), DataType::Integer);
        assert_eq!(from_byte_vec::<i64>(first_value.get_value().unwrap()), 100);

        assert_eq!(second_value.get_type(), DataType::Boolean);
        assert_eq!(
            from_byte_vec::<bool>(second_value.get_value().unwrap()),
            true
        );
    }

    #[test]
    fn executes_empty_list() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(","));
        let mut execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let list: List = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::List);

        let values = list.get_values();

        assert_eq!(values.len(), 0);
    }

    #[test]
    fn executes_single_item_list_trailing() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("100,"));
        let mut execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let list: List = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::List);

        let values = list.get_values();

        assert_eq!(values.len(), 1);

        let first_value: &SELValue = values.get(0).unwrap();

        assert_eq!(first_value.get_type(), DataType::Integer);
        assert_eq!(from_byte_vec::<i64>(first_value.get_value().unwrap()), 100);
    }

    #[test]
    fn executes_single_item_list_leading() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(",100"));
        let mut execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let list: List = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::List);

        let values = list.get_values();

        assert_eq!(values.len(), 1);

        let first_value: &SELValue = values.get(0).unwrap();

        assert_eq!(first_value.get_type(), DataType::Integer);
        assert_eq!(from_byte_vec::<i64>(first_value.get_value().unwrap()), 100);
    }

    #[test]
    fn executes_five_member_list() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("100, 200, 300, 400, 500"));
        let mut execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let list: List = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::List);

        let values = list.get_values();

        assert_eq!(values.len(), 5);

        let first_value: &SELValue = values.get(0).unwrap();
        let second_value: &SELValue = values.get(1).unwrap();
        let third_value: &SELValue = values.get(2).unwrap();
        let fourth_value: &SELValue = values.get(3).unwrap();
        let fifth_value: &SELValue = values.get(4).unwrap();

        assert_eq!(first_value.get_type(), DataType::Integer);
        assert_eq!(from_byte_vec::<i64>(first_value.get_value().unwrap()), 100);

        assert_eq!(second_value.get_type(), DataType::Integer);
        assert_eq!(from_byte_vec::<i64>(second_value.get_value().unwrap()), 200);

        assert_eq!(third_value.get_type(), DataType::Integer);
        assert_eq!(from_byte_vec::<i64>(third_value.get_value().unwrap()), 300);

        assert_eq!(fourth_value.get_type(), DataType::Integer);
        assert_eq!(from_byte_vec::<i64>(fourth_value.get_value().unwrap()), 400);

        assert_eq!(fifth_value.get_type(), DataType::Integer);
        assert_eq!(from_byte_vec::<i64>(fifth_value.get_value().unwrap()), 500);
    }

    #[test]
    fn executes_nested_list() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("100, 200, (300, 400, 500)"));
        let mut execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let list: List = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::List);

        let values = list.get_values();

        assert_eq!(values.len(), 3);

        let first_value: &SELValue = values.get(0).unwrap();
        let second_value: &SELValue = values.get(1).unwrap();
        let third_value: &SELValue = values.get(2).unwrap();

        let nested_list: List = from_byte_vec(third_value.get_value().unwrap());
        let nested_values = nested_list.get_values();

        let nested_first_value: &SELValue = nested_values.get(0).unwrap();
        let nested_second_value: &SELValue = nested_values.get(1).unwrap();
        let nested_third_value: &SELValue = nested_values.get(2).unwrap();

        assert_eq!(first_value.get_type(), DataType::Integer);
        assert_eq!(from_byte_vec::<i64>(first_value.get_value().unwrap()), 100);

        assert_eq!(second_value.get_type(), DataType::Integer);
        assert_eq!(from_byte_vec::<i64>(second_value.get_value().unwrap()), 200);

        assert_eq!(third_value.get_type(), DataType::List);

        assert_eq!(nested_first_value.get_type(), DataType::Integer);
        assert_eq!(
            from_byte_vec::<i64>(nested_first_value.get_value().unwrap()),
            300
        );

        assert_eq!(nested_second_value.get_type(), DataType::Integer);
        assert_eq!(
            from_byte_vec::<i64>(nested_second_value.get_value().unwrap()),
            400
        );

        assert_eq!(nested_third_value.get_type(), DataType::Integer);
        assert_eq!(
            from_byte_vec::<i64>(nested_third_value.get_value().unwrap()),
            500
        );
    }

    #[test]
    fn executes_multiple_nested_list() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("100, 200, (300, (400, 500), 600)"));
        let mut execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let list: List = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::List);

        let values = list.get_values();

        assert_eq!(values.len(), 3);

        let first_value: &SELValue = values.get(0).unwrap();
        let second_value: &SELValue = values.get(1).unwrap();
        let third_value: &SELValue = values.get(2).unwrap();

        let nested_list: List = from_byte_vec(third_value.get_value().unwrap());
        let nested_values = nested_list.get_values();

        let nested_first_value: &SELValue = nested_values.get(0).unwrap();
        let nested_second_value: &SELValue = nested_values.get(1).unwrap();
        let nested_third_value: &SELValue = nested_values.get(2).unwrap();

        let second_nested_list: List = from_byte_vec(nested_second_value.get_value().unwrap());
        let second_nested_values = second_nested_list.get_values();

        let second_nested_first_value: &SELValue = second_nested_values.get(0).unwrap();
        let second_nested_second_value: &SELValue = second_nested_values.get(1).unwrap();

        assert_eq!(first_value.get_type(), DataType::Integer);
        assert_eq!(from_byte_vec::<i64>(first_value.get_value().unwrap()), 100);

        assert_eq!(second_value.get_type(), DataType::Integer);
        assert_eq!(from_byte_vec::<i64>(second_value.get_value().unwrap()), 200);

        assert_eq!(third_value.get_type(), DataType::List);

        assert_eq!(nested_first_value.get_type(), DataType::Integer);
        assert_eq!(
            from_byte_vec::<i64>(nested_first_value.get_value().unwrap()),
            300
        );

        assert_eq!(nested_second_value.get_type(), DataType::List);

        assert_eq!(second_nested_first_value.get_type(), DataType::Integer);
        assert_eq!(
            from_byte_vec::<i64>(second_nested_first_value.get_value().unwrap()),
            400
        );

        assert_eq!(second_nested_second_value.get_type(), DataType::Integer);
        assert_eq!(
            from_byte_vec::<i64>(second_nested_second_value.get_value().unwrap()),
            500
        );

        assert_eq!(nested_third_value.get_type(), DataType::Integer);
        assert_eq!(
            from_byte_vec::<i64>(nested_third_value.get_value().unwrap()),
            600
        );
    }
}
