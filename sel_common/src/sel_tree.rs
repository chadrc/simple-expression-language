use super::data_type::DataType;
use super::operation::Operation;
use super::DataHeap;
use crate::annotation::Annotation;
use crate::annotation_document::AnnotationDocument;
use crate::named_expression::NamedExpression;
use crate::symbol_table::SymbolTable;
use crate::{SELContext, SELSubTree};
use std::collections::HashMap;

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
    pub fn new(
        operation: Operation,
        data_type: DataType,
        own_index: usize,
        value: Option<usize>,
    ) -> Self {
        return SELTreeNode {
            operation,
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

    pub fn set_value(&mut self, value: Option<usize>) {
        self.value = value;
    }

    pub fn set_operation(&mut self, op: Operation) {
        self.operation = op;
    }
}

#[derive(Debug)]
pub struct SELTree {
    data: DataHeap,
    root: usize,
    sub_trees: Vec<SELSubTree>,
    sub_roots: Vec<usize>,
    nodes: Vec<SELTreeNode>,
    context: SELContext,
    annotations: Vec<Annotation>,
    documents: Vec<AnnotationDocument>,
    named_expressions: HashMap<usize, NamedExpression>,
    namespaces: HashMap<usize, Vec<String>>,
}

impl SELTree {
    pub fn new(
        root: usize,
        sub_trees: Vec<SELSubTree>,
        sub_roots: Vec<usize>,
        nodes: Vec<SELTreeNode>,
        data: DataHeap,
        context: SELContext,
        annotations: Vec<Annotation>,
        documents: Vec<AnnotationDocument>,
        named_expressions: HashMap<usize, NamedExpression>,
        name_spaces: HashMap<usize, Vec<String>>,
    ) -> SELTree {
        return SELTree {
            root,
            sub_trees,
            sub_roots,
            nodes,
            data,
            context,
            annotations,
            documents,
            named_expressions,
            namespaces: name_spaces,
        };
    }

    pub fn get_nodes(&self) -> &Vec<SELTreeNode> {
        return &self.nodes;
    }

    pub fn get_sub_trees(&self) -> &Vec<SELSubTree> {
        return &self.sub_trees;
    }

    pub fn get_sub_roots(&self) -> &Vec<usize> {
        return &self.sub_roots;
    }

    pub fn get_root(&self) -> &SELTreeNode {
        return &self.nodes.get(self.root).unwrap();
    }

    pub fn get_context(&self) -> &SELContext {
        return &self.context;
    }

    pub fn get_symbol_table(&self) -> &SymbolTable {
        return &self.context.get_symbol_table();
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

    pub fn get_usize_value_of(&self, node: &SELTreeNode) -> Option<usize> {
        return node
            .get_value()
            .and_then(|index| self.data.get_usize(index));
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

    pub fn get_documents(&self) -> &Vec<AnnotationDocument> {
        return &self.documents;
    }

    pub fn get_annotations(&self) -> &Vec<Annotation> {
        return &self.annotations;
    }

    pub fn get_named_expressions(&self) -> &HashMap<usize, NamedExpression> {
        return &self.named_expressions;
    }

    pub fn get_namespaces_for_symbol(&self) -> &HashMap<usize, Vec<String>> {
        return &self.namespaces;
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NodeSide {
    Left,
    Right,
    Parent,
}
