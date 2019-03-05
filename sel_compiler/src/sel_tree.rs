use super::data_type::DataType;
use super::operation::Operation;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Value {
    data_type: DataType,
}

impl Value {
    pub fn get_data_type(&self) -> DataType {
        return self.data_type;
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct SELTreeNode {
    operation: Operation,
    value: Value,
    own_index: usize,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

impl SELTreeNode {
    pub fn new(op: Operation, data_type: DataType, own_index: usize) -> Self {
        return SELTreeNode {
            operation: op,
            value: Value {
                data_type: data_type,
            },
            // largest operation has two operands
            left: None,
            right: None,
            parent: None,
            own_index: own_index,
        };
    }

    pub fn get_own_index(&self) -> usize {
        return self.own_index;
    }

    pub fn get_operation(&self) -> Operation {
        return self.operation;
    }

    pub fn get_value(&self) -> Value {
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
    root: usize,
    nodes: Vec<SELTreeNode>,
}

impl SELTree {
    pub fn new(root: usize, nodes: Vec<SELTreeNode>) -> SELTree {
        return SELTree {
            root: root,
            nodes: nodes,
        };
    }

    pub fn get_nodes(&self) -> &Vec<SELTreeNode> {
        return &self.nodes;
    }

    pub fn get_root(&self) -> &SELTreeNode {
        return &self.nodes.get(self.root).unwrap();
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NodeSide {
    Left,
    Right,
    Parent,
}

pub fn opposite_of_side(side: NodeSide) -> NodeSide {
    match side {
        NodeSide::Parent => NodeSide::Parent,
        NodeSide::Right => NodeSide::Right,
        NodeSide::Left => NodeSide::Left,
    }
}
