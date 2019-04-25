use sel_common::Operation;
use std::collections::HashMap;

// lower number means higher priority
const VALUE_PRECEDENCE: usize = 0;
const START_GROUP_PRECEDENCE: usize = VALUE_PRECEDENCE + 1;
const END_GROUP_PRECEDENCE: usize = START_GROUP_PRECEDENCE + 1;
const UNARY_PRECEDENCE: usize = END_GROUP_PRECEDENCE + 1;
const RANGE_PRECEDENCE: usize = UNARY_PRECEDENCE + 1;
const EXPONENTIAL_PRECEDENCE: usize = RANGE_PRECEDENCE + 1;
const MULTIPLICATION_PRECEDENCE: usize = EXPONENTIAL_PRECEDENCE + 1;
const ADDITION_PRECEDENCE: usize = MULTIPLICATION_PRECEDENCE + 1;
const RELATIONAL_PRECEDENCE: usize = ADDITION_PRECEDENCE + 1;
const EQUALITY_PRECEDENCE: usize = RELATIONAL_PRECEDENCE + 1;
const AND_PRECEDENCE: usize = EQUALITY_PRECEDENCE + 1;
const OR_PRECEDENCE: usize = AND_PRECEDENCE + 1;

pub struct PrecedenceGroup {
    parent: usize,
    first: usize,
    last: usize,
    members: Vec<Vec<usize>>,
}

impl PrecedenceGroup {
    pub fn new(parent: usize, first: usize) -> Self {
        let mut members: Vec<Vec<usize>> = vec![];

        members.push(vec![]); // VALUE_PRECEDENCE
        members.push(vec![]); // START_GROUP_PRECEDENCE
        members.push(vec![]); // END_GROUP_PRECEDENCE
        members.push(vec![]); // UNARY_PRECEDENCE
        members.push(vec![]); // RANGE_PRECEDENCE
        members.push(vec![]); // EXPONENTIAL_PRECEDENCE
        members.push(vec![]); // MULTIPLICATION_PRECEDENCE
        members.push(vec![]); // ADDITION_PRECEDENCE
        members.push(vec![]); // RELATIONAL_PRECEDENCE
        members.push(vec![]); // EQUALITY_PRECEDENCE
        members.push(vec![]); // AND_PRECEDENCE
        members.push(vec![]); // OR_PRECEDENCE

        return PrecedenceGroup {
            first,
            last: first,
            members,
            parent,
        };
    }
}

pub struct PrecedenceManager {
    operation_priorities: HashMap<Operation, usize>,
    //    precedence_buckets: Vec<Vec<usize>>,
    precedence_groups: Vec<Vec<PrecedenceGroup>>,
    current_tier: usize,
    current_group: usize,
}

impl PrecedenceManager {
    pub fn new() -> Self {
        let mut operation_priorities = HashMap::new();

        operation_priorities.insert(Operation::Touch, VALUE_PRECEDENCE);
        operation_priorities.insert(Operation::Input, VALUE_PRECEDENCE);
        operation_priorities.insert(Operation::CurrentResult, VALUE_PRECEDENCE);

        operation_priorities.insert(Operation::StartGroup, START_GROUP_PRECEDENCE);
        operation_priorities.insert(Operation::EndGroup, END_GROUP_PRECEDENCE);

        operation_priorities.insert(Operation::LogicalNot, UNARY_PRECEDENCE);
        operation_priorities.insert(Operation::Negation, UNARY_PRECEDENCE);

        operation_priorities.insert(Operation::ExclusiveRange, RANGE_PRECEDENCE);
        operation_priorities.insert(Operation::InclusiveRange, RANGE_PRECEDENCE);

        operation_priorities.insert(Operation::Exponential, EXPONENTIAL_PRECEDENCE);

        operation_priorities.insert(Operation::Multiplication, MULTIPLICATION_PRECEDENCE);
        operation_priorities.insert(Operation::Division, MULTIPLICATION_PRECEDENCE);
        operation_priorities.insert(Operation::Modulo, MULTIPLICATION_PRECEDENCE);

        operation_priorities.insert(Operation::Addition, ADDITION_PRECEDENCE);
        operation_priorities.insert(Operation::Subtraction, ADDITION_PRECEDENCE);

        operation_priorities.insert(Operation::GreaterThan, RELATIONAL_PRECEDENCE);
        operation_priorities.insert(Operation::GreaterThanOrEqual, RELATIONAL_PRECEDENCE);
        operation_priorities.insert(Operation::LessThan, RELATIONAL_PRECEDENCE);
        operation_priorities.insert(Operation::LessThanOrEqual, RELATIONAL_PRECEDENCE);

        operation_priorities.insert(Operation::Equality, EQUALITY_PRECEDENCE);
        operation_priorities.insert(Operation::Inequality, EQUALITY_PRECEDENCE);

        operation_priorities.insert(Operation::LogicalAnd, AND_PRECEDENCE);

        operation_priorities.insert(Operation::LogicalOr, OR_PRECEDENCE);

        let mut root_group_tier: Vec<PrecedenceGroup> = vec![];
        root_group_tier.push(PrecedenceGroup::new(0, 0));

        let mut precedence_groups: Vec<Vec<PrecedenceGroup>> = vec![];
        precedence_groups.push(root_group_tier);

        return PrecedenceManager {
            operation_priorities,
            //            precedence_buckets,
            precedence_groups,
            current_tier: 0,
            current_group: 0,
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

    pub fn get_buckets(&self) -> &Vec<Vec<usize>> {
        return &self.current_group().members;
    }

    pub fn get_group_tiers(&self) -> &Vec<Vec<PrecedenceGroup>> {
        return &self.precedence_groups;
    }

    pub fn get_start_group_bucket(&self) -> &Vec<usize> {
        return self
            .current_group()
            .members
            .get(START_GROUP_PRECEDENCE)
            .unwrap();
    }

    pub fn get_end_group_bucket(&self) -> &Vec<usize> {
        return self
            .current_group()
            .members
            .get(END_GROUP_PRECEDENCE)
            .unwrap();
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

    pub fn add_index_with_operation(&mut self, op: Operation, index: usize) {
        let precedence = *self.operation_priorities.get(&op).unwrap();

        self.current_group_mut().last = index;
        match self.current_group_mut().members.get_mut(precedence) {
            None => (),
            Some(bucket) => {
                bucket.push(index);
            }
        }
    }

    pub fn start_group(&mut self) {
        // parent of new group is always the last group of the current tier
        let parent = self.last_group_in_current_tier();
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
        tier.push(PrecedenceGroup::new(last_of_parent_group, 0));
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

        let buckets = manager.get_buckets();
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
}
