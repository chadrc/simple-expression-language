use super::{SELExecutionContext, SELExecutionResult};
use sel_common::{DataType, SELTree, SELTreeNode};

pub fn operation(
    _tree: &SELTree,
    _node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return if context.get_results().len() > 0 {
        // if there is are results in context
        // clone the latest one and use it as the result
        let last = context.get_results().len() - 1;
        context.get_results().get(last).unwrap().clone()
    } else {
        // else use input as the result
        match context.get_input() {
            Some(input) => SELExecutionResult::new(
                input.get_type(),
                match input.get_value() {
                    Some(value) => Some(std::vec::Vec::from(value.as_slice())),
                    None => None,
                },
            ),
            None => SELExecutionResult::new(DataType::Unit, None),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::super::super::context::SELValue;
    use super::super::{get_node_result, SELExecutionContext};
    use sel_common::{from_byte_vec, DataType};
    use sel_compiler::Compiler;

    #[test]
    fn executes_input() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("?"));

        let mut context = SELExecutionContext::new();

        let input = SELValue::new_from_int(100);

        context.set_input(input);

        let result = get_node_result(&tree, tree.get_root(), &context);

        let result_value = match result.get_value() {
            Some(value) => Some(from_byte_vec(value)),
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Integer);
        assert_eq!(result_value, Some(100));
    }

    #[test]
    fn executes_empty_input() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("?"));

        let context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &context);

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }
}
