use sel_common::Operation;
use std::collections::HashMap;

// lower number means higher priority
const VALUE_PRECEDENCE: usize = 0;
const GROUP_PRECEDENCE: usize = VALUE_PRECEDENCE + 1;
const TRANSFORM_PRECEDENCE: usize = GROUP_PRECEDENCE + 1;
const UNARY_PRECEDENCE: usize = TRANSFORM_PRECEDENCE + 1;
const ACCESS_PRECEDENCE: usize = UNARY_PRECEDENCE + 1;
const RANGE_PRECEDENCE: usize = ACCESS_PRECEDENCE + 1;
const EXPONENTIAL_PRECEDENCE: usize = RANGE_PRECEDENCE + 1;
const MULTIPLICATION_PRECEDENCE: usize = EXPONENTIAL_PRECEDENCE + 1;
const ADDITION_PRECEDENCE: usize = MULTIPLICATION_PRECEDENCE + 1;
const BITSHIFT_PRECEDENCE: usize = ADDITION_PRECEDENCE + 1;
const RELATIONAL_PRECEDENCE: usize = BITSHIFT_PRECEDENCE + 1;
const EQUALITY_PRECEDENCE: usize = RELATIONAL_PRECEDENCE + 1;
const BITWISE_AND_PRECEDENCE: usize = EQUALITY_PRECEDENCE + 1;
const BITWISE_XOR_PRECEDENCE: usize = BITWISE_AND_PRECEDENCE + 1;
const BITWISE_OR_PRECEDENCE: usize = BITWISE_XOR_PRECEDENCE + 1;
const LOGICAL_AND_PRECEDENCE: usize = BITWISE_OR_PRECEDENCE + 1;
const LOGICAL_XOR_PRECEDENCE: usize = LOGICAL_AND_PRECEDENCE + 1;
const LOGICAL_OR_PRECEDENCE: usize = LOGICAL_XOR_PRECEDENCE + 1;
const PAIR_PRECEDENCE: usize = LOGICAL_OR_PRECEDENCE + 1;
const MATCH_PRECEDENCE: usize = PAIR_PRECEDENCE + 1;
pub const LIST_PRECEDENCE: usize = MATCH_PRECEDENCE + 1;
const PIPE_RIGHT_PRECEDENCE: usize = LIST_PRECEDENCE + 1;
const PIPE_LEFT_PRECEDENCE: usize = PIPE_RIGHT_PRECEDENCE + 1;
const STREAM_PRECEDENCE: usize = PIPE_RIGHT_PRECEDENCE + 1;
const COLLECT_INIT_PRECEDENCE: usize = PIPE_RIGHT_PRECEDENCE + 1;
const COLLECT_PRECEDENCE: usize = PIPE_RIGHT_PRECEDENCE + 1;

pub const RIGHT_TO_LEFT_PRECEDENCES: [usize; 1] = [PAIR_PRECEDENCE];

#[derive(Debug)]
pub struct PrecedenceGroup {
    parent: usize,
    first: usize,
    last: usize,
    count: usize,
    members: Vec<Vec<usize>>,
}

