use crate::change::Change;
use sel_common::{DataType, NodeSide, Operation, SELTreeNode};
use sel_tokenizer::{Token, TokenType};

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
        TokenType::TransformationSign => Operation::Transform,
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
        TokenType::MatchEqual => Operation::MatchEqual,
        TokenType::MatchNotEqual => Operation::MatchNotEqual,
        TokenType::MatchLessThan => Operation::MatchLessThan,
        TokenType::MatchLessThanEqual => Operation::MatchLessThanEqual,
        TokenType::MatchGreaterThan => Operation::MatchGreaterThan,
        TokenType::MatchGreaterThanEqual => Operation::MatchGreaterThanEqual,
        TokenType::MatchKeysEqual => Operation::MatchKeysEqual,
        TokenType::MatchKeysNotEqual => Operation::MatchKeysNotEqual,
        TokenType::MatchValuesEqual => Operation::MatchValuesEqual,
        TokenType::MatchValuesNotEqual => Operation::MatchValuesNotEqual,
        TokenType::MatchContains => Operation::MatchContains,
        TokenType::MatchNotContains => Operation::MatchNotContains,
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
