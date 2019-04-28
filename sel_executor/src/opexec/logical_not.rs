use super::super::context::SELExecutionContext;
use super::utils::get_value_from_result;
use super::{get_node_result, SELExecutionResult};
use sel_common::{to_byte_vec, DataType, SELTree, SELTreeNode};

pub fn operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    let right = tree.get_nodes().get(node.get_right().unwrap()).unwrap();
    let result = get_node_result(tree, &right, context);

    return match result.get_type() {
        DataType::Boolean => {
            let right_val: bool = get_value_from_result(&result);

            let val = !right_val;

            SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(val)))
        }
        // Unit value is considered logically false
        // so negating it will always result in true
        DataType::Unit => SELExecutionResult::new(DataType::Boolean, Some(to_byte_vec(true))),
        _ => SELExecutionResult::new(DataType::Unknown, None),
    };
}

#[cfg(test)]
mod tests {
    use super::super::super::context;
    use super::super::get_node_result;
    use super::super::test_utils::result_of_binary_op;
    use sel_common::{
        from_byte_vec, DataHeap, DataType, Operation, SELContext, SELTree, SELTreeNode, SymbolTable,
    };

    #[test]
    fn executes_boolean() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let mut right = SELTreeNode::new(
            Operation::Touch,
            DataType::Boolean,
            0,
            heap.insert_from_string(DataType::Boolean, &String::from("true")),
        );

        let mut root = SELTreeNode::new(Operation::LogicalNot, DataType::Unknown, 1, None);

        right.set_parent(Some(1));

        root.set_left(None);
        root.set_right(Some(0));

        nodes.push(right);
        nodes.push(root);

        let tree = SELTree::new(1, vec![], nodes, heap, SELContext::new());

        let context = context::SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &context);

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(false));
    }

    #[test]
    fn executes_unit() {
        let result = result_of_binary_op(
            Operation::LogicalNot,
            DataType::Unknown,
            "",
            DataType::Unit,
            "()",
        );

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(true));
    }
}
