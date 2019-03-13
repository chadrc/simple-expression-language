use super::parse_state::ParseState;
use super::symbol_tree::{SymbolTree, SymbolTreeNode};
use super::token::Token;
use super::token_type::TokenType;

pub struct Tokenizer<'a> {
    current_token: String,
    current_token_type: TokenType,
    parse_state: ParseState,
    deferred_parse_state: ParseState,
    chars: std::str::Chars<'a>,
    symbol_tree: SymbolTree,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a String) -> Tokenizer<'a> {
        let mut symbol_tree = SymbolTree::new();
        symbol_tree.attach("true", TokenType::Boolean);
        symbol_tree.attach("false", TokenType::Boolean);
        symbol_tree.attach("()", TokenType::Unit);
        symbol_tree.attach("+", TokenType::PlusSign);
        symbol_tree.attach("-", TokenType::MinusSign);
        symbol_tree.attach("*", TokenType::MultiplicationSign);
        symbol_tree.attach("/", TokenType::DivisionSign);
        symbol_tree.attach("^", TokenType::ExponentialSign);
        symbol_tree.attach("%", TokenType::ModulusSign);
        symbol_tree.attach("==", TokenType::Equal);
        symbol_tree.attach("!=", TokenType::NotEqual);
        symbol_tree.attach(">", TokenType::GreaterThan);
        symbol_tree.attach(">=", TokenType::GreaterThanOrEqual);
        symbol_tree.attach("<", TokenType::LessThan);
        symbol_tree.attach("<=", TokenType::LessThanOrEqual);
        symbol_tree.attach("&&", TokenType::LogicalAnd);
        symbol_tree.attach("||", TokenType::LogicalOr);
        symbol_tree.attach("!", TokenType::LogicalNot);
        symbol_tree.attach("$", TokenType::Input);
        symbol_tree.attach("?", TokenType::CurrentResult);
        symbol_tree.attach("(", TokenType::StartGroup);
        symbol_tree.attach(")", TokenType::EndGroup);

        return Tokenizer {
            current_token: String::new(),
            current_token_type: TokenType::Unknown,
            parse_state: ParseState::NoToken,
            deferred_parse_state: ParseState::NoToken,
            chars: input.chars(),
            symbol_tree: symbol_tree,
        };
    }

    fn start_new_token(&mut self, c: char) {
        if c == '\n' {
            // special check here to catch before whitespace check
            self.current_token.push(c);
            self.current_token_type = TokenType::LineEnd;
            self.parse_state = ParseState::EndOfToken;
            return;
        } else if c.is_whitespace() {
            // no tokens start with a white space
            return;
        }

        match c {
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
                self.current_token.push(c);
                self.current_token_type = TokenType::ExclusiveRange;
                self.parse_state = ParseState::ParsingExclusiveRange;
            }
            _ => {
                self.current_token.push(c);
                if c.is_numeric() {
                    self.parse_state = ParseState::ParsingInteger;
                    self.current_token_type = TokenType::Integer;
                } else {
                    let mut s = String::new();
                    s.push(c);
                    match self.symbol_tree.get_branch(&s) {
                        Some(node) => {
                            self.parse_state = ParseState::ParsingSymbol;
                            self.current_token_type = node.get_token_type();
                        }
                        None => (),
                    }
                }
            }
        }
    }

    fn check_escape_character(&mut self, current_character: char, end_character: char) {
        if current_character == '\\' {
            // defer current state to escape next character
            self.deferred_parse_state = self.parse_state;
            self.parse_state = ParseState::EscapeCharacter;
        } else {
            // character not escaped,
            // means its end of token
            if current_character == end_character {
                // mark state to output token in next iteration
                self.parse_state = ParseState::EndOfToken;
            } else {
                // don't add ending quote to token
                self.current_token.push(current_character);
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

    fn end_current_token(&mut self, c: char) -> Option<Token> {
        let token = self.make_current_token();
        self.start_new_token(c);
        return token;
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            match self.chars.next() {
                Some(c) => {
                    match self.parse_state {
                        ParseState::NoToken => {
                            self.start_new_token(c);
                        }
                        ParseState::EndOfToken => {
                            return self.end_current_token(c);
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
                                return self.end_current_token(c);
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
                                return self.end_current_token(c);
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
                                return self.end_current_token(c);
                            }
                        }
                        ParseState::ParsingSingleQuotedString => {
                            self.check_escape_character(c, '\'');
                        }
                        ParseState::ParsingDoubleQuotedString => {
                            self.check_escape_character(c, '"');
                        }
                        ParseState::ParsingFormattedString => {
                            self.check_escape_character(c, '`');
                        }
                        ParseState::EscapeCharacter => {
                            self.current_token.push(c);
                            self.parse_state = self.deferred_parse_state;
                            self.deferred_parse_state = ParseState::NoToken;
                        }
                        ParseState::ParsingSymbol => {
                            let mut node: Option<&SymbolTreeNode> = None;

                            // Get current node based on current token characters
                            for nc in self.current_token.chars() {
                                let mut ns = String::new();
                                ns.push(nc);
                                match node {
                                    None => {
                                        node = self.symbol_tree.get_branch(&ns);
                                    }
                                    Some(n) => {
                                        node = n.get(&ns);
                                    }
                                }
                            }

                            // make sure node exists
                            match node {
                                None => (),
                                Some(n) => {
                                    let mut ns = String::new();
                                    ns.push(c);
                                    match n.get(&ns) {
                                        None => {
                                            return self.end_current_token(c);
                                        }
                                        Some(next) => {
                                            // has child for current character
                                            // update token type and push character to token
                                            self.current_token_type = next.get_token_type();
                                            self.current_token.push(c);
                                        }
                                    }
                                }
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
