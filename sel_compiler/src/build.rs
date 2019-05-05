use crate::find_root::find_root_index;
use crate::group_handling::{correct_group, identifier_call_check, update_group};
use crate::precedence_manager::{PrecedenceManager, RIGHT_TO_LEFT_PRECEDENCES};
use crate::process_tokens::make_nodes_from_tokenizer;
use crate::resolve_tree::resolve_tree;
use sel_common::{SELContext, SELTree};
use sel_tokenizer::Tokenizer;

fn build(s: &String, context: SELContext) -> SELTree {
    let mut context = context;
    let mut precedence_manager = PrecedenceManager::new();
    let mut tokenizer = Tokenizer::new(s);
    let (mut nodes, data, firsts_of_expression) =
        make_nodes_from_tokenizer(&mut precedence_manager, &mut tokenizer, &mut context);

    let precedence_groups = precedence_manager.get_group_tiers();

    for (index, tier) in precedence_groups.iter().enumerate().rev() {
        for group in tier.iter().rev() {
            // base tier doesn't need any correction
            // before or after creating the tree

            if index != 0 {
                nodes = correct_group(nodes, group);
            }

            // check identifiers first
            nodes = identifier_call_check(nodes, group.get_members().get(0).unwrap());

            // skip value and group precedences
            for (i, bucket) in group.get_members().iter().skip(2).enumerate() {
                // plus two to compensate for skipped precedences
                let right_to_left = RIGHT_TO_LEFT_PRECEDENCES.contains(&(i + 2));
                nodes = resolve_tree(&precedence_manager, nodes, &bucket, right_to_left);
            }

            if index != 0 {
                nodes = update_group(nodes, group);
            }
        }
    }

    // firsts of group doesn't contain very first
    // we find this one by starting at 0
    let root = find_root_index(&nodes, None);

    // collect remaining roots by transforming firsts of group
    let sub_roots: Vec<usize> = firsts_of_expression
        .iter()
        .map(|first| find_root_index(&nodes, Some(*first)))
        .collect();

    return SELTree::new(root, sub_roots, nodes, data, context);
}

pub fn build_tree_from_string(s: &String, context: SELContext) -> SELTree {
    return build(s, context);
}
