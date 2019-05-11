use super::execution_result::SELExecutionResult;
use super::{get_node_result, SELExecutionContext};
use sel_common::sel_types::associative_list::AssociativeList;
use sel_common::sel_types::list::List;
use sel_common::{from_byte_vec, to_byte_vec, DataType, SELTree, SELTreeNode, SELValue};

pub fn operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &mut SELExecutionContext,
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
                    | DataType::Pair
                    | DataType::AssociativeList
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
                        let a_list = AssociativeList::from(list);

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
    use sel_common::sel_types::pair::Pair;
    use sel_common::sel_types::symbol::Symbol;

    #[test]
    fn executes_associative_list_from_single_integer_value() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("[100]"));
        let mut execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let list: AssociativeList = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::AssociativeList);

        let values = list.get_list().get_values();

        assert_eq!(values.len(), 1);

        let first_value: SELValue = values.get(0).unwrap().to_owned();

        assert_eq!(first_value.get_type(), DataType::Integer);
        assert_eq!(from_byte_vec::<i64>(first_value.get_value().unwrap()), 100);
    }

    #[test]
    fn executes_associative_list_from_single_symbol_integer_pair_value() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("[:max = 100]"));
        let mut execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let list: AssociativeList = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::AssociativeList);

        let values = list.get_list().get_values();
        let associations = list.get_associations();

        assert_eq!(values.len(), 1);
        assert_eq!(associations.len(), 1);

        let first_value: SELValue = list.get_by_index(0).unwrap().to_owned();

        assert_eq!(first_value.get_type(), DataType::Pair);

        let pair: Pair = from_byte_vec(first_value.get_value().unwrap());
        let symbol: Symbol = from_byte_vec(pair.get_left().get_value().unwrap());
        let pair_value: i64 = from_byte_vec(pair.get_right().get_value().unwrap());

        assert_eq!(symbol.get_table_index(), 0);
        assert_eq!(pair_value, 100);

        let associated_value = list
            .get_by_association_index(symbol.get_table_index())
            .unwrap()
            .to_owned();

        let value: i64 = from_byte_vec(associated_value.get_value().unwrap());

        assert_eq!(value, 100);
    }

    #[test]
    fn executes_associative_list_of_associative_list() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("[[:max = 100]]"));
        println!("{:?}", tree);
        let mut execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let list: AssociativeList = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::AssociativeList);

        let values = list.get_list().get_values();

        assert_eq!(values.len(), 1);

        let first_value: SELValue = list.get_by_index(0).unwrap().to_owned();

        assert_eq!(first_value.get_type(), DataType::AssociativeList);

        let nested_list: AssociativeList = from_byte_vec(first_value.get_value().unwrap());

        let pair_value: SELValue = nested_list.get_by_index(0).unwrap().to_owned();
        let pair: Pair = from_byte_vec(pair_value.get_value().unwrap());

        let symbol_value: &SELValue = pair.get_left();
        let symbol: Symbol = from_byte_vec(symbol_value.get_value().unwrap());

        assert_eq!(symbol.get_identifier(), &String::from("max"));

        let integer_value: &SELValue = pair.get_right();
        let integer: i64 = from_byte_vec(integer_value.get_value().unwrap());

        assert_eq!(integer, 100);

        let associated_value = nested_list
            .get_by_association_index(symbol.get_table_index())
            .unwrap()
            .to_owned();
        let associated_integer: i64 = from_byte_vec(associated_value.get_value().unwrap());

        assert_eq!(associated_integer, 100);
    }

    #[test]
    fn executes_associative_list_from_list() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("[100, true]"));
        let mut execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let list: AssociativeList = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::AssociativeList);

        let values = list.get_list().get_values();

        assert_eq!(values.len(), 2);

        let first_value: SELValue = values.get(0).unwrap().to_owned();
        let second_value: SELValue = values.get(1).unwrap().to_owned();

        assert_eq!(first_value.get_type(), DataType::Integer);
        assert_eq!(from_byte_vec::<i64>(first_value.get_value().unwrap()), 100);

        assert_eq!(second_value.get_type(), DataType::Boolean);
        assert_eq!(
            from_byte_vec::<bool>(second_value.get_value().unwrap()),
            true
        );
    }

    #[test]
    fn executes_associative_list_from_list_of_pairs() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(
            "[:first_name = \"Panda\", :last_name = \"Bear\"]",
        ));
        let mut execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);
        let list: AssociativeList = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::AssociativeList);

        let values = list.get_list().get_values();
        let associations = list.get_associations();

        assert_eq!(values.len(), 2);
        assert_eq!(associations.len(), 2);

        let first_value: SELValue = list.get_by_index(0).unwrap().to_owned();
        let second_value: SELValue = list.get_by_index(1).unwrap().to_owned();

        assert_pair_equal(&list, &first_value, 0, String::from("Panda"));
        assert_pair_equal(&list, &second_value, 1, String::from("Bear"));
    }

    fn assert_pair_equal(
        list: &AssociativeList,
        value: &SELValue,
        symbol_index: usize,
        pair_value: String,
    ) {
        assert_eq!(value.get_type(), DataType::Pair);

        let pair: Pair = from_byte_vec(value.get_value().unwrap());
        let symbol: Symbol = from_byte_vec(pair.get_left().get_value().unwrap());
        let p_value: String = from_byte_vec(pair.get_right().get_value().unwrap());

        assert_eq!(symbol.get_table_index(), symbol_index);
        assert_eq!(p_value, pair_value);

        let associated_value = list
            .get_by_association_index(symbol.get_table_index())
            .unwrap()
            .to_owned();

        let associated_value: String = from_byte_vec(associated_value.get_value().unwrap());

        assert_eq!(associated_value, pair_value);
    }
}
