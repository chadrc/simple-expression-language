use sel_common::{DataType, Operation, SELTree};

pub struct SELExecutionResult {
    data_type: DataType,
    value: Option<usize>,
}

impl SELExecutionResult {
    pub fn get_type(&self) -> DataType {
        return self.data_type;
    }

    pub fn get_value(&self) -> Option<usize> {
        return self.value;
    }
}

pub fn execute_sel_tree(tree: SELTree) -> SELExecutionResult {
    if tree.get_nodes().len() > 0 {
        let root = tree.get_root();

        if root.get_operation() == Operation::Touch {
            if root.get_value().get_data_type() == DataType::Unit {
                return SELExecutionResult {
                    data_type: DataType::Unit,
                    value: None,
                };
            }
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
    use sel_common::{DataType, Operation, SELTree, SELTreeNode};

    #[test]
    fn executes_empty() {
        let tree = SELTree::new(0, vec![]);

        let result = execute_sel_tree(tree);

        assert_eq!(result.get_type(), DataType::Unknown);
        assert_eq!(result.get_value(), None);
    }

    #[test]
    fn executes_unit_touch() {
        let mut nodes: Vec<SELTreeNode> = vec![];
        nodes.push(SELTreeNode::new(Operation::Touch, DataType::Unit, 0));

        let tree = SELTree::new(0, nodes);

        let result = execute_sel_tree(tree);

        assert_eq!(result.get_type(), DataType::Unit);
        assert_eq!(result.get_value(), None);
    }
}
