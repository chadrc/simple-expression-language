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
