use sel_tokenizer::{Token, TokenType};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Operation {
    Touch,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    None,
    Start,
}

pub fn get_operation_type_for_token(token: &Token) -> Operation {
    return if token.get_token_type() == TokenType::PlusSign {
        Operation::Addition
    } else if token.get_token_type() == TokenType::MinusSign {
        Operation::Subtraction
    } else if token.get_token_type() == TokenType::MultiplicationSign {
        Operation::Multiplication
    } else if token.get_token_type() == TokenType::DivisionSign {
        Operation::Division
    } else if token.get_token_type() == TokenType::Boolean
        || token.get_token_type() == TokenType::Integer
        || token.get_token_type() == TokenType::Decimal
        || token.get_token_type() == TokenType::SingleQuotedString
        || token.get_token_type() == TokenType::DoubleQuotedString
        || token.get_token_type() == TokenType::FormattedString
    {
        // all value tokens result in a touch operation
        Operation::Touch
    } else {
        Operation::None
    };
}
