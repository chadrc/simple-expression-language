use sel_common::{DataType, Operation};
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

pub fn get_operation_type_for_token(token: &Token) -> Operation {
    return match token.get_token_type() {
        TokenType::PlusSign => Operation::Addition,
        TokenType::MinusSign => Operation::Subtraction,
        TokenType::MultiplicationSign => Operation::Multiplication,
        TokenType::DivisionSign => Operation::Division,
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
        TokenType::LogicalAnd => Operation::LogicalAnd,
        TokenType::LogicalOr => Operation::LogicalOr,
        TokenType::LogicalNot => Operation::LogicalNot,
        TokenType::Boolean
        | TokenType::Integer
        | TokenType::Decimal
        | TokenType::SingleQuotedString
        | TokenType::DoubleQuotedString
        | TokenType::FormattedString
        | TokenType::Unit
        | TokenType::Input
        | TokenType::CurrentResult => Operation::Touch,
        _ => Operation::None,
    };
}

pub fn get_data_type_for_token(token: &Token) -> DataType {
    return match token.get_token_type() {
        TokenType::Integer => DataType::Integer,
        TokenType::Decimal => DataType::Decimal,
        TokenType::SingleQuotedString
        | TokenType::DoubleQuotedString
        | TokenType::FormattedString => DataType::String,
        TokenType::Boolean => DataType::Boolean,
        TokenType::Unit => DataType::Unit,
        TokenType::Input => DataType::Input,
        TokenType::CurrentResult => DataType::CurrentResult,
        _ => DataType::Unknown,
    };
}
