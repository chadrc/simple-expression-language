use crate::change::Change;
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;
use sel_common::{DataType, NodeSide, Operation, SELTreeNode};

pub fn loop_max<T>(max: usize, mut f: T)
where
    T: FnMut(),
{
    let mut loop_count = 0;
    loop {
        f();

        // fail safe
        // iterate maximum of nodes length
        if loop_count > max {
            break;
        }

        loop_count += 1;
    }
}

pub fn apply_changes(nodes: &mut Vec<SELTreeNode>, changes: Vec<Change>) {
    for change in changes {
        let node = nodes.get_mut(change.index_to_change).unwrap();

        match change.side_to_set {
            NodeSide::Left => node.set_left(change.new_index),
            NodeSide::Right => node.set_right(change.new_index),
            NodeSide::Parent => node.set_parent(change.new_index),
        }
    }
}

fn is_match(op: Operation) -> bool {
    return op == Operation::MatchTrue || op == Operation::MatchFalse;
}

pub fn promote_match_lists(nodes: Vec<SELTreeNode>, list_indices: &Vec<usize>) -> Vec<SELTreeNode> {
    let mut nodes = nodes;
    let mut to_update: Vec<usize> = vec![];

    for index in list_indices {
        let index = *index;

        let update = nodes
            .get(index)
            .and_then(|node| {
                node.get_left()
                    .and_then(|left_index| nodes.get(left_index))
                    .map(|left_node| {
                        if is_match(left_node.get_operation()) {
                            true
                        } else {
                            node.get_right()
                                .and_then(|right_index| nodes.get(right_index))
                                .map(|right_node| is_match(right_node.get_operation()))
                                .unwrap_or(false)
                        }
                    })
            })
            .unwrap_or(false);

        if update {
            to_update.push(index);
        }
    }

    {
        let nodes = &mut nodes;

        for index in to_update {
            nodes.get_mut(index).and_then(|node| {
                node.set_operation(Operation::MatchList);
                Some(true)
            });
        }
    }

    nodes
}

pub fn get_operation_type_for_token(token: &Token) -> Operation {
    return match token.get_token_type() {
        TokenType::PlusSign => Operation::Addition,
        TokenType::MinusSign => Operation::Subtraction,
        TokenType::MultiplicationSign => Operation::Multiplication,
        TokenType::DivisionSign => Operation::Division,
        TokenType::IntegerDivisionSign => Operation::IntegerDivision,
        TokenType::ModulusSign => Operation::Modulo,
        TokenType::ExponentialSign => Operation::Exponential,
        TokenType::ExclusiveRange => Operation::ExclusiveRange,
        TokenType::InclusiveRange => Operation::InclusiveRange,
        TokenType::GreaterThan => Operation::GreaterThan,
        TokenType::GreaterThanOrEqual => Operation::GreaterThanOrEqual,
        TokenType::LessThan => Operation::LessThan,
        TokenType::LessThanOrEqual => Operation::LessThanOrEqual,
        TokenType::Equal => Operation::Equality,
        TokenType::NotEqual => Operation::Inequality,
        TokenType::KeysEqual => Operation::KeysEqual,
        TokenType::KeysNotEqual => Operation::KeysNotEqual,
        TokenType::ValuesEqual => Operation::ValuesEqual,
        TokenType::ValuesNotEqual => Operation::ValuesNotEqual,
        TokenType::Contains => Operation::Contains,
        TokenType::NotContains => Operation::NotContains,
        TokenType::LogicalAnd => Operation::LogicalAnd,
        TokenType::LogicalOr => Operation::LogicalOr,
        TokenType::LogicalXOR => Operation::LogicalXOR,
        TokenType::NotSign => Operation::Not,
        TokenType::Input => Operation::Input,
        TokenType::CurrentResult => Operation::CurrentResult,
        TokenType::StartGroup => Operation::Group,
        TokenType::StartAssociativeList => Operation::AssociativeList,
        TokenType::StartExpressionBlock => Operation::Expression,
        TokenType::BackTick => Operation::Transform,
        TokenType::BitwiseOrSign => Operation::BitwiseOr,
        TokenType::BitwiseXorSign => Operation::BitwiseXOR,
        TokenType::BitwiseAndSign => Operation::BitwiseAnd,
        TokenType::BitwiseLeftShiftSign => Operation::BitwiseLeftShift,
        TokenType::BitwiseRightShiftSign => Operation::BitwiseRightShift,
        TokenType::Pair => Operation::Pair,
        TokenType::Comma => Operation::List,
        TokenType::Dot => Operation::DotAccess,
        TokenType::PipeFirstRight => Operation::PipeFirstRight,
        TokenType::PipeFirstLeft => Operation::PipeFirstLeft,
        TokenType::PipeLastRight => Operation::PipeLastRight,
        TokenType::PipeLastLeft => Operation::PipeLastLeft,
        TokenType::MatchTrue => Operation::MatchTrue,
        TokenType::MatchFalse => Operation::MatchFalse,
        TokenType::Stream => Operation::Stream,
        TokenType::Collect => Operation::Collect,
        TokenType::SeedCollect => Operation::CollectInit,
        TokenType::Partial => Operation::PartialApplication,
        TokenType::Boolean
        | TokenType::Integer
        | TokenType::Decimal
        | TokenType::SingleQuotedString
        | TokenType::DoubleQuotedString
        | TokenType::Symbol
        | TokenType::Identifier
        | TokenType::Unit => Operation::Touch,
        _ => Operation::None,
    };
}

pub fn get_data_type_for_token(token: &Token) -> DataType {
    return match token.get_token_type() {
        TokenType::Integer => DataType::Integer,
        TokenType::Decimal => DataType::Decimal,
        TokenType::SingleQuotedString | TokenType::DoubleQuotedString => DataType::String,
        TokenType::Boolean => DataType::Boolean,
        TokenType::Unit => DataType::Unit,
        TokenType::Symbol => DataType::Symbol,
        TokenType::Identifier => DataType::Identifier,
        _ => DataType::Unknown,
    };
}
