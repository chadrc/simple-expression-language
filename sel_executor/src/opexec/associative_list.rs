use super::execution_result::SELExecutionResult;
use super::{get_node_result, SELExecutionContext};
use sel_common::{
    from_byte_vec, to_byte_vec, AssociativeList, DataType, List, SELTree, SELTreeNode, SELValue,
};

pub fn operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    node.get_right()
        .and_then(|right_index| tree.get_nodes().get(right_index))
        .map(|right_node| get_node_result(tree, right_node, context))
        .and_then(|result: SELExecutionResult| {
            result
                .get_value()
                .and_then(|value| match result.get_type() {
                    DataType::String
                    | DataType::Integer
                    | DataType::Decimal
                    | DataType::Boolean
                    | DataType::Symbol => {
                        let mut a_list = AssociativeList::new();
                        a_list.push(result.get_sel_value().to_owned());

                        Some(SELExecutionResult::new(
                            DataType::AssociativeList,
                            Some(to_byte_vec(a_list)),
                        ))
                    }
                    DataType::List => {
                        let list: List = from_byte_vec(value);
                        let mut a_list = AssociativeList::from(list);

                        Some(SELExecutionResult::new(
                            DataType::AssociativeList,
                            Some(to_byte_vec(a_list)),
                        ))
                    }
                    _ => Some(SELExecutionResult::new(DataType::Unknown, None)),
                })
        })
        .unwrap_or(SELExecutionResult::new(DataType::Unknown, None))
}

#[cfg(test)]
mod tests {
    use sel_common::{from_byte_vec, DataType, SELContext, SELValue};
    use sel_compiler::Compiler;

    use super::super::super::execute_sel_tree;
    use super::*;

    #[test]
    fn executes_associative_list_from_single_integer_value() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("[100]"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let list: AssociativeList = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::AssociativeList);

        let values = list.get_list().get_values();

        assert_eq!(values.len(), 1);

        let first_value: &SELValue = values.get(0).unwrap();

        assert_eq!(first_value.get_type(), DataType::Integer);
        assert_eq!(from_byte_vec::<i64>(first_value.get_value().unwrap()), 100);
    }

    #[test]
    fn executes_associative_list_from_list() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("[100, true]"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let list: AssociativeList = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::AssociativeList);

        let values = list.get_list().get_values();

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
