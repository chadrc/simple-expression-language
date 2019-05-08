use crate::precedence_manager::PrecedenceManager;
use crate::utils::{get_data_type_for_token, get_operation_type_for_token};
use sel_common::{DataHeap, DataType, Operation, SELContext, SELTreeNode};
use sel_tokenizer::{TokenType, Tokenizer};

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
) -> (Vec<SELTreeNode>, DataHeap, Vec<usize>) {
    let mut nodes: Vec<SELTreeNode> = vec![];
    let mut data = DataHeap::new();
    let mut firsts_of_expression: Vec<usize> = vec![];

    // loop trough all tokens
    // convert them to tree nodes
    // and link them together

    let mut last_data_type = DataType::Unknown;
    let mut last_op = Operation::None;
    let mut link_next = true;
    let mut symbol_next = false;
    let mut empty_group = false;

    for token in tokenizer {
        let inserted_index = nodes.len();
        let previous_index = if nodes.len() > 0 {
            inserted_index - 1
        } else {
            0
        };

        if token.get_token_type() == TokenType::CommentAnnotation
            || token.get_token_type() == TokenType::DocumentAnnotation
        {
            // will store later for meta data
            // for now, just drop the token
            continue;
        }

        if token.get_token_type() == TokenType::LineEnd {
            // set next token to not be linked to previous
            // skip this token, no need to convert LineEnd to a node
            link_next = false;
            continue;
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
            let symbol_value = context.add_symbol(&token.get_token_str());
            data.insert_integer(symbol_value as i64)
        } else {
            data.insert_from_string(data_type, &token.get_token_str())
        };

        if op == Operation::Subtraction && last_data_type == DataType::Unknown {
            // if previous node is not a value
            // this op is actually a Negation operation
            op = Operation::Negation;
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

    return (nodes, data, firsts_of_expression);
}
