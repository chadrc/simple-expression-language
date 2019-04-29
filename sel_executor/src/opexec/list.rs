use super::SELExecutionContext;
use crate::opexec::execution_result::SELExecutionResult;
use crate::opexec::get_node_result;
use crate::opexec::utils::get_left_right_results;
use sel_common::{from_byte_vec, to_byte_vec, DataType, List, SELTree, SELTreeNode, SELValue};

fn add_value_to_list(value: SELValue, list: &mut List) {
    if value.get_type() == DataType::List {
        let sub_list = value
            .get_value()
            .map_or(List::new(), |value| from_byte_vec(value));

        for item in sub_list.get_values() {
            add_value_to_list(item.to_owned(), list);
        }
    } else {
        list.push(value);
    }
}

fn add_if_exists(
    index: Option<usize>,
    tree: &SELTree,
    context: &SELExecutionContext,
    list: &mut List,
) {
    let result: Option<SELExecutionResult> = index
        .and_then(|index| tree.get_nodes().get(index))
        .map(|node| get_node_result(tree, node, context));

    if result.is_some() {
        add_value_to_list(result.unwrap().get_sel_value().to_owned(), list);
    }
}

pub fn operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    let mut list = List::new();

    add_if_exists(node.get_left(), tree, context, &mut list);
    add_if_exists(node.get_right(), tree, context, &mut list);

    return SELExecutionResult::new(DataType::List, Some(to_byte_vec(list)));
}

#[cfg(test)]
mod tests {
    use super::super::{get_node_result, SELExecutionContext};
    use sel_common::{
        from_byte_vec, DataHeap, DataType, List, Operation, SELContext, SELTree, SELTreeNode,
        SELValue, SymbolTable,
    };
    use sel_compiler::Compiler;

    #[test]
    fn executes_two_member_list() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("100, true"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
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
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let list: List = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::List);

        let values = list.get_values();

        assert_eq!(values.len(), 0);
    }

    #[test]
    fn executes_five_member_list() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("100, 200, 300, 400, 500"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
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
}
