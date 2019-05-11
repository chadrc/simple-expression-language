use crate::opexec::execution_result::SELExecutionResult;
use crate::opexec::get_node_result;
use crate::SELExecutionContext;
use sel_common::sel_types::list::List;
use sel_common::sel_types::stream::SELStream;
use sel_common::sel_types::stream_instruction::StreamInstruction;
use sel_common::{from_byte_vec, to_byte_vec, DataType, SELTree, SELTreeNode};

pub fn operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &mut SELExecutionContext,
) -> SELExecutionResult {
    // get left result
    // this will contain our streamable value
    node.get_left()
        .and_then(|left_index| tree.get_nodes().get(left_index))
        .map(|left_node| get_node_result(tree, left_node, context))
        .and_then(|left_result| {
            // now get right side node
            node.get_right().map(|right_index| {
                let stream = SELStream::new(left_result.get_sel_value().to_owned(), right_index);

                SELExecutionResult::new(DataType::Stream, Some(to_byte_vec(stream)))
            })
        })
        .unwrap_or(SELExecutionResult::new(DataType::Unknown, None))
}

#[cfg(test)]
mod tests {
    use crate::opexec::get_node_result;
    use crate::{execute_sel_tree, SELExecutionContext};
    use sel_common::sel_types::stream::SELStream;
    use sel_common::sel_types::stream_instruction::StreamInstruction;
    use sel_common::{from_byte_vec, DataType};
    use sel_compiler::Compiler;

    #[test]
    fn executes_stream_of_list() {
        let compiler = Compiler::new();
        let tree = compiler.compile(&String::from("10, 20, 30 >>> $"));
        let mut execution_context = SELExecutionContext::new();

        let result = get_node_result(&tree, tree.get_root(), &mut execution_context);

        assert_eq!(result.get_type(), DataType::Stream);

        let stream: SELStream = from_byte_vec(result.get_value().unwrap());

        assert_eq!(stream.get_processor_root(), 7);

        let mut stream_iter = stream.iter();

        let result_1 = stream_iter.next().unwrap();
        let result_2 = stream_iter.next().unwrap();
        let result_3 = stream_iter.next().unwrap();
        let result_4 = stream_iter.next();

        let value_1: i64 = from_byte_vec(result_1.get_value().unwrap());
        assert_eq!(result_1.get_type(), DataType::Integer);
        assert_eq!(value_1, 10);

        let value_2: i64 = from_byte_vec(result_2.get_value().unwrap());
        assert_eq!(result_2.get_type(), DataType::Integer);
        assert_eq!(value_2, 20);

        let value_3: i64 = from_byte_vec(result_3.get_value().unwrap());
        assert_eq!(result_3.get_type(), DataType::Integer);
        assert_eq!(value_3, 30);

        assert!(result_4.is_none());
    }
}
