mod tokenizer;
use tokenizer::types::{ParseState, SymbolTree, SymbolTreeNode, Token, TokenType};

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
        if c.is_whitespace() {
            // no tokens start with a white space
            return;
        }

        self.current_token.push(c);

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
                self.current_token_type = TokenType::ExclusiveRange;
                self.parse_state = ParseState::ParsingExclusiveRange;
            }
            _ => {
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
            self.current_token.push(current_character);

            // character not escaped,
            // means its end of token
            if current_character == end_character {
                // mark state to output token in next iteration
                self.parse_state = ParseState::EndOfToken;
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

#[cfg(test)]
mod tests {
    use super::*;

    //#region Tokenizing
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
    fn tokenize_string_single_quote_with_escape() {
        let tokens: Vec<Token> = tokens_from_str("'Hello\\' World'");

        assert_eq!(tokens.len(), 1);
        assert_token(
            tokens.get(0).unwrap(),
            TokenType::SingleQuotedString,
            "'Hello' World'",
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
    fn tokenize_string_double_quote_with_escape() {
        let tokens: Vec<Token> = tokens_from_str("\"Hello\\\" World\"");

        assert_eq!(tokens.len(), 1);
        assert_token(
            tokens.get(0).unwrap(),
            TokenType::DoubleQuotedString,
            "\"Hello\" World\"",
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
    fn tokenize_string_formatted_with_escape() {
        let tokens: Vec<Token> = tokens_from_str("`Hello\\` World`");

        assert_eq!(tokens.len(), 1);
        assert_token(
            tokens.get(0).unwrap(),
            TokenType::FormattedString,
            "`Hello` World`",
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

    #[test]
    fn tokenize_unit() {
        let tokens: Vec<Token> = tokens_from_str("()");

        assert_eq!(tokens.len(), 1);
        assert_token(tokens.get(0).unwrap(), TokenType::Unit, "()");
    }

    #[test]
    fn tokenize_boolean_true() {
        let tokens: Vec<Token> = tokens_from_str("true");

        assert_eq!(tokens.len(), 1);
        assert_token(tokens.get(0).unwrap(), TokenType::Boolean, "true");
    }

    #[test]
    fn tokenize_boolean_false() {
        let tokens: Vec<Token> = tokens_from_str("false");

        assert_eq!(tokens.len(), 1);
        assert_token(tokens.get(0).unwrap(), TokenType::Boolean, "false");
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
        assert_4_5_binary_operation("+", TokenType::PlusSign);
    }

    #[test]
    fn tokenize_subtraction() {
        assert_4_5_binary_operation("-", TokenType::MinusSign);
    }

    #[test]
    fn tokenize_multiplication() {
        assert_4_5_binary_operation("*", TokenType::MultiplicationSign);
    }

    #[test]
    fn tokenize_division() {
        assert_4_5_binary_operation("/", TokenType::DivisionSign);
    }

    #[test]
    fn tokenize_modulus() {
        assert_4_5_binary_operation("%", TokenType::ModulusSign);
    }

    #[test]
    fn tokenize_exponential() {
        assert_4_5_binary_operation("^", TokenType::ExponentialSign);
    }

    #[test]
    fn tokenize_negation() {
        let tokens: Vec<Token> = tokens_from_str("-2");

        assert_eq!(tokens.len(), 2);
        assert_token(tokens.get(0).unwrap(), TokenType::MinusSign, "-");
        assert_token(tokens.get(1).unwrap(), TokenType::Integer, "2");
    }

    #[test]
    fn tokenize_equality() {
        assert_4_5_binary_operation("==", TokenType::Equal);
    }

    #[test]
    fn tokenize_inequality() {
        assert_4_5_binary_operation("!=", TokenType::NotEqual);
    }

    #[test]
    fn tokenize_greater_than() {
        assert_4_5_binary_operation(">", TokenType::GreaterThan);
    }

    #[test]
    fn tokenize_greater_than_or_equal() {
        assert_4_5_binary_operation(">=", TokenType::GreaterThanOrEqual);
    }

    #[test]
    fn tokenize_less_than() {
        assert_4_5_binary_operation("<", TokenType::LessThan);
    }

    #[test]
    fn tokenize_less_than_or_equal() {
        assert_4_5_binary_operation("<=", TokenType::LessThanOrEqual);
    }

    #[test]
    fn tokenize_logicial_and() {
        assert_4_5_binary_operation("&&", TokenType::LogicalAnd);
    }

    #[test]
    fn tokenize_logical_or() {
        assert_4_5_binary_operation("||", TokenType::LogicalOr);
    }

    #[test]
    fn tokenize_logical_not() {
        let tokens: Vec<Token> = tokens_from_str("!true");

        assert_eq!(tokens.len(), 2);
        assert_token(tokens.get(0).unwrap(), TokenType::LogicalNot, "!");
        assert_token(tokens.get(1).unwrap(), TokenType::Boolean, "true");
    }

    #[test]
    fn tokenize_input_symbol() {
        let tokens: Vec<Token> = tokens_from_str("$");
        assert_token(tokens.get(0).unwrap(), TokenType::Input, "$");
    }

    #[test]
    fn tokenize_result_symbol() {
        let tokens: Vec<Token> = tokens_from_str("?");
        assert_token(tokens.get(0).unwrap(), TokenType::CurrentResult, "?");
    }

    //#endregion Tokenizing

    //#region Symbol Tree

    #[test]
    fn symbol_tree_make_empty() {
        SymbolTree::new();
    }

    #[test]
    fn symbol_tree_one_symbol() {
        let mut tree = SymbolTree::new();
        tree.attach("true", TokenType::Boolean);
        check_tree_for_true(&tree);
    }

    #[test]
    fn symbol_tree_two_symbols() {
        let mut tree = SymbolTree::new();

        tree.attach("true", TokenType::Boolean);
        tree.attach("false", TokenType::Boolean);

        check_tree_for_true(&tree);

        // false check
        let f_branch = tree.get_branch("f").unwrap();
        assert_eq!(f_branch.get_character(), "f");

        let a_branch = f_branch.get("a").unwrap();
        assert_eq!(a_branch.get_character(), "a");

        let l_branch = a_branch.get("l").unwrap();
        assert_eq!(l_branch.get_character(), "l");

        let s_branch = l_branch.get("s").unwrap();
        assert_eq!(s_branch.get_character(), "s");

        let e_branch = s_branch.get("e").unwrap();
        assert_eq!(e_branch.get_character(), "e");
    }

    #[test]
    fn symbol_tree_similar_symbols() {
        let mut tree = SymbolTree::new();

        tree.attach("true", TokenType::Boolean);
        tree.attach("tree", TokenType::Unknown);

        let r_branch = tree.get_branch("t").unwrap().get("r").unwrap();

        let e_branch = r_branch.get("e").unwrap();
        assert_eq!(e_branch.get_character(), "e");

        let u_branch = r_branch.get("u").unwrap();
        assert_eq!(u_branch.get_character(), "u");
    }

    //#endregion Symbol Tree

    // Test utils
    fn tokens_from_str(s: &str) -> Vec<Token> {
        let input = String::from(s);
        let tokenizer = Tokenizer::new(&input);
        return tokenizer.collect();
    }

    fn assert_token(token: &Token, token_type: TokenType, token_str: &str) {
        assert_eq!(token.get_token_type(), token_type);
        assert_eq!(token.get_token_str(), token_str);
    }

    fn assert_4_5_binary_operation(op: &str, op_token_type: TokenType) {
        let tokens: Vec<Token> =
            tokens_from_str(&("4".to_owned() + &op.to_owned() + &"5".to_owned()));

        assert_eq!(tokens.len(), 3);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "4");
        assert_token(tokens.get(1).unwrap(), op_token_type, op);
        assert_token(tokens.get(2).unwrap(), TokenType::Integer, "5");
    }

    fn check_tree_for_true(tree: &SymbolTree) {
        let t_branch = tree.get_branch("t").unwrap();
        assert_eq!(t_branch.get_character(), "t");
        assert_eq!(t_branch.get_token_type(), TokenType::Unknown);

        let r_branch = t_branch.get("r").unwrap();
        assert_eq!(r_branch.get_character(), "r");
        assert_eq!(r_branch.get_token_type(), TokenType::Unknown);

        let u_branch = r_branch.get("u").unwrap();
        assert_eq!(u_branch.get_character(), "u");
        assert_eq!(u_branch.get_token_type(), TokenType::Unknown);

        let e_branch = u_branch.get("e").unwrap();
        assert_eq!(e_branch.get_character(), "e");
        assert_eq!(e_branch.get_token_type(), TokenType::Boolean);
    }
}
