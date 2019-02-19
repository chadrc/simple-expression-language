#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    Integer,
    SingleQuotedString,
    DoubleQuotedString,
    PlusSign,
    Unknown,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    token_str: String,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum ParseState {
    NoToken,
    ParsingInteger,
    ParsingSingleQuotedString,
    ParsingDoubleQuotedString,
}

struct Tokenizer<'a> {
    current_token: String,
    current_token_type: TokenType,
    parse_state: ParseState,
    chars: std::str::Chars<'a>,
    end_of_token: bool,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a String) -> Tokenizer<'a> {
        return Tokenizer {
            current_token: String::new(),
            current_token_type: TokenType::Unknown,
            parse_state: ParseState::NoToken,
            chars: input.chars(),
            end_of_token: false,
        };
    }

    fn start_new_token(&mut self, c: char) {
        if c.is_whitespace() {
            // no tokens start with a white space
            return;
        }

        self.current_token.push(c);
        if c.is_numeric() {
            self.parse_state = ParseState::ParsingInteger;
            self.current_token_type = TokenType::Integer;
        } else if self.current_token == "+" {
            self.current_token_type = TokenType::PlusSign;
            self.end_of_token = true;
        } else if c == '\'' {
            self.current_token_type = TokenType::SingleQuotedString;
            self.parse_state = ParseState::ParsingSingleQuotedString;
        } else if c == '"' {
            self.current_token_type = TokenType::DoubleQuotedString;
            self.parse_state = ParseState::ParsingDoubleQuotedString;
        }
    }

    fn make_current_token(&mut self) -> Option<Token> {
        return if self.current_token.len() > 0 {
            let token = Token {
                token_type: self.current_token_type,
                token_str: self.current_token.clone(),
            };

            self.current_token = String::new();
            self.current_token_type = TokenType::Unknown;
            self.parse_state = ParseState::NoToken;

            Some(token)
        } else {
            None
        };
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            if self.end_of_token {
                self.end_of_token = false;
                return self.make_current_token();
            }

            match self.chars.next() {
                Some(c) => {
                    match self.parse_state {
                        ParseState::NoToken => {
                            self.start_new_token(c);
                        }
                        ParseState::ParsingInteger => {
                            if c.is_numeric() {
                                self.current_token.push(c);
                            } else {
                                let token = self.make_current_token();
                                self.start_new_token(c);
                                return token;
                            }
                        }
                        ParseState::ParsingSingleQuotedString => {
                            self.current_token.push(c);
                            if c == '\'' {
                                return self.make_current_token();
                            }
                        }
                        ParseState::ParsingDoubleQuotedString => {
                            self.current_token.push(c);
                            if c == '"' {
                                return self.make_current_token();
                            }
                        }
                    };
                }
                // will return None if there is not a last token
                None => return self.make_current_token(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_integer_expression() {
        let input = String::from("4");

        let tokenizer = Tokenizer::new(&input);

        let tokens: Vec<Token> = tokenizer.collect();

        assert_eq!(tokens.len(), 1);

        let only = tokens.get(0).unwrap();

        assert_eq!(only.token_type, TokenType::Integer);
        assert_eq!(only.token_str, "4");
    }

    #[test]
    fn tokenize_two_digit_integer() {
        let input = String::from("43");
        let tokenizer = Tokenizer::new(&input);
        let tokens: Vec<Token> = tokenizer.collect();

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
        let tokenizer = Tokenizer::new(&input);
        let tokens: Vec<Token> = tokenizer.collect();

        // assert_eq!(tokens.len(), 3);

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

    #[test]
    fn tokenize_string_single_quote() {
        let input = String::from("'Hello World'");
        let tokenizer = Tokenizer::new(&input);
        let tokens: Vec<Token> = tokenizer.collect();

        assert_eq!(tokens.len(), 1);

        let only = tokens.get(0).unwrap();

        assert_eq!(only.token_type, TokenType::SingleQuotedString);
        assert_eq!(only.token_str, "'Hello World'");
    }

    #[test]
    fn tokenize_string_double_quote() {
        let input = String::from("\"Hello World\"");
        let tokenizer = Tokenizer::new(&input);
        let tokens: Vec<Token> = tokenizer.collect();

        assert_eq!(tokens.len(), 1);

        let only = tokens.get(0).unwrap();

        assert_eq!(only.token_type, TokenType::DoubleQuotedString);
        assert_eq!(only.token_str, "\"Hello World\"");
    }
}
