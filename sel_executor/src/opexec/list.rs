use super::SELExecutionContext;
use crate::opexec::execution_result::SELExecutionResult;
use crate::opexec::utils::get_left_right_results;
use sel_common::{to_byte_vec, DataType, List, SELTree, SELTreeNode};

fn add_result_to_list(result: SELExecutionResult, list: &mut List) {
    list.push(result.get_sel_value().to_owned());
}

pub fn operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    let (left_result, right_result) = get_left_right_results(tree, node, context);
    let mut list = List::new();

    add_result_to_list(left_result, &mut list);
    add_result_to_list(right_result, &mut list);

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
}
