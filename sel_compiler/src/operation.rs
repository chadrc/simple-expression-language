use sel_tokenizer::{Token, TokenType};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Operation {
    Touch,
    Addition,
    Subtraction,
    Multiplication,
    Modulo,
    Division,
    ExclusiveRange,
    InclusiveRange,
    None,
}

pub fn get_operation_type_for_token(token: &Token) -> Operation {
    return match token.get_token_type() {
        TokenType::PlusSign => Operation::Addition,
        TokenType::MinusSign => Operation::Subtraction,
        TokenType::MultiplicationSign => Operation::Multiplication,
        TokenType::DivisionSign => Operation::Division,
        TokenType::ModulusSign => Operation::Modulo,
        TokenType::ExclusiveRange => Operation::ExclusiveRange,
        TokenType::InclusiveRange => Operation::InclusiveRange,
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
