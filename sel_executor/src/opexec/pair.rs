use super::super::context::SELExecutionContext;
use super::execution_result::SELExecutionResult;
use crate::opexec::utils::{get_left_right_results, get_values_from_results};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use sel_common::{DataType, SELTree, SELTreeNode};

fn result_to_bytes(result: &SELExecutionResult) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];
    bytes.write_u8(result.get_type() as u8);

    match result.get_value() {
        Some(value) => {
            bytes.append(&mut value.clone());
        }
        None => (),
    }

    return bytes;
}

pub fn operation(
    tree: &SELTree,
    node: &SELTreeNode,
    context: &SELExecutionContext,
) -> SELExecutionResult {
    let (left_result, right_result) = get_left_right_results(tree, node, context);

    let mut left_data = result_to_bytes(&left_result);
    let mut right_data = result_to_bytes(&right_result);

    left_data.append(&mut right_data);

    return match node.get_data_type() {
        _ => SELExecutionResult::new(DataType::Pair, Some(left_data)),
    };
}

#[cfg(test)]
mod tests {
    use super::super::get_node_result;
    use super::*;
    use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
    use sel_common::{
        from_byte_vec, DataHeap, DataType, Operation, SELContext, SELTree, SELTreeNode, SymbolTable,
    };
    use sel_compiler::Compiler;
    use std::io::Cursor;

    //    #[test]
    //    fn executes_pair() {
    //        let compiler = Compiler::new();
    //        let mut context = SELContext::new();
    //        let tree = compiler.compile_with_context(&String::from(":value = 10"), context);
    //        let execution_context = SELExecutionContext::new();
    //
    //        let result = get_node_result(&tree, tree.get_root(), &execution_context);
    //
    //        assert_eq!(result.get_type(), DataType::Pair);
    //
    //        let (left_type, left_value) =
    //            match Cursor::new(&Vec::from(result.get_value().unwrap().clone())[0..1]).read_u8() {
    //                Ok(result) => {
    //                    let data_type = result;
    //                    let value = from_byte_vec::<usize>(
    //                        &Vec::from(result.get_value().unwrap().clone())[1..9],
    //                    );
    //                    (data_type, value)
    //                }
    //                Err(error) => (DataType::Unknown, None),
    //            };
    //
    //        let (right_type, right_value) =
    //            match Cursor::new(&Vec::from(result.get_value().unwrap().clone())[10..11]).read_u8() {
    //                Ok(result) => {
    //                    let data_type = result;
    //                    let value = from_byte_vec::<i64>(
    //                        &Vec::from(result.get_value().unwrap().clone())[11..19],
    //                    );
    //                    (data_type, value)
    //                }
    //                Err(error) => (DataType::Unknown, None),
    //            };
    //
    //        assert_eq!(left_type, 0);
    //        assert_eq!(left_value, 0);
    //
    //        assert_eq!(right_type, 0);
    //        assert_eq!(right_value, 10);
    //    }
}
