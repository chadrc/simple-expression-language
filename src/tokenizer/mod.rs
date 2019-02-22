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
        fn new(s: &str, children: HashMap<String, SymbolTreeNode>) -> SymbolTreeNode {
            return SymbolTreeNode {
                character: String::from(s),
                children: children,
            };
        }

        fn from(s: &str) -> Option<SymbolTreeNode> {
            let mut map = HashMap::new();
            let mut last: Option<SymbolTreeNode> = None;
            for c in s.graphemes(true).rev() {
                match last {
                    Some(l) => {
                        map.insert(l.get_character(), l);
                    }
                    None => (),
                }

                last = Some(SymbolTreeNode::new(c, map));
                map = HashMap::new();
            }

            return last;
        }

        pub fn get_character(&self) -> String {
            return self.character.clone();
        }

        pub fn get(&self, s: &str) -> Option<&SymbolTreeNode> {
            return self.children.get(&String::from(s));
        }

        fn get_mut(&mut self, s: &str) -> Option<&mut SymbolTreeNode> {
            return self.children.get_mut(&String::from(s));
        }
    }

    pub struct SymbolTree {
        root: SymbolTreeNode,
    }

    impl SymbolTree {
        pub fn new() -> SymbolTree {
            return SymbolTree {
                root: SymbolTreeNode::new("", HashMap::new()),
            };
        }

        pub fn get_branch(&self, s: &str) -> Option<&SymbolTreeNode> {
            return self.root.get(&String::from(s));
        }

        fn get_branch_mut(&mut self, s: &str) -> Option<&mut SymbolTreeNode> {
            return self.root.get_mut(&String::from(s));
        }

        fn root_mut(&mut self) -> &mut SymbolTreeNode {
            return &mut self.root;
        }

        pub fn attach(&mut self, s: &str) {
            SymbolTree::attach_deep(&String::from(s), 0, &mut self.root);
        }

        fn attach_deep(s: &String, character_index: usize, node: &mut SymbolTreeNode) {
            if character_index + 1 > s.len() {
                return;
            }

            // Drill down branch add non-exisiting nodes
            let next_character = &s[character_index..character_index + 1];

            match node.get_mut(next_character) {
                Some(b1) => {
                    // continue drill
                    SymbolTree::attach_deep(s, character_index + 1, b1);
                }
                None => {
                    // attach here
                    let remaining_characters = &s[character_index..];
                    match SymbolTreeNode::from(remaining_characters) {
                        Some(t) => {
                            node.children.insert(t.get_character(), t);
                        }
                        None => {
                            println!("end {:?} {:?}", s, character_index);
                        }
                    }
                }
            }
        }
    }
}
