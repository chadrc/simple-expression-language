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

#[cfg(test)]
mod tests {
    use super::*;

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
