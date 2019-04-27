use super::data_type::DataType;
use super::operation::Operation;
use super::DataHeap;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct SELTreeNode {
    operation: Operation,
    value: Option<usize>,
    data_type: DataType,
    own_index: usize,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

impl SELTreeNode {
    pub fn new(op: Operation, data_type: DataType, own_index: usize, value: Option<usize>) -> Self {
        return SELTreeNode {
            operation: op,
            value,
            left: None,
            right: None,
            parent: None,
            data_type,
            own_index,
        };
    }

    pub fn get_data_type(&self) -> DataType {
        return self.data_type;
    }

    pub fn get_own_index(&self) -> usize {
        return self.own_index;
    }

    pub fn get_operation(&self) -> Operation {
        return self.operation;
    }

    pub fn get_value(&self) -> Option<usize> {
        return self.value;
    }

    pub fn get_left(&self) -> Option<usize> {
        return self.left;
    }

    pub fn get_right(&self) -> Option<usize> {
        return self.right;
    }

    pub fn get_parent(&self) -> Option<usize> {
        return self.parent;
    }

    pub fn set_left(&mut self, left: Option<usize>) {
        self.left = left;
    }

    pub fn set_right(&mut self, right: Option<usize>) {
        self.right = right;
    }

    pub fn set_parent(&mut self, parent: Option<usize>) {
        self.parent = parent;
    }
}

#[derive(Debug, Clone)]
pub struct SELTree {
    data: DataHeap,
    root: usize,
    sub_roots: Vec<usize>,
    nodes: Vec<SELTreeNode>,
}

impl SELTree {
    pub fn new(
        root: usize,
        sub_roots: Vec<usize>,
        nodes: Vec<SELTreeNode>,
        data: DataHeap,
    ) -> SELTree {
        return SELTree {
            root,
            sub_roots,
            nodes,
            data,
        };
    }

    pub fn get_nodes(&self) -> &Vec<SELTreeNode> {
        return &self.nodes;
    }

    pub fn get_sub_roots(&self) -> &Vec<usize> {
        return &self.sub_roots;
    }

    pub fn get_root(&self) -> &SELTreeNode {
        return &self.nodes.get(self.root).unwrap();
    }

    pub fn get_sub_root(&self, index: usize) -> Option<&SELTreeNode> {
        return match self.sub_roots.get(index) {
            Some(sub_root) => self.nodes.get(*sub_root),
            None => None,
        };
    }

    pub fn get_value_bytes_of(&self, node: &SELTreeNode) -> Option<Vec<u8>> {
        return match node.get_value() {
            Some(value_index) => self.data.get_bytes(value_index),
            None => None,
        };
    }

    pub fn get_integer_value_of(&self, node: &SELTreeNode) -> Option<i64> {
        return match node.get_value() {
            Some(value_index) => self.data.get_integer(value_index),
            None => None,
        };
    }

    pub fn get_decimal_value_of(&self, node: &SELTreeNode) -> Option<f64> {
        return match node.get_value() {
            Some(value_index) => self.data.get_decimal(value_index),
            None => None,
        };
    }

    pub fn get_string_value_of(&self, node: &SELTreeNode) -> Option<String> {
        return match node.get_value() {
            Some(value_index) => self.data.get_string(value_index),
            None => None,
        };
    }

    pub fn get_boolean_value_of(&self, node: &SELTreeNode) -> Option<bool> {
        return match node.get_value() {
            Some(value_index) => self.data.get_boolean(value_index),
            None => None,
        };
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NodeSide {
    Left,
    Right,
    Parent,
}
