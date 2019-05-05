use super::super::context::SELExecutionContext;
use crate::opexec::execution_result::SELExecutionResult;
use crate::opexec::utils::{get_left_right_results, get_values_from_results, match_equality_ops};
use sel_common::sel_types::associative_list::AssociativeList;
use sel_common::{to_byte_vec, DataType, SELTree, SELTreeNode};

fn keys_equal_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
    invert: bool,
) -> SELExecutionResult {
    let (left_result, right_result) = get_left_right_results(tree, node, context);

    return match (left_result.get_type(), right_result.get_type()) {
        (DataType::AssociativeList, DataType::AssociativeList) => {
            let (left_value, right_value) = get_values_from_results::<
                AssociativeList,
                AssociativeList,
            >(&left_result, &right_result);

            let mut contains_all = true;
            for (global_index, _) in left_value.get_associations() {
                if !right_value.get_associations().contains_key(global_index) {
                    contains_all = false;
                }
            }

            if invert {
                contains_all = !contains_all;
            }

            SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(contains_all)))
        }
        _ => SELExecutionResult::new(DataType::Unknown, None),
    };
}

pub fn equal_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return keys_equal_operation(tree, node, context, false);
}

pub fn not_equal_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return keys_equal_operation(tree, node, context, true);
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::result_of_binary_op;
    use crate::opexec::get_node_result;
    use crate::SELExecutionContext;
    use sel_common::{from_byte_vec, DataType, Operation};
    use sel_compiler::Compiler;

    #[test]
    fn executes_keys_equal_true() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(
            "[:email = \"panda@example.com\", :username = \"panda\"] := [:email = \"polar@example.com\", :username = \"polar\"]",
        ));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, true);
    }

    #[test]
    fn executes_keys_equal_false() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(
            "[:email = \"panda@example.com\", :password=\"secret\", :username = \"panda\"] := [:email = \"polar@example.com\", :username = \"polar\"]",
        ));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, false);
    }

    #[test]
    fn executes_keys_not_equal_false() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(
            "[:email = \"panda@example.com\", :username = \"panda\"] :!= [:email = \"polar@example.com\", :username = \"polar\"]",
        ));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, false);
    }

    #[test]
    fn executes_keys_not_equal_true() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(
            "[:email = \"panda@example.com\", :password=\"secret\", :username = \"panda\"] :!= [:email = \"polar@example.com\", :username = \"polar\"]",
        ));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, true);
    }
}
