use crate::opexec::execution_result::SELExecutionResult;
use crate::SELExecutionContext;
use sel_common::{DataType, SELTree, SELTreeNode};

pub fn pipe_last_left_operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    return SELExecutionResult::new(DataType::Unknown, None);
}

#[cfg(test)]
mod tests {
    use crate::opexec::get_node_result;
    use crate::{execute_sel_tree, SELExecutionContext};
    use sel_common::sel_types::stream_instruction::StreamInstruction;
    use sel_common::{from_byte_vec, DataType};
    use sel_compiler::Compiler;

    #[test]
    fn executes_stream_of_list() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("10, 20, 30 >>> $"));
        let execution_context = SELExecutionContext::new();

        let results = execute_sel_tree(&tree, &execution_context);

        assert_eq!(results.len(), 4);

        let result_1 = results.get(0).unwrap();
        let result_2 = results.get(1).unwrap();
        let result_3 = results.get(2).unwrap();
        let result_4 = results.get(3).unwrap();

        let value_1: i64 = from_byte_vec(result_1.get_value().unwrap());
        assert_eq!(result_1.get_type(), DataType::Integer);
        assert_eq!(value_1, 10);

        let value_2: i64 = from_byte_vec(result_2.get_value().unwrap());
        assert_eq!(result_2.get_type(), DataType::Integer);
        assert_eq!(value_2, 20);

        let value_3: i64 = from_byte_vec(result_3.get_value().unwrap());
        assert_eq!(result_3.get_type(), DataType::Integer);
        assert_eq!(value_3, 30);

        let value_4: StreamInstruction = from_byte_vec(result_4.get_value().unwrap());
        assert_eq!(result_4.get_type(), DataType::StreamInstruction);
        assert_eq!(value_4, StreamInstruction::Close);
    }
}
