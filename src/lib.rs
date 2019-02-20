mod tokenizer;

use tokenizer::types::{ParseState, Token, TokenType};

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

        match c {
            '+' => {
                self.current_token_type = TokenType::PlusSign;
                self.end_of_token = true;
            }
            '\'' => {
                self.current_token_type = TokenType::SingleQuotedString;
                self.parse_state = ParseState::ParsingSingleQuotedString;
            }
            '"' => {
                self.current_token_type = TokenType::DoubleQuotedString;
                self.parse_state = ParseState::ParsingDoubleQuotedString;
            }
            '`' => {
                self.current_token_type = TokenType::FormattedString;
                self.parse_state = ParseState::ParsingFormattedString;
            }
            '.' => {
                self.current_token_type = TokenType::ExclusiveRange;
                self.parse_state = ParseState::ParsingExclusiveRange;
            }
            _ => {
                if c.is_numeric() {
                    self.parse_state = ParseState::ParsingInteger;
                    self.current_token_type = TokenType::Integer;
                }
            }
        }
    }

    fn make_current_token(&mut self) -> Option<Token> {
        return if self.current_token.len() > 0 {
            let token = Token::new(self.current_token_type, self.current_token.clone());

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
                        ParseState::ParsingFormattedString => {
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
        let tokens: Vec<Token> = tokens_from_str("4");

        assert_eq!(tokens.len(), 1);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "4");
    }

    #[test]
    fn tokenize_two_digit_integer() {
        let tokens: Vec<Token> = tokens_from_str("43");

        assert_eq!(tokens.len(), 1);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "43");
    }

    #[test]
    fn tokenize_decimal_number() {
        let tokens: Vec<Token> = tokens_from_str("3.14");

        assert_eq!(tokens.len(), 1);
        assert_token(tokens.get(0).unwrap(), TokenType::Decimal, "3.14");
    }

    #[test]
    fn tokenize_addition_expression() {
        let tokens: Vec<Token> = tokens_from_str("4 + 5");

        assert_eq!(tokens.len(), 3);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "4");
        assert_token(tokens.get(1).unwrap(), TokenType::PlusSign, "+");
        assert_token(tokens.get(2).unwrap(), TokenType::Integer, "5");
    }

    #[test]
    fn tokenize_addition_expression_no_space() {
        let tokens: Vec<Token> = tokens_from_str("4+5");

        assert_eq!(tokens.len(), 3);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "4");
        assert_token(tokens.get(1).unwrap(), TokenType::PlusSign, "+");
        assert_token(tokens.get(2).unwrap(), TokenType::Integer, "5");
    }

    #[test]
    fn tokenize_string_single_quote() {
        let tokens: Vec<Token> = tokens_from_str("'Hello World'");

        assert_eq!(tokens.len(), 1);
        assert_token(
            tokens.get(0).unwrap(),
            TokenType::SingleQuotedString,
            "'Hello World'",
        );
    }

    #[test]
    fn tokenize_string_double_quote() {
        let tokens: Vec<Token> = tokens_from_str("\"Hello World\"");

        assert_eq!(tokens.len(), 1);
        assert_token(
            tokens.get(0).unwrap(),
            TokenType::DoubleQuotedString,
            "\"Hello World\"",
        );
    }

    #[test]
    fn tokenize_string_formatted() {
        let tokens: Vec<Token> = tokens_from_str("`Hello World`");

        assert_eq!(tokens.len(), 1);
        assert_token(
            tokens.get(0).unwrap(),
            TokenType::FormattedString,
            "`Hello World`",
        );
    }

    #[test]
    fn tokenize_exclusive_range() {
        let tokens: Vec<Token> = tokens_from_str("1..10");

        assert_eq!(tokens.len(), 3);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "1");
        assert_token(tokens.get(1).unwrap(), TokenType::ExclusiveRange, "..");
        assert_token(tokens.get(2).unwrap(), TokenType::Integer, "10");
    }

    #[test]
    fn tokenize_inxclusive_range() {
        let tokens: Vec<Token> = tokens_from_str("1...10");

        assert_eq!(tokens.len(), 3);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "1");
        assert_token(tokens.get(1).unwrap(), TokenType::InclusiveRange, "...");
        assert_token(tokens.get(2).unwrap(), TokenType::Integer, "10");
    }

    fn tokens_from_str(s: &str) -> Vec<Token> {
        let input = String::from(s);
        let tokenizer = Tokenizer::new(&input);
        return tokenizer.collect();
    }

    fn assert_token(token: &Token, token_type: TokenType, token_str: &str) {
        assert_eq!(token.get_token_type(), token_type);
        assert_eq!(token.get_token_str(), token_str);
    }
}
