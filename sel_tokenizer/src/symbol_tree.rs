use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct SymbolTreeNode {
    character: String,
    token_type: TokenType,
    children: HashMap<String, SymbolTreeNode>,
}

impl SymbolTreeNode {
    fn new(
        s: &str,
        token_type: TokenType,
        children: HashMap<String, SymbolTreeNode>,
    ) -> SymbolTreeNode {
        return SymbolTreeNode {
            character: String::from(s),
            token_type: token_type,
            children: children,
        };
    }

    fn from(s: &str, token_type: TokenType) -> Option<SymbolTreeNode> {
        let mut map = HashMap::new();
        let mut last: Option<SymbolTreeNode> = None;

        for c in s.graphemes(true).rev() {
            let mut t = TokenType::Unknown;
            match last {
                Some(l) => {
                    map.insert(l.get_character(), l);
                }
                None => {
                    t = token_type;
                }
            }

            last = Some(SymbolTreeNode::new(c, t, map));
            map = HashMap::new();
        }

        return last;
    }

    pub fn get_character(&self) -> String {
        return self.character.clone();
    }

    pub fn get_token_type(&self) -> TokenType {
        return self.token_type;
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
            root: SymbolTreeNode::new("", TokenType::Unknown, HashMap::new()),
        };
    }

    pub fn get_branch(&self, s: &str) -> Option<&SymbolTreeNode> {
        return self.root.get(&String::from(s));
    }

    pub fn attach(&mut self, s: &str, token_type: TokenType) {
        SymbolTree::attach_deep(token_type, &String::from(s), 0, &mut self.root);
    }

    fn attach_deep(
        token_type: TokenType,
        s: &String,
        character_index: usize,
        node: &mut SymbolTreeNode,
    ) {
        if character_index + 1 > s.len() {
            node.token_type = token_type;
            return;
        }

        // Drill down branch add non-exisiting nodes
        let next_character = &s[character_index..character_index + 1];

        match node.get_mut(next_character) {
            Some(b1) => {
                // continue drill
                SymbolTree::attach_deep(token_type, s, character_index + 1, b1);
            }
            None => {
                // attach here
                let remaining_characters = &s[character_index..];
                match SymbolTreeNode::from(remaining_characters, token_type) {
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
