use super::super::context::SELExecutionContext;
use super::execution_result::SELExecutionResult;
use sel_common::sel_types::symbol::Symbol;
use sel_common::{to_byte_vec, DataType, SELTree, SELTreeNode};

pub fn operation(
    tree: &SELTree,
    node: &SELTreeNode,
    _context: &SELExecutionContext,
) -> SELExecutionResult {
    return match node.get_data_type() {
        DataType::Unit => SELExecutionResult::new(DataType::Unit, None),
        DataType::Identifier => tree
            .get_usize_value_of(node)
            .and_then(|index| tree.get_context().get_value(index))
            .map_or(SELExecutionResult::new(DataType::Unit, None), |value| {
                SELExecutionResult::from(value)
            }),
        DataType::Symbol => {
            let value = tree.get_usize_value_of(node).unwrap();
            let identifier = tree.get_symbol_table().get_symbol(value).unwrap();

            SELExecutionResult::new(
                DataType::Symbol,
                Some(to_byte_vec(Symbol::new(identifier.clone(), value))),
            )
        }
        DataType::Integer | DataType::Decimal | DataType::String | DataType::Boolean => {
            SELExecutionResult::new(node.get_data_type(), tree.get_value_bytes_of(node))
        }
        _ => SELExecutionResult::new(DataType::Unknown, None),
    };
}

#[cfg(test)]
mod tests {
    use super::super::get_node_result;
    use super::*;
    use sel_common::{
        from_byte_vec, DataHeap, DataType, Operation, SELContext, SELTree, SELTreeNode,
    };
    use sel_compiler::Compiler;
    use std::collections::HashMap;

    #[test]
    fn executes_unit_touch() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        nodes.push(SELTreeNode::new(Operation::Touch, DataType::Unit, 0, None));

        let tree = SELTree::new(
            0,
            vec![],
            vec![],
            nodes,
            DataHeap::new(),
            SELContext::new(),
            vec![],
            vec![],
            HashMap::new(),
            HashMap::new(),
        );

        let context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &context);

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }

    #[test]
    fn executes_integer_touch() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let value = heap.insert_from_string(DataType::Integer, &String::from("9"));
        nodes.push(SELTreeNode::new(
            Operation::Touch,
            DataType::Integer,
            0,
            value,
        ));

        let tree = SELTree::new(
            0,
            vec![],
            vec![],
            nodes,
            heap,
            SELContext::new(),
            vec![],
            vec![],
            HashMap::new(),
            HashMap::new(),
        );

        let context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &context);

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(result_value, Some(9));
    }

    #[test]
    fn executes_decimal_touch() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let value = heap.insert_from_string(DataType::Decimal, &String::from("3.14"));
        nodes.push(SELTreeNode::new(
            Operation::Touch,
            DataType::Decimal,
            0,
            value,
        ));

        let tree = SELTree::new(
            0,
            vec![],
            vec![],
            nodes,
            heap,
            SELContext::new(),
            vec![],
            vec![],
            HashMap::new(),
            HashMap::new(),
        );

        let context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &context);

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Decimal);
        assert_eq!(result_value, Some(3.14));
    }

    #[test]
    fn executes_string_touch() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let value = heap.insert_from_string(DataType::String, &String::from("Hello World"));
        nodes.push(SELTreeNode::new(
            Operation::Touch,
            DataType::String,
            0,
            value,
        ));

        let tree = SELTree::new(
            0,
            vec![],
            vec![],
            nodes,
            heap,
            SELContext::new(),
            vec![],
            vec![],
            HashMap::new(),
            HashMap::new(),
        );

        let context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &context);

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::String);
        assert_eq!(result_value, Some(String::from("Hello World")));
    }

    #[test]
    fn executes_boolean_touch() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        let mut heap = DataHeap::new();

        let value = heap.insert_from_string(DataType::Boolean, &String::from("true"));
        nodes.push(SELTreeNode::new(
            Operation::Touch,
            DataType::Boolean,
            0,
            value,
        ));

        let tree = SELTree::new(
            0,
            vec![],
            vec![],
            nodes,
            heap,
            SELContext::new(),
            vec![],
            vec![],
            HashMap::new(),
            HashMap::new(),
        );

        let context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &context);

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(true));
    }

    #[test]
    fn executes_symbol_touch() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from(":value"));
        let context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &context);

        let symbol: Symbol = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Symbol);
        assert_eq!(symbol.get_identifier(), &String::from("value"));
        assert_eq!(symbol.get_table_index(), 0);
    }

    #[test]
    fn executes_identifier_touch() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("value"));
        let context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &context);

        // identifiers with no context value always yield unit
        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }

    #[test]
    fn executes_identifier_touch_with_value() {
        let compiler = Compiler::new();
        let mut context = SELContext::new();
        context.set_integer_symbol(&String::from("value"), 10);

        let tree = compiler.compile_with_context(&String::from("value"), context);
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(result_value, Some(10));
    }
}
