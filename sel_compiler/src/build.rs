use crate::find_root::find_root_index;
use crate::group_handling::{correct_group, identifier_call_check, update_group};
use crate::precedence_manager::{PrecedenceManager, LIST_PRECEDENCE, RIGHT_TO_LEFT_PRECEDENCES};
use crate::process_tokens::make_nodes_from_tokenizer;
use crate::resolve_tree::resolve_tree;
use crate::utils::promote_match_lists;
use sel_common::named_expression::NamedExpression;
use sel_common::{DataHeap, Operation, SELContext, SELSubTree, SELTree, SELTreeNode};
use sel_tokenizer::Tokenizer;
use std::collections::{HashMap, HashSet};

pub fn build_tree_from_string(s: &String, context: SELContext) -> SELTree {
    let mut context = context;
    let mut precedence_manager = PrecedenceManager::new();
    let mut tokenizer = Tokenizer::new(s);
    let (
        mut nodes,
        mut data,
        firsts_of_expression,
        annotations,
        documents,
        named_expressions,
        name_spaces,
    ) = make_nodes_from_tokenizer(&mut precedence_manager, &mut tokenizer, &mut context);

    let precedence_groups = precedence_manager.get_group_tiers();

    let mut sub_root_ban_set: HashSet<usize> = HashSet::new();
    let mut sub_trees: Vec<SELSubTree> = vec![];

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

            nodes = promote_match_lists(nodes, group.get_members().get(LIST_PRECEDENCE).unwrap());

            if index != 0 {
                let updated_result = update_group(nodes, group);
                nodes = updated_result.1;
                let group_root = updated_result.0;

                // if this is an expression group
                // create a sub tree

                // find sub roots for this expression
                // by extracting all sub roots that are between
                // group.get_first() ..= group.get_last()
                // add found sub roots to a black set, so they are ignored by later expressions

                let mut group_sub_roots: Vec<usize> = vec![];

                for sub_root in firsts_of_expression.iter() {
                    let sub_root = *sub_root;
                    if group.get_first() <= sub_root
                        && sub_root <= group.get_last()
                        && !sub_root_ban_set.contains(&sub_root)
                    {
                        group_sub_roots.push(sub_root);
                        sub_root_ban_set.insert(sub_root);
                    }
                }

                // if the expression is a single line expression
                // or root is on same line as opening brace
                // root won't be included
                // insert as beginning if not contained
                if !group_sub_roots.contains(&group_root) {
                    group_sub_roots.insert(0, group_root);
                }

                check_set_expression_sub_tree(
                    &mut nodes,
                    &mut data,
                    &mut sub_trees,
                    group_sub_roots,
                    group.get_parent(),
                );
            }
        }
    }

    let named_expressions = named_expressions
        .iter()
        .fold(HashMap::new(), |mut map, value| {
            println!("banning {:?}", value.get_root());
            sub_root_ban_set.insert(value.get_root());

            map.insert(
                value.get_symbol(),
                NamedExpression::new(
                    find_root_index(&nodes, Some(value.get_root())),
                    value.get_symbol(),
                ),
            );
            map
        });

    // firsts of group doesn't contain very first
    // we find this one by starting at 0
    let root = if !sub_root_ban_set.contains(&0) {
        find_root_index(&nodes, None)
    } else {
        // find first sub root that isn't in named_expression or ban set
        let mut root = 0;
        for sub_root in firsts_of_expression.iter() {
            if !sub_root_ban_set.contains(&sub_root) {
                root = *sub_root;
                break;
            }
        }

        // this sub_root has been promoted
        // ban it
        sub_root_ban_set.insert(root);

        find_root_index(&nodes, Some(root))
    };

    // collect remaining roots by transforming firsts of group
    let sub_roots: Vec<usize> = firsts_of_expression
        .iter()
        .filter(|sub_root| !sub_root_ban_set.contains(sub_root))
        .map(|first| find_root_index(&nodes, Some(*first)))
        .collect();

    return SELTree::new(
        root,
        sub_trees,
        sub_roots,
        nodes,
        data,
        context,
        annotations,
        documents,
        named_expressions,
        name_spaces,
    );
}

fn check_set_expression_sub_tree(
    nodes: &mut Vec<SELTreeNode>,
    data: &mut DataHeap,
    sub_trees: &mut Vec<SELSubTree>,
    group_sub_roots: Vec<usize>,
    expression_index: usize,
) {
    nodes
        .get_mut(expression_index)
        .filter(|group_parent| group_parent.get_operation() == Operation::Expression)
        .and_then(|group_parent| {
            sub_trees.push(SELSubTree::new(group_sub_roots));

            let data_index = data.insert_usize(sub_trees.len() - 1);

            group_parent.set_value(data_index);

            Some(true)
        });
}
