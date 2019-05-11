use crate::precedence_manager::PrecedenceManager;
use crate::utils::{get_data_type_for_token, get_operation_type_for_token};
use sel_common::annotation::Annotation;
use sel_common::annotation_document::AnnotationDocument;
use sel_common::named_expression::NamedExpression;
use sel_common::{DataHeap, DataType, Operation, SELContext, SELTreeNode};
use sel_tokenizer::{TokenType, Tokenizer};
use std::collections::HashMap;

const TERMINABLE_OPS: [Operation; 6] = [
    Operation::Touch,
    Operation::Input,
    Operation::CurrentResult,
    Operation::Group,
    Operation::Expression,
    Operation::AssociativeList,
];

fn op_is_terminable(op: Operation) -> bool {
    return TERMINABLE_OPS.contains(&op);
}

pub fn make_nodes_from_tokenizer(
    precedence_manager: &mut PrecedenceManager,
    tokenizer: &mut Tokenizer,
    context: &mut SELContext,
) -> (
    Vec<SELTreeNode>,
    DataHeap,
    Vec<usize>,
    Vec<Annotation>,
    Vec<AnnotationDocument>,
    Vec<NamedExpression>,
    HashMap<usize, Vec<String>>,
) {
    let mut nodes: Vec<SELTreeNode> = vec![];
    let mut data = DataHeap::new();
    let mut firsts_of_expression: Vec<usize> = vec![];
    let mut annotations: Vec<Annotation> = vec![];
    let mut documents: Vec<AnnotationDocument> = vec![];
    let mut named_expressions: Vec<NamedExpression> = vec![];
    let mut identifier_namespaces: HashMap<usize, Vec<String>> = HashMap::new();

    let mut current_document: AnnotationDocument = AnnotationDocument::new();
    let mut last_data_type = DataType::Unknown;
    let mut last_op = Operation::None;
    let mut link_next = true;
    let mut symbol_next = false;
    let mut empty_group = false;
    let mut in_document = false;
    let mut infix_next = false;
    let mut infix_last = false;
    let mut current_identifier: Vec<String> = vec![];

    // loop trough all tokens
    // convert them to tree nodes
    // and link them together
    for token in tokenizer {
        let inserted_index = nodes.len();
        let previous_index = if nodes.len() > 0 {
            inserted_index - 1
        } else {
            0
        };

        if token.get_token_type() == TokenType::CommentAnnotation {
            // drop
            continue;
        } else if token.get_token_type() == TokenType::Annotation {
            let name = String::from(token.get_token_str()[1..].trim());
            annotations.push(Annotation::new(name));
            continue;
        } else if token.get_token_type() == TokenType::DocumentAnnotation {
            // slice out the line without the leading '@@'
            let line = String::from(token.get_token_str()[2..].trim());
            current_document.add_line(line);
            in_document = true;
            continue;
        } else if token.get_token_type() == TokenType::LineEnd {
            // set next token to not be linked to previous
            // skip this token, no need to convert LineEnd to a node
            link_next = false;
            continue;
        } else if token.get_token_type() == TokenType::BackTick {
            if infix_last {
                // flag to continue normal parsing
                infix_last = false;
            } else {
                // flag to say next identifier is infix
                infix_next = true;
            }
            continue;
        } else if token.get_token_type() == TokenType::TaggedIdentifier {
            // slice away the leading '#'
            let name = String::from(token.get_token_str()[1..].as_ref());
            let symbol_index = context.add_symbol(&name);

            // set to next node index for now
            // will find root after precedence resolution
            // next node is inserted index because we are dropping this token
            //so inserted index will be the same next iteration
            let root = inserted_index;

            named_expressions.push(NamedExpression::new(root, symbol_index));
            continue;
        }

        if in_document {
            in_document = false;

            // end current document
            // and make new one
            documents.push(current_document);
            current_document = AnnotationDocument::new();
        }

        if token.get_token_type() == TokenType::EndGroup
            || token.get_token_type() == TokenType::EndAssociativeList
            || token.get_token_type() == TokenType::EndExpressionBlock
        {
            // end current group
            // and drop token
            precedence_manager.end_group();
            continue;
        }

        if symbol_next {
            let symbol_value = context.add_symbol(&token.get_token_str());
            nodes
                .get_mut(previous_index)
                .and_then(|previous_node| -> Option<usize> {
                    previous_node.set_value(data.insert_integer(symbol_value as i64));
                    None
                });
            symbol_next = false;
            continue;
        }

        let mut op = get_operation_type_for_token(&token);
        let mut data_type = get_data_type_for_token(&token);

        if data_type == DataType::Symbol {
            symbol_next = true;
        } else if data_type == DataType::Unit {
            // if Unit symbol immediately follows an identifier
            // it is an empty argument list
            if last_data_type == DataType::Identifier || last_op == Operation::CurrentResult {
                op = Operation::Group;
                data_type = DataType::Unknown;
                empty_group = true; // so we don't start a group later
            }
        }

        let value = if token.get_token_type() == TokenType::Identifier {
            let parts: Vec<String> = token
                .get_token_str()
                .split("::")
                .map(|s| String::from(s))
                .collect();
            let identifier = parts.get(parts.len() - 1).unwrap();
            let namespaces = if parts.len() > 1 {
                parts
                    .iter()
                    .take(parts.len() - 1)
                    .map(|s| s.to_owned())
                    .collect()
            } else {
                vec![]
            };

            let symbol_value = context.add_symbol(&identifier);

            if namespaces.len() > 0 {
                identifier_namespaces.insert(symbol_value, namespaces);
            }

            data.insert_integer(symbol_value as i64)
        } else {
            data.insert_from_string(data_type, &token.get_token_str())
        };

        if infix_next {
            // set to be infix
            op = Operation::InfixCall;

            // set to skip next back tick
            infix_next = false;
            infix_last = true;
        }

        if op == Operation::Subtraction && last_data_type == DataType::Unknown {
            // if previous node is not a value
            // this op is actually a Negation operation
            op = Operation::Negation;
        } else if op == Operation::AssociativeList && last_op == Operation::Touch {
            // if op before associative list was a value type
            // then it is an interpreted access operation
            op = Operation::InterpretedAccess;
        }

        let mut node = SELTreeNode::new(op, data_type, inserted_index, value);

        if !link_next {
            // check to see if previous node and current node are terminable
            // i.e. a node that can end an expression
            // if not we need to link it

            // right now only value operations can terminate an expression
            // if last op wasn't one of those
            if !op_is_terminable(last_op) || !op_is_terminable(op) {
                // flip link next so we link previous node with this one
                link_next = true;
            }
        }

        if inserted_index > 0 && link_next {
            match nodes.get_mut(previous_index) {
                None => (),
                Some(previous_node) => {
                    node.set_left(Some(previous_index));
                    previous_node.set_right(Some(inserted_index));
                }
            }
        }

        // flip back for next node
        if !link_next {
            firsts_of_expression.push(inserted_index);
            link_next = true;
        }

        nodes.push(node);

        precedence_manager.add_index_with_operation(node.get_operation(), node.get_own_index());

        if (node.get_operation() == Operation::Group && !empty_group)
            || node.get_operation() == Operation::AssociativeList
            || node.get_operation() == Operation::InterpretedAccess
            || node.get_operation() == Operation::Expression
        {
            precedence_manager.start_group();
        }

        last_data_type = data_type;
        last_op = op;
    }

    // no tokens
    // insert unit node as default
    if nodes.len() == 0 {
        nodes.push(SELTreeNode::new(Operation::None, DataType::Unit, 0, None));
    }

    // add last document if has lines
    if current_document.get_lines().len() > 0 {
        documents.push(current_document);
    }

    return (
        nodes,
        data,
        firsts_of_expression,
        annotations,
        documents,
        named_expressions,
        identifier_namespaces,
    );
}
