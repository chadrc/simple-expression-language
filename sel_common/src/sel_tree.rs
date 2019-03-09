use super::data_type::DataType;
use super::operation::Operation;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

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
            value: value,
            left: None,
            right: None,
            parent: None,
            data_type: data_type,
            own_index: own_index,
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
    data: Vec<Vec<u8>>,
    root: usize,
    nodes: Vec<SELTreeNode>,
}

impl SELTree {
    pub fn new(root: usize, nodes: Vec<SELTreeNode>, data: Vec<Vec<u8>>) -> SELTree {
        return SELTree {
            root: root,
            nodes: nodes,
            data: data,
        };
    }

    pub fn get_nodes(&self) -> &Vec<SELTreeNode> {
        return &self.nodes;
    }

    pub fn get_root(&self) -> &SELTreeNode {
        return &self.nodes.get(self.root).unwrap();
    }

    pub fn get_integer_value_of(&self, node: &SELTreeNode) -> Option<i64> {
        return match node.value {
            Some(value_index) => match self.data.get(value_index) {
                Some(datum) => match Cursor::new(datum).read_i64::<LittleEndian>() {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                None => None,
            },
            None => None,
        };
    }

    pub fn get_decimal_value_of(&self, node: &SELTreeNode) -> Option<f64> {
        return match node.value {
            Some(value_index) => match self.data.get(value_index) {
                Some(datum) => match Cursor::new(datum).read_f64::<LittleEndian>() {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                None => None,
            },
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