impl PrecedenceGroup {
    pub fn new(parent: usize) -> Self {
        let mut members: Vec<Vec<usize>> = vec![];

        members.push(vec![]); // VALUE_PRECEDENCE
        members.push(vec![]); // GROUP_PRECEDENCE
        members.push(vec![]); // TRANSFORM_PRECEDENCE
        members.push(vec![]); // ACCESS_PRECEDENCE
        members.push(vec![]); // UNARY_PRECEDENCE
        members.push(vec![]); // RANGE_PRECEDENCE
        members.push(vec![]); // EXPONENTIAL_PRECEDENCE
        members.push(vec![]); // MULTIPLICATION_PRECEDENCE
        members.push(vec![]); // ADDITION_PRECEDENCE
        members.push(vec![]); // BITSHIFT_PRECEDENCE
        members.push(vec![]); // RELATIONAL_PRECEDENCE
        members.push(vec![]); // EQUALITY_PRECEDENCE
        members.push(vec![]); // BITWISE_AND_PRECEDENCE
        members.push(vec![]); // BITWISE_XOR_PRECEDENCE
        members.push(vec![]); // BITWISE_OR_PRECEDENCE
        members.push(vec![]); // LOGICAL_AND_PRECEDENCE
        members.push(vec![]); // LOGICAL_XOR_PRECEDENCE
        members.push(vec![]); // LOGICAL_OR_PRECEDENCE
        members.push(vec![]); // PAIR_PRECEDENCE
        members.push(vec![]); // LIST_PRECEDENCE
        members.push(vec![]); // MATCH_PRECEDENCE
        members.push(vec![]); // PIPE_RIGHT_PRECEDENCE
        members.push(vec![]); // PIPE_LEFT_PRECEDENCE
        members.push(vec![]); // STREAM_LEFT_PRECEDENCE
        members.push(vec![]); // COLLECT_LEFT_PRECEDENCE
        members.push(vec![]); // COLLECT_INIT_LEFT_PRECEDENCE

        return PrecedenceGroup {
            first: 0,
            last: 0,
            count: 0,
            members,
            parent,
        };
    }

    pub fn get_members(&self) -> &Vec<Vec<usize>> {
        return &self.members;
    }

    pub fn get_first(&self) -> usize {
        return self.first;
    }

    pub fn get_last(&self) -> usize {
        return self.last;
    }

    pub fn get_parent(&self) -> usize {
        return self.parent;
    }
}

pub struct PrecedenceManager {
    operation_priorities: HashMap<Operation, usize>,
    precedence_groups: Vec<Vec<PrecedenceGroup>>,
    current_tier: usize,
}

