pub mod types {
    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum TokenType {
        Integer,
        Decimal,
        SingleQuotedString,
        DoubleQuotedString,
        FormattedString,
        ExclusiveRange,
        InclusiveRange,
        Boolean,
        PlusSign,
        Unit,
        Unknown,
    }

    #[derive(PartialEq, Debug, Copy, Clone)]
    pub enum ParseState {
        NoToken,
        EndOfToken,
        ParsingInteger,
        ParsingDecimal,
        ParsingSingleQuotedString,
        ParsingDoubleQuotedString,
        ParsingFormattedString,
        EscapeCharacter,
        ParsingExclusiveRange,
        ParsingUnit,
    }

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

    pub struct SymbolTree {}

    impl SymbolTree {
        pub fn new() -> SymbolTree {
            return SymbolTree {};
        }
        pub fn num_children(self) -> u64 {
            return 0;
        }
    }
}
