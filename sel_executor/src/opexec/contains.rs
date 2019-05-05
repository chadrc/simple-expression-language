use super::super::context::SELExecutionContext;
use crate::opexec::execution_result::SELExecutionResult;
use crate::opexec::utils::{
    get_left_right_results, get_value_from_result, get_values_from_results, match_equality_ops,
};
use sel_common::sel_types::associative_list::AssociativeList;
use sel_common::sel_types::list::List;
use sel_common::sel_types::pair::Pair;
use sel_common::{from_byte_vec, to_byte_vec, DataType, SELTree, SELTreeNode};

fn contains(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
    invert: bool,
) -> SELExecutionResult {
    let (left_result, right_result) = get_left_right_results(tree, node, context);

    return match left_result.get_type() {
        DataType::AssociativeList => {
            let left_value = get_value_from_result::<AssociativeList>(&left_result);

            let mut contains = false;

            for item in left_value.get_list().get_values() {
                let equal = if item.get_type() == DataType::Pair {
                    let pair: Pair = from_byte_vec(item.get_value().unwrap());
                    pair.get_right().get_value() == right_result.get_value()
                } else {
                    item.get_value() == right_result.get_value()
                };

                if equal {
                    contains = true;
                    break;
                }
            }

            if invert {
                contains = !contains;
            }

            SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(contains)))
        }
        DataType::List => {
            let left_value = get_value_from_result::<List>(&left_result);

            let mut contains = false;

            for item in left_value.get_values() {
                if item.get_value() == right_result.get_value() {
                    contains = true;
                    break;
                }
            }

            if invert {
                contains = !contains;
            }

            SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(contains)))
        }
        _ => SELExecutionResult::new(DataType::Unknown, None),
    };
}

pub fn contains_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return contains(tree, node, context, false);
}

pub fn not_contains_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return contains(tree, node, context, true);
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::result_of_binary_op;
    use crate::opexec::get_node_result;
    use crate::SELExecutionContext;
    use sel_common::{from_byte_vec, DataType, Operation};
    use sel_compiler::Compiler;

    #[test]
    fn executes_associative_list_contains_value_true() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(
            "[:email = \"panda@example.com\", :username = \"panda\"] ~= \"panda\"",
        ));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, true);
    }

    #[test]
    fn executes_associative_list_contains_value_false() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(
            "[:email = \"panda@example.com\", :username = \"panda\"] ~= \"polar\"",
        ));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, false);
    }

    #[test]
    fn executes_associative_list_not_contains_value_false() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(
            "[:email = \"panda@example.com\", :username = \"panda\"] ~!= \"panda\"",
        ));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, false);
    }

    #[test]
    fn executes_associative_list_not_contains_value_true() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(
            "[:email = \"panda@example.com\", :username = \"panda\"] ~!= \"polar\"",
        ));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, true);
    }

    #[test]
    fn executes_list_contains_value_true() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("(10, 20, 30) ~= 20"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, true);
    }

    #[test]
    fn executes_list_contains_value_false() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("(10, 20, 30) ~= 40"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, false);
    }

    #[test]
    fn executes_list_not_contains_value_false() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("(10, 20, 30) ~!= 20"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, false);
    }

    #[test]
    fn executes_list_not_contains_value_true() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("(10, 20, 30) ~!= 40"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, true);
    }
}
