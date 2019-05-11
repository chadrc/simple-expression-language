use super::token_type::TokenType;

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    token_str: String,
}

impl Token {
    pub fn new(token_type: TokenType, token_str: String) -> Token {
        return Token {
            token_type: token_type,
            token_str: token_str,
        };
    }

    pub fn get_token_type(&self) -> TokenType {
        return self.token_type;
    }

    pub fn get_token_str(&self) -> String {
        return self.token_str.clone();
    }
}
