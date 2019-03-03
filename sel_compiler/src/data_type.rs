use sel_tokenizer::{Token, TokenType};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum DataType {
    Unknown,
    Unit,
    Integer,
    Decimal,
    String,
    Boolean,
    Input,
    CurrentResult,
}

pub fn get_data_type_for_token(token: &Token) -> DataType {
    let token_type = token.get_token_type();

    return if token_type == TokenType::Integer {
        DataType::Integer
    } else if token_type == TokenType::Decimal {
        DataType::Decimal
    } else if token_type == TokenType::SingleQuotedString
        || token_type == TokenType::DoubleQuotedString
        || token_type == TokenType::FormattedString
    {
        DataType::String
    } else if token_type == TokenType::Boolean {
        DataType::Boolean
    } else if token_type == TokenType::Unit {
        DataType::Unit
    } else if token_type == TokenType::Input {
        DataType::Input
    } else if token_type == TokenType::CurrentResult {
        DataType::CurrentResult
    } else {
        DataType::Unknown
    };
}
