use sel_common::{DataType, Operation, SELTree};

pub struct SELExecutionResult {
    data_type: DataType,
    value: Option<Vec<u8>>,
}

impl SELExecutionResult {
    pub fn get_type(&self) -> DataType {
        return self.data_type;
    }

    pub fn get_value(&self) -> Option<&Vec<u8>> {
        return match &self.value {
            Some(v) => Some(&v),
            None => None,
        };
    }
}

pub fn execute_sel_tree(tree: SELTree) -> SELExecutionResult {
    if tree.get_nodes().len() > 0 {
        let root = tree.get_root();

        if root.get_operation() == Operation::Touch {
            return match root.get_data_type() {
                DataType::Unit => SELExecutionResult {
                    data_type: DataType::Unit,
                    value: None,
                },
                DataType::Integer | DataType::Decimal | DataType::String | DataType::Boolean => {
                    SELExecutionResult {
                        data_type: root.get_data_type(),
                        value: tree.get_value_bytes_of(root),
                    }
                }
                _ => SELExecutionResult {
                    data_type: DataType::Unknown,
                    value: None,
                },
            };
        }
    }

    return SELExecutionResult {
        data_type: DataType::Unknown,
        value: None,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::{LittleEndian, ReadBytesExt};
    use sel_common::{DataHeap, DataType, Operation, SELTree, SELTreeNode};
    use std::io::Cursor;

    #[test]
    fn executes_empty() {
        let tree = SELTree::new(0, vec![], DataHeap::new());

        let result = execute_sel_tree(tree);

        assert_eq!(result.get_type(), DataType::Unknown);
        assert_eq!(result.get_value(), None);
    }

    #[test]
    fn executes_unit_touch() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        nodes.push(SELTreeNode::new(Operation::Touch, DataType::Unit, 0, None));

        let tree = SELTree::new(0, nodes, DataHeap::new());

        let result = execute_sel_tree(tree);

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

        let tree = SELTree::new(0, nodes, heap);

        let result = execute_sel_tree(tree);

        let result_value = match result.get_value() {
            Some(value) => match Cursor::new(value).read_i64::<LittleEndian>() {
                Ok(val) => Some(val),
                Err(_) => None,
            },
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

        let tree = SELTree::new(0, nodes, heap);

        let result = execute_sel_tree(tree);

        let result_value = match result.get_value() {
            Some(value) => match Cursor::new(value).read_f64::<LittleEndian>() {
                Ok(val) => Some(val),
                Err(_) => None,
            },
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

        let tree = SELTree::new(0, nodes, heap);

        let result = execute_sel_tree(tree);

        let result_value = match result.get_value() {
            Some(value) => {
                let cow = String::from_utf8_lossy(value);
                Some(cow.to_owned().to_string())
            }
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

        let tree = SELTree::new(0, nodes, heap);

        let result = execute_sel_tree(tree);

        let result_value = match result.get_value() {
            Some(value) => match value.get(0) {
                Some(num) => match num {
                    0 => Some(false),
                    1 => Some(true),
                    _ => None,
                },
                None => None,
            },
            None => None,
        };

        assert_eq!(result.get_type(), DataType::Boolean);
        assert_eq!(result_value, Some(true));
    }
}
