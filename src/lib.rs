#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    Integer,
    Decimal,
    SingleQuotedString,
    DoubleQuotedString,
    FormattedString,
    ExclusiveRange,
    InclusiveRange,
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
    ParsingDecimal,
    ParsingSingleQuotedString,
    ParsingDoubleQuotedString,
    FormattedString,
    ParsingExclusiveRange,
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
        } else if c == '`' {
            self.current_token_type = TokenType::FormattedString;
            self.parse_state = ParseState::FormattedString;
        } else if c == '.' {
            self.current_token_type = TokenType::ExclusiveRange;
            self.parse_state = ParseState::ParsingExclusiveRange;
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
                            } else if c == '.' {
                                // consume and
                                // convert to decimal token
                                self.current_token.push(c);
                                self.current_token_type = TokenType::Decimal;
                                self.parse_state = ParseState::ParsingDecimal;
                            } else {
                                let token = self.make_current_token();
                                self.start_new_token(c);
                                return token;
                            }
                        }
                        ParseState::ParsingDecimal => {
                            if c == '.' {
                                // have one '.' in current token to be here
                                // if we receive another one, we're parsing a range

                                // remove first '.' char from current_token
                                // and change it back to an integer
                                self.current_token.pop();
                                self.current_token_type = TokenType::Integer;

                                // generate current token
                                // and store to be returned later
                                let integer_token = self.make_current_token();

                                // start with single dot (one we just removed)
                                self.start_new_token('.');
                                // add current one as well
                                self.current_token.push(c);

                                // return integer token that just ended
                                return integer_token;
                            } else if c.is_numeric() {
                                // continue decimal number
                                self.current_token.push(c);
                            } else {
                                // end token
                                let token = self.make_current_token();
                                self.start_new_token(c);
                                return token;
                            }
                        }
                        ParseState::ParsingExclusiveRange => {
                            // expecting another integer
                            // if that is so
                            // end this token and start new integer token
                            if c == '.' {
                                // already have 2 dots to be here
                                // if next is a third convert to inclusive range
                                self.current_token.push(c);
                                self.current_token_type = TokenType::InclusiveRange;

                                // this is furthest this token can go
                                // end token
                                return self.make_current_token();
                            } else {
                                // not an inclusive range
                                // end with exclusive range token
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
                        ParseState::FormattedString => {
                            self.current_token.push(c);
                            if c == '`' {
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
    fn tokenize_decimal_number() {
        let input = String::from("3.14");
        let tokenizer = Tokenizer::new(&input);
        let tokens: Vec<Token> = tokenizer.collect();

        assert_eq!(tokens.len(), 1);

        let only = tokens.get(0).unwrap();

        assert_eq!(only.token_type, TokenType::Decimal);
        assert_eq!(only.token_str, "3.14");
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

    #[test]
    fn tokenize_string_formatted() {
        let input = String::from("`Hello World`");
        let tokenizer = Tokenizer::new(&input);
        let tokens: Vec<Token> = tokenizer.collect();

        assert_eq!(tokens.len(), 1);

        let only = tokens.get(0).unwrap();

        assert_eq!(only.token_type, TokenType::FormattedString);
        assert_eq!(only.token_str, "`Hello World`");
    }

    #[test]
    fn tokenize_exclusive_range() {
        let input = String::from("1..10");
        let tokenizer = Tokenizer::new(&input);
        let tokens: Vec<Token> = tokenizer.collect();

        assert_eq!(tokens.len(), 3);

        let first = tokens.get(0).unwrap();
        let second = tokens.get(1).unwrap();
        let third = tokens.get(2).unwrap();

        assert_eq!(first.token_type, TokenType::Integer);
        assert_eq!(first.token_str, "1");

        assert_eq!(second.token_type, TokenType::ExclusiveRange);
        assert_eq!(second.token_str, "..");

        assert_eq!(third.token_type, TokenType::Integer);
        assert_eq!(third.token_str, "10");
    }

    #[test]
    fn tokenize_inxclusive_range() {
        let input = String::from("1...10");
        let tokenizer = Tokenizer::new(&input);
        let tokens: Vec<Token> = tokenizer.collect();

        assert_eq!(tokens.len(), 3);

        let first = tokens.get(0).unwrap();
        let second = tokens.get(1).unwrap();
        let third = tokens.get(2).unwrap();

        assert_eq!(first.token_type, TokenType::Integer);
        assert_eq!(first.token_str, "1");

        assert_eq!(second.token_type, TokenType::InclusiveRange);
        assert_eq!(second.token_str, "...");

        assert_eq!(third.token_type, TokenType::Integer);
        assert_eq!(third.token_str, "10");
    }
}