impl PrecedenceManager {
    pub fn new() -> Self {
        let mut operation_priorities = HashMap::new();

        operation_priorities.insert(Operation::Touch, VALUE_PRECEDENCE);
        operation_priorities.insert(Operation::Input, VALUE_PRECEDENCE);
        operation_priorities.insert(Operation::CurrentResult, VALUE_PRECEDENCE);

        operation_priorities.insert(Operation::Group, GROUP_PRECEDENCE);
        operation_priorities.insert(Operation::AssociativeList, GROUP_PRECEDENCE);
        operation_priorities.insert(Operation::Expression, GROUP_PRECEDENCE);

        operation_priorities.insert(Operation::Transform, TRANSFORM_PRECEDENCE);

        operation_priorities.insert(Operation::DotAccess, ACCESS_PRECEDENCE);

        operation_priorities.insert(Operation::Symbol, UNARY_PRECEDENCE);
        operation_priorities.insert(Operation::Not, UNARY_PRECEDENCE);
        operation_priorities.insert(Operation::Negation, UNARY_PRECEDENCE);

        operation_priorities.insert(Operation::ExclusiveRange, RANGE_PRECEDENCE);
        operation_priorities.insert(Operation::InclusiveRange, RANGE_PRECEDENCE);

        operation_priorities.insert(Operation::Exponential, EXPONENTIAL_PRECEDENCE);

        operation_priorities.insert(Operation::Multiplication, MULTIPLICATION_PRECEDENCE);
        operation_priorities.insert(Operation::Division, MULTIPLICATION_PRECEDENCE);
        operation_priorities.insert(Operation::IntegerDivision, MULTIPLICATION_PRECEDENCE);
        operation_priorities.insert(Operation::Modulo, MULTIPLICATION_PRECEDENCE);

        operation_priorities.insert(Operation::Addition, ADDITION_PRECEDENCE);
        operation_priorities.insert(Operation::Subtraction, ADDITION_PRECEDENCE);

        operation_priorities.insert(Operation::BitwiseRightShift, BITSHIFT_PRECEDENCE);
        operation_priorities.insert(Operation::BitwiseLeftShift, BITSHIFT_PRECEDENCE);

        operation_priorities.insert(Operation::GreaterThan, RELATIONAL_PRECEDENCE);
        operation_priorities.insert(Operation::GreaterThanOrEqual, RELATIONAL_PRECEDENCE);
        operation_priorities.insert(Operation::LessThan, RELATIONAL_PRECEDENCE);
        operation_priorities.insert(Operation::LessThanOrEqual, RELATIONAL_PRECEDENCE);

        operation_priorities.insert(Operation::Equality, EQUALITY_PRECEDENCE);
        operation_priorities.insert(Operation::Inequality, EQUALITY_PRECEDENCE);
        operation_priorities.insert(Operation::KeysEqual, EQUALITY_PRECEDENCE);
        operation_priorities.insert(Operation::KeysNotEqual, EQUALITY_PRECEDENCE);
        operation_priorities.insert(Operation::ValuesEqual, EQUALITY_PRECEDENCE);
        operation_priorities.insert(Operation::ValuesNotEqual, EQUALITY_PRECEDENCE);
        operation_priorities.insert(Operation::Contains, EQUALITY_PRECEDENCE);
        operation_priorities.insert(Operation::NotContains, EQUALITY_PRECEDENCE);

        operation_priorities.insert(Operation::BitwiseAnd, BITWISE_AND_PRECEDENCE);

        operation_priorities.insert(Operation::BitwiseXOR, BITWISE_XOR_PRECEDENCE);

        operation_priorities.insert(Operation::BitwiseOr, BITWISE_OR_PRECEDENCE);

        operation_priorities.insert(Operation::LogicalAnd, LOGICAL_AND_PRECEDENCE);

        operation_priorities.insert(Operation::LogicalXOR, LOGICAL_XOR_PRECEDENCE);

        operation_priorities.insert(Operation::LogicalOr, LOGICAL_OR_PRECEDENCE);

        operation_priorities.insert(Operation::Pair, PAIR_PRECEDENCE);

        operation_priorities.insert(Operation::MatchTrue, MATCH_PRECEDENCE);
        operation_priorities.insert(Operation::MatchFalse, MATCH_PRECEDENCE);

        operation_priorities.insert(Operation::List, LIST_PRECEDENCE);
        operation_priorities.insert(Operation::MatchList, LIST_PRECEDENCE);

        operation_priorities.insert(Operation::PipeFirstRight, PIPE_RIGHT_PRECEDENCE);
        operation_priorities.insert(Operation::PipeLastRight, PIPE_RIGHT_PRECEDENCE);

        operation_priorities.insert(Operation::PipeFirstLeft, PIPE_LEFT_PRECEDENCE);
        operation_priorities.insert(Operation::PipeLastLeft, PIPE_LEFT_PRECEDENCE);

        operation_priorities.insert(Operation::Stream, STREAM_PRECEDENCE);

        operation_priorities.insert(Operation::CollectInit, COLLECT_INIT_PRECEDENCE);

        operation_priorities.insert(Operation::Collect, COLLECT_PRECEDENCE);

        let mut root_group_tier: Vec<PrecedenceGroup> = vec![];
        root_group_tier.push(PrecedenceGroup::new(0));

        let mut precedence_groups: Vec<Vec<PrecedenceGroup>> = vec![];
        precedence_groups.push(root_group_tier);

        return PrecedenceManager {
            operation_priorities,
            precedence_groups,
            current_tier: 0,
        };
    }

    fn last_group_in_current_tier(&self) -> usize {
        return self.precedence_groups.get(self.current_tier).unwrap().len() - 1;
    }

    fn current_group(&self) -> &PrecedenceGroup {
        return self
            .precedence_groups
            .get(self.current_tier)
            .unwrap()
            .get(self.last_group_in_current_tier())
            .unwrap();
    }

    fn current_group_mut(&mut self) -> &mut PrecedenceGroup {
        let last_group = self.last_group_in_current_tier();
        return self
            .precedence_groups
            .get_mut(self.current_tier)
            .unwrap()
            .get_mut(last_group)
            .unwrap();
    }

    pub fn get_group_tiers(&self) -> &Vec<Vec<PrecedenceGroup>> {
        return &self.precedence_groups;
    }

    #[allow(dead_code)]
    // will be used eventually in error handling
    pub fn get_current_tier(&self) -> usize {
        return self.current_tier;
    }

