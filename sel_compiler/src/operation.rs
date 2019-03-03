use sel_tokenizer::{Token, TokenType};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Operation {
    Touch,
    Addition,
    Subtraction,
    Multiplication,
    Modulo,
    Division,
    Exponential,
    ExclusiveRange,
    InclusiveRange,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equality,
    Inequality,
    LogicalAnd,
    LogicalOr,
    None,
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
