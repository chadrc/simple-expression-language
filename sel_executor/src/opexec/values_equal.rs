use super::super::context::SELExecutionContext;
use crate::opexec::execution_result::SELExecutionResult;
use crate::opexec::utils::{get_left_right_results, get_values_from_results, match_equality_ops};
use sel_common::{to_byte_vec, AssociativeList, DataType, SELTree, SELTreeNode, SELValue};

fn values_equal_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
    invert: bool,
) -> SELExecutionResult {
    let (left_result, right_result) = get_left_right_results(tree, node, context);

    return match (left_result.get_type(), right_result.get_type()) {
        (DataType::List, DataType::List) => {
            let mut equal = left_result.get_value() == right_result.get_value();

            if invert {
                equal = !equal;
            }

            SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(equal)))
        }
        (DataType::AssociativeList, DataType::AssociativeList) => {
            let (left_value, right_value) = get_values_from_results::<
                AssociativeList,
                AssociativeList,
            >(&left_result, &right_result);

            let mut all_equal = true;
            for (global_index, _) in left_value.get_associations() {
                let left_item: &SELValue =
                    &left_value.get_by_association_index(*global_index).unwrap();
                match right_value.get_by_association_index(*global_index) {
                    Some(right_item) => {
                        let equal = if left_item.get_value().is_none()
                            && right_item.get_value().is_none()
                        {
                            true
                        } else {
                            left_item
                                .get_value()
                                .and_then(|left_item_val| {
                                    right_item
                                        .get_value()
                                        .map(|right_item_value| left_item_val == right_item_value)
                                })
                                .unwrap_or(false)
                        };

                        if !equal {
                            all_equal = false;
                            break;
                        }
                    }
                    // no key in right means they are not equal
                    None => {
                        all_equal = false;
                        break;
                    }
                }
            }

            if invert {
                all_equal = !all_equal;
            }

            SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(all_equal)))
        }
        _ => SELExecutionResult::new(DataType::Unknown, None),
    };
}

pub fn equal_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return values_equal_operation(tree, node, context, false);
}

pub fn not_equal_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return values_equal_operation(tree, node, context, true);
}

#[cfg(test)]
mod tests {
    use super::super::test_utils::result_of_binary_op;
    use crate::opexec::get_node_result;
    use crate::SELExecutionContext;
    use sel_common::{from_byte_vec, to_byte_vec, DataType, Operation};
    use sel_compiler::Compiler;

    #[test]
    fn executes_associative_list_values_equal_true() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(
            "[:email = \"panda@example.com\", :username = \"panda\"] $= [:email = \"panda@example.com\", :username = \"panda\"]",
        ));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, true);
    }

    #[test]
    fn executes_associative_list_values_equal_false() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(
            "[:email = \"panda@example.com\", :username = \"panda\"] $= [:email = \"polar@example.com\", :username = \"polar\"]",
        ));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, false);
    }

    #[test]
    fn executes_associative_list_values_not_equal_false() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(
            "[:email = \"panda@example.com\", :username = \"panda\"] $!= [:email = \"panda@example.com\", :username = \"panda\"]",
        ));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, false);
    }

    #[test]
    fn executes_associative_list_values_not_equal_true() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(
            "[:email = \"panda@example.com\", :username = \"panda\"] $!= [:email = \"polar@example.com\", :username = \"polar\"]",
        ));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, true);
    }

    #[test]
    fn executes_list_values_equal_true() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("(10, 20, 30) $= (10, 20, 30)"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, true);
    }

    #[test]
    fn executes_list_values_equal_false() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("(10, 30, 40) $= (10, 20, 30)"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, false);
    }

    #[test]
    fn executes_list_values_not_equal_false() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("(10, 20, 30) $!= (10, 20, 30)"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, false);
    }

    #[test]
    fn executes_list_values_not_equal_true() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("(10, 30, 40) $!= (10, 20, 30)"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let value: bool = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(value, true);
    }
}