    pub fn add_index_with_operation(&mut self, op: Operation, index: usize) {
        let precedence = *self.operation_priorities.get(&op).unwrap();

        if self.current_group().count == 0 {
            self.current_group_mut().first = index;
        }
        self.current_group_mut().last = index;
        match self.current_group_mut().members.get_mut(precedence) {
            None => (),
            Some(bucket) => {
                bucket.push(index);
            }
        }

        self.current_group_mut().count += 1;
    }

    pub fn start_group(&mut self) {
        // parent of new group is always the last group of the current tier
        let last_of_parent_group = self.current_group().last;

        // update and fetch new current tier
        self.current_tier += 1;
        let tier = match self.precedence_groups.get_mut(self.current_tier) {
            Some(t) => t,
            None => {
                // need to create tier
                self.precedence_groups.push(vec![]);

                // now exists, safe to unwrap
                self.precedence_groups.get_mut(self.current_tier).unwrap()
            }
        };

        // add new group
        tier.push(PrecedenceGroup::new(last_of_parent_group));
    }

    pub fn end_group(&mut self) {
        // ending a group always means returning to previous tier
        self.current_tier -= 1;
    }

    // TODO: make static along with op priorities struct
    pub fn is_lower(&self, op: Operation, relative_to: Operation) -> bool {
        let op_precedence = match self.operation_priorities.get(&op) {
            None => return false,
            Some(precedence) => precedence,
        };

        let relative_precedence = match self.operation_priorities.get(&relative_to) {
            None => return false,
            Some(precedence) => precedence,
        };

        // higher precedence's have lower priority
        op_precedence > relative_precedence
    }

