use super::SELExecutionContext;
use crate::opexec::execution_result::SELExecutionResult;
use crate::opexec::get_node_result;
use sel_common::{
    from_byte_vec, to_byte_vec, DataType, Expression, List, Operation, SELTree, SELTreeNode,
    SELValue,
};

pub fn operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    node.get_right()
        // right index of expression operation is the root of the expression
        .map(|right_index| {
            let expr = Expression::new(Some(right_index));
            SELExecutionResult::new(DataType::Expression, Some(to_byte_vec(expr)))
        })
        .unwrap_or(SELExecutionResult::new(
            DataType::Expression,
            Some(to_byte_vec(Expression::new(None))),
        ))
}

#[cfg(test)]
mod tests {
    use super::super::{get_node_result, SELExecutionContext};
    use sel_common::{from_byte_vec, DataType, Expression, List, SELValue};
    use sel_compiler::Compiler;

    #[test]
    fn executes_expression_declaration() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("{ 10 + 5 }"));
        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let result_value: Expression = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Expression);
        assert_eq!(result_value.get_root(), Some(2));
    }

    #[test]
    fn executes_empty_expression_declaration() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("{ }"));

        let execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &execution_context);
        let result_value: Expression = from_byte_vec(result.get_value().unwrap());

        assert_eq!(result.get_type(), DataType::Expression);
        assert_eq!(result_value.get_root(), None);
    }
}
