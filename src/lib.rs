#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    Integer,
    PlusSign,
    Unknown,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    token_type: TokenType,
    token_str: String,
}

enum ParseState {
    NoToken,
    ParsingInteger,
}

struct Tokenizer {
    tokens: Vec<Token>,
    current_token: String,
    current_token_type: TokenType,
    parse_state: ParseState,
}

impl Tokenizer {
    fn new() -> Tokenizer {
        return Tokenizer {
            tokens: vec![],
            current_token: String::new(),
            current_token_type: TokenType::Unknown,
            parse_state: ParseState::NoToken,
        };
    }

    fn tokenize(mut self, input: &String) -> Vec<Token> {
        for c in input.chars() {
            if c.is_whitespace() {
                self.parse_state = ParseState::NoToken;

                if self.current_token.len() > 0 {
                    self.add_current_token();
                }

                continue;
            }

            match &self.parse_state {
                ParseState::NoToken => {
                    self.current_token.push(c);
                    if c.is_numeric() {
                        self.parse_state = ParseState::ParsingInteger;
                        self.current_token_type = TokenType::Integer;
                    } else if self.current_token == "+" {
                        self.current_token_type = TokenType::PlusSign;
                        self.add_current_token();
                    }
                }
                ParseState::ParsingInteger => {
                    if c.is_numeric() {
                        self.current_token.push(c);
                    } else {
                        self.parse_state = ParseState::NoToken;

                        self.add_current_token();

                        // add non numeric char to next token
                        self.current_token.push(c);

                        if self.current_token == "+" {
                            self.current_token_type = TokenType::PlusSign;
                            self.add_current_token();
                        }
                    }
                }
            };
        }

        // Add last token if exists
        if self.current_token.len() > 0 {
            self.tokens.push(Token {
                token_type: self.current_token_type,
                token_str: self.current_token,
            });
        }

        return self.tokens;
    }

    fn add_current_token(&mut self) {
        self.tokens.push(Token {
            token_type: self.current_token_type,
            token_str: self.current_token.clone(),
        });

        self.current_token = String::new();
        self.current_token_type = TokenType::Unknown;
    }
}

pub fn tokenize(input: &String) -> Vec<Token> {
    return Tokenizer::new().tokenize(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_integer_expression() {
        let input = String::from("4");
        let tokens = tokenize(&input);

        assert_eq!(tokens.len(), 1);

        let only = tokens.get(0).unwrap();

        assert_eq!(only.token_type, TokenType::Integer);
        assert_eq!(only.token_str, "4");
    }

    #[test]
    fn tokenize_two_digit_integer() {
        let input = String::from("43");
        let tokens = tokenize(&input);

        assert_eq!(tokens.len(), 1);

        let only = tokens.get(0).unwrap();

        assert_eq!(only.token_type, TokenType::Integer);
        assert_eq!(only.token_str, "43");
    }

    #[test]
    fn tokenize_addition_expression() {
        let input = String::from("4 + 5");
        assert_addition_expression(input);
    }

    #[test]
    fn tokenize_addition_expression_no_space() {
        let input = String::from("4+5");
        assert_addition_expression(input);
    }

    fn assert_addition_expression(input: String) {
        let tokens = tokenize(&input);
        assert_eq!(tokens.len(), 3);

        assert_eq!(
            *tokens.get(0).unwrap(),
            Token {
                token_type: TokenType::Integer,
                token_str: String::from("4")
            }
        );

        assert_eq!(
            *tokens.get(1).unwrap(),
            Token {
                token_type: TokenType::PlusSign,
                token_str: String::from("+")
            }
        );

        assert_eq!(
            *tokens.get(2).unwrap(),
            Token {
                token_type: TokenType::Integer,
                token_str: String::from("5")
            }
        );
    }
}