    // TODO: make static along with op priorities struct
    pub fn is_op_value_precedence(&self, op: Operation) -> bool {
        return match self.operation_priorities.get(&op) {
            None => false,
            Some(precedence) => match precedence {
                &VALUE_PRECEDENCE => true,
                _ => false,
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sel_common::Operation;

    #[test]
    fn create() {
        PrecedenceManager::new();
    }

    #[test]
    fn add_operation() {
        let mut manager = PrecedenceManager::new();
        manager.add_index_with_operation(Operation::Touch, 0);

        let group_tiers = manager.get_group_tiers();
        let group = group_tiers.get(0).unwrap().get(0).unwrap();
        let buckets = group.get_members();
        let value_bucket = buckets.get(VALUE_PRECEDENCE).unwrap();

        assert_eq!(value_bucket.len(), 1);
        assert_eq!(*value_bucket.get(0).unwrap(), 0);
    }

    #[test]
    fn single_grouping() {
        let mut manager = PrecedenceManager::new();
        manager.add_index_with_operation(Operation::Touch, 0);

        manager.start_group();

        manager.add_index_with_operation(Operation::Touch, 1);
        manager.add_index_with_operation(Operation::Touch, 2);

        manager.end_group();

        let group_tiers = manager.get_group_tiers();

        assert_eq!(group_tiers.len(), 2);
        assert_eq!(group_tiers.get(1).unwrap().get(0).unwrap().parent, 0);
        assert_eq!(
            group_tiers
                .get(1)
                .unwrap()
                .get(0)
                .unwrap()
                .members
                .get(VALUE_PRECEDENCE)
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn group_range() {
        let mut manager = PrecedenceManager::new();
        manager.add_index_with_operation(Operation::Touch, 0);

        manager.start_group();

        manager.add_index_with_operation(Operation::Touch, 1);
        manager.add_index_with_operation(Operation::Touch, 2);
        manager.add_index_with_operation(Operation::Touch, 3);
        manager.add_index_with_operation(Operation::Touch, 4);

        manager.end_group();

        manager.add_index_with_operation(Operation::Touch, 5);
        manager.add_index_with_operation(Operation::Touch, 6);

        let group = manager.get_group_tiers().get(1).unwrap().get(0).unwrap();

        assert_eq!(group.parent, 0);
        assert_eq!(group.first, 1);
        assert_eq!(group.last, 4);
    }

    #[test]
    fn multiple_groups_in_tier() {
        let mut manager = PrecedenceManager::new();
        manager.add_index_with_operation(Operation::Touch, 0);

        manager.start_group();

        manager.add_index_with_operation(Operation::Touch, 1);
        manager.add_index_with_operation(Operation::Touch, 2);

        manager.end_group();

        manager.add_index_with_operation(Operation::Touch, 3);
        manager.add_index_with_operation(Operation::Touch, 4);

        manager.start_group();

        manager.add_index_with_operation(Operation::Touch, 5);
        manager.add_index_with_operation(Operation::Touch, 6);

        manager.end_group();

        manager.start_group();

        manager.add_index_with_operation(Operation::Touch, 7);
        manager.add_index_with_operation(Operation::Touch, 8);

        manager.end_group();

        let group_tiers = manager.get_group_tiers();

        let first_tier = group_tiers.get(1).unwrap();

        assert_eq!(first_tier.len(), 3);

        let first_group = first_tier.get(0).unwrap();
        let second_group = first_tier.get(1).unwrap();
        let third_group = first_tier.get(2).unwrap();

        assert_eq!(first_group.parent, 0);
        assert_eq!(second_group.parent, 4);
        assert_eq!(third_group.parent, 4);
    }

    #[test]
    fn multiple_group_tiers() {
        let mut manager = PrecedenceManager::new();
        manager.add_index_with_operation(Operation::Touch, 0);

        manager.start_group();

        manager.add_index_with_operation(Operation::Touch, 1);
        manager.add_index_with_operation(Operation::Touch, 2);

        manager.start_group();

        manager.add_index_with_operation(Operation::Touch, 3);
        manager.add_index_with_operation(Operation::Touch, 4);

        manager.start_group();

        manager.add_index_with_operation(Operation::Touch, 5);
        manager.add_index_with_operation(Operation::Touch, 6);

        manager.end_group();
        manager.end_group();
        manager.end_group();

        let group_tiers = manager.get_group_tiers();

        // 3 explicit tiers + implicit first tier
        assert_eq!(group_tiers.len(), 4);

        let first_tier = group_tiers.get(1).unwrap();
        let second_tier = group_tiers.get(2).unwrap();
        let third_tier = group_tiers.get(3).unwrap();

        assert_eq!(first_tier.len(), 1);
        assert_eq!(second_tier.len(), 1);
        assert_eq!(third_tier.len(), 1);

        let first_group = first_tier.get(0).unwrap();
        let second_group = second_tier.get(0).unwrap();
        let third_group = third_tier.get(0).unwrap();

        assert_eq!(first_group.parent, 0);
        assert_eq!(second_group.parent, 2);
        assert_eq!(third_group.parent, 4);
    }

    #[test]
    fn complete_groups() {
        let mut manager = PrecedenceManager::new();
        manager.add_index_with_operation(Operation::Touch, 0);

        manager.start_group();

        manager.add_index_with_operation(Operation::Touch, 1);
        manager.add_index_with_operation(Operation::Touch, 2);

        manager.start_group();

        manager.add_index_with_operation(Operation::Touch, 3);
        manager.add_index_with_operation(Operation::Touch, 4);

        manager.start_group();

        manager.add_index_with_operation(Operation::Touch, 5);
        manager.add_index_with_operation(Operation::Touch, 6);

        manager.end_group();
        manager.end_group();
        manager.end_group();

        // all groups have been closed if current tier is 0
        assert_eq!(manager.get_current_tier(), 0);;
    }

    #[test]
    fn incomplete_groups() {
        let mut manager = PrecedenceManager::new();
        manager.add_index_with_operation(Operation::Touch, 0);

        manager.start_group();

        manager.add_index_with_operation(Operation::Touch, 1);
        manager.add_index_with_operation(Operation::Touch, 2);

        manager.start_group();

        manager.add_index_with_operation(Operation::Touch, 3);
        manager.add_index_with_operation(Operation::Touch, 4);

        manager.start_group();

        manager.add_index_with_operation(Operation::Touch, 5);
        manager.add_index_with_operation(Operation::Touch, 6);

        manager.end_group();

        // one or more groups are still open
        assert_eq!(manager.get_current_tier(), 2);
    }
}
