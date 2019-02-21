pub mod types {
    use std::collections::HashMap;
    use unicode_segmentation::UnicodeSegmentation;

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

    #[derive(Debug, Clone)]
    pub struct SymbolTreeNode {
        character: String,
        children: HashMap<String, SymbolTreeNode>,
    }

    impl SymbolTreeNode {
        fn new(c: &str, children: HashMap<String, SymbolTreeNode>) -> SymbolTreeNode {
            return SymbolTreeNode {
                character: String::from(c),
                children: children,
            };
        }

        pub fn get_character(&self) -> String {
            return self.character.clone();
        }

        pub fn get(&self, s: &str) -> Option<&SymbolTreeNode> {
            return self.children.get(&String::from(s));
        }
    }

    pub struct SymbolTree {
        branches: HashMap<String, SymbolTreeNode>,
    }

    impl SymbolTree {
        pub fn new() -> SymbolTree {
            let mut branches = HashMap::new();

            match SymbolTree::make_t_branch() {
                Some(t) => {
                    branches.insert(t.get_character(), t);
                }
                None => (),
            }

            return SymbolTree { branches: branches };
        }

        fn make_t_branch() -> Option<SymbolTreeNode> {
            let mut map = HashMap::new();
            let mut last: Option<SymbolTreeNode> = None;
            for c in "true".graphemes(true).rev() {
                match last {
                    Some(l) => {
                        map.insert(l.get_character(), l);
                    }
                    None => (),
                }

                last = Some(SymbolTreeNode::new(c, map));
                map = HashMap::new();
            }

            // let e = SymbolTreeNode::new('r', HashMap::new());

            // let mut uc = HashMap::new();
            // uc.insert('e', e);
            // let u = SymbolTreeNode::new('r', uc);

            // let mut rc = HashMap::new();
            // rc.insert('u', u);
            // let r = SymbolTreeNode::new('r', rc);

            // let mut tc = HashMap::new();
            // tc.insert('r', r);
            // let t = SymbolTreeNode::new('t', tc);

            return last;
        }

        pub fn get_branch(&self, s: &str) -> Option<&SymbolTreeNode> {
            return self.branches.get(&String::from(s));
        }
    }
}
