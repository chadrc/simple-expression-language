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
    input: String,
    next_index: usize,
    token_type_history: Vec<TokenType>,
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
        symbol_tree.attach("//", TokenType::IntegerDivisionSign);
        symbol_tree.attach("**", TokenType::ExponentialSign);
        symbol_tree.attach("%", TokenType::ModulusSign);
        symbol_tree.attach("==", TokenType::Equal);
        symbol_tree.attach("!=", TokenType::NotEqual);
        symbol_tree.attach(">", TokenType::GreaterThan);
        symbol_tree.attach(">=", TokenType::GreaterThanOrEqual);
        symbol_tree.attach("<", TokenType::LessThan);
        symbol_tree.attach("<=", TokenType::LessThanOrEqual);
        symbol_tree.attach("&&", TokenType::LogicalAnd);
        symbol_tree.attach("||", TokenType::LogicalOr);
        symbol_tree.attach("^^", TokenType::LogicalXOR);
        symbol_tree.attach("!", TokenType::NotSign);
        symbol_tree.attach("|", TokenType::BitwiseOrSign);
        symbol_tree.attach("&", TokenType::BitwiseAndSign);
        symbol_tree.attach("^", TokenType::BitwiseXorSign);
        symbol_tree.attach("<<", TokenType::BitwiseLeftShiftSign);
        symbol_tree.attach(">>", TokenType::BitwiseRightShiftSign);
        symbol_tree.attach("`", TokenType::TransformationSign);
        symbol_tree.attach("$", TokenType::Input);
        symbol_tree.attach("?", TokenType::CurrentResult);
        symbol_tree.attach("(", TokenType::StartGroup);
        symbol_tree.attach(")", TokenType::EndGroup);
        symbol_tree.attach(":", TokenType::Symbol);
        symbol_tree.attach("=", TokenType::Pair);
        symbol_tree.attach(",", TokenType::Comma);
        symbol_tree.attach("///", TokenType::Comment);
        symbol_tree.attach("->", TokenType::PipeFirstRight);
        symbol_tree.attach("<-", TokenType::PipeFirstLeft);
        symbol_tree.attach("|>", TokenType::PipeLastRight);
        symbol_tree.attach("<|", TokenType::PipeLastLeft);
        symbol_tree.attach("~", TokenType::Partial);

        return Tokenizer {
            current_token: String::new(),
            current_token_type: TokenType::Unknown,
            parse_state: ParseState::NoToken,
            deferred_parse_state: ParseState::NoToken,
            chars: input.chars(),
            symbol_tree,
            input: input.clone(),
            next_index: 0,
            token_type_history: vec![],
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
            '.' => {
                self.current_token.push(c);
                self.current_token_type = TokenType::Dot;
                self.parse_state = ParseState::ParsingDot;
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
                        None => {
                            self.parse_state = ParseState::ParsingIdentifier;
                            self.current_token_type = TokenType::Identifier;
                        }
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
        self.token_type_history.push(self.current_token_type);
        let token = self.make_current_token();
        self.start_new_token(c);
        return token;
    }

    fn nth_token_history_is(&self, n: usize, token_types: &[TokenType]) -> bool {
        if (self.token_type_history.len() as i64) - (n as i64) < 0 {
            return false;
        }

        return self
            .token_type_history
            .get(self.token_type_history.len() - n)
            .map_or(false, |history_type| token_types.contains(history_type));
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            // enumerate was causing mutability issues
            // maintaining own index for now
            self.next_index += 1;
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
                                let preceded_by_dot =
                                    self.nth_token_history_is(1, &[TokenType::Dot]);
                                // slight look ahead to determine if this dot is its own token
                                // check for alpha or _ because 3.value or 3._ are not decimals
                                // also check if preceded by a dot because .3. is not a decimal
                                let next = self.input.chars().nth(self.next_index).unwrap_or('\0');
                                if (next.is_alphabetic() || next == '_') || preceded_by_dot {
                                    // it is a dot
                                    // end current integer
                                    // starting dot token as well
                                    return self.end_current_token(c);
                                } else {
                                    // consume and convert to decimal token
                                    self.current_token.push(c);
                                    self.current_token_type = TokenType::Decimal;
                                    self.parse_state = ParseState::ParsingDecimal;
                                }
                            } else {
                                return self.end_current_token(c);
                            }
                        }
                        ParseState::ParsingDecimal => {
                            let curr_last_char =
                                self.current_token.chars().rev().next().unwrap_or('\0');
                            if c == '.' && curr_last_char == '.' {
                                // have one '.' in current token to be here
                                // if we receive another one in a row
                                // we're parsing a range

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

                                // the start_new_token will set parsing to Dot
                                // override because we already added second dot
                                self.current_token_type = TokenType::ExclusiveRange;
                                self.parse_state = ParseState::ParsingExclusiveRange;

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
                        ParseState::ParsingDot => {
                            let preceded_by_value = self.nth_token_history_is(
                                1,
                                &[
                                    TokenType::Identifier,
                                    TokenType::DoubleQuotedString,
                                    TokenType::Boolean,
                                    TokenType::CurrentResult,
                                    TokenType::Input,
                                    TokenType::Integer,
                                    TokenType::Decimal,
                                    TokenType::EndGroup,
                                ],
                            );

                            if c == '.' {
                                // start parsing inclusive range
                                self.current_token.push(c);

                                self.current_token_type = TokenType::ExclusiveRange;
                                self.parse_state = ParseState::ParsingExclusiveRange;
                            } else if c.is_numeric() && !preceded_by_value {
                                // actually parsing a decimal
                                self.current_token.push(c);

                                self.current_token_type = TokenType::Decimal;
                                self.parse_state = ParseState::ParsingDecimal;
                            } else {
                                return self.end_current_token(c);
                            }
                        }
                        ParseState::ParsingSingleQuotedString => {
                            self.check_escape_character(c, '\'');
                        }
                        ParseState::ParsingDoubleQuotedString => {
                            self.check_escape_character(c, '"');
                        }
                        ParseState::EscapeCharacter => {
                            self.current_token.push(c);
                            self.parse_state = self.deferred_parse_state;
                            self.deferred_parse_state = ParseState::NoToken;
                        }
                        ParseState::ParsingIdentifier => {
                            if c.is_alphanumeric() || c == '_' {
                                self.current_token.push(c);
                            } else {
                                return self.end_current_token(c);
                            }
                        }
                        ParseState::ParsingComment => {
                            if c == '\n' {
                                // starting new EndLine token shouldn't affect anything
                                // may want to skip in future anyway
                                return self.end_current_token(c);
                            } else {
                                self.current_token.push(c);
                            }
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
                                            let first_is_alpha = self
                                                .current_token
                                                .chars()
                                                .next()
                                                .map_or(false, |c| c.is_alphabetic() || c == '_');

                                            let curr_is_alpha_num = c.is_alphanumeric();

                                            if first_is_alpha && curr_is_alpha_num {
                                                // continuing an identifier
                                                self.parse_state = ParseState::ParsingIdentifier;
                                                self.current_token_type = TokenType::Identifier;
                                                self.current_token.push(c);
                                            } else if self.current_token_type == TokenType::Comment
                                            {
                                                // comment tokens consist of entire line
                                                // not just the comment symbol
                                                self.parse_state = ParseState::ParsingComment;
                                                self.current_token.push(c);
                                            } else {
                                                // end of symbol
                                                return self.end_current_token(c);
                                            }
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
