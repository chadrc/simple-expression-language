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

pub struct PrecedenceManager {
    operation_priorities: HashMap<Operation, usize>,
    precedence_buckets: Vec<Vec<usize>>,
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

        let mut precedence_buckets: Vec<Vec<usize>> = vec![];

        precedence_buckets.push(vec![]); // VALUE_PRECEDENCE
        precedence_buckets.push(vec![]); // START_GROUP_PRECEDENCE
        precedence_buckets.push(vec![]); // END_GROUP_PRECEDENCE
        precedence_buckets.push(vec![]); // UNARY_PRECEDENCE
        precedence_buckets.push(vec![]); // RANGE_PRECEDENCE
        precedence_buckets.push(vec![]); // EXPONENTIAL_PRECEDENCE
        precedence_buckets.push(vec![]); // MULTIPLICATION_PRECEDENCE
        precedence_buckets.push(vec![]); // ADDITION_PRECEDENCE
        precedence_buckets.push(vec![]); // RELATIONAL_PRECEDENCE
        precedence_buckets.push(vec![]); // EQUALITY_PRECEDENCE
        precedence_buckets.push(vec![]); // AND_PRECEDENCE
        precedence_buckets.push(vec![]); // OR_PRECEDENCE

        return PrecedenceManager {
            operation_priorities: operation_priorities,
            precedence_buckets: precedence_buckets,
        };
    }

    pub fn get_buckets(&self) -> &Vec<Vec<usize>> {
        return &self.precedence_buckets;
    }

    pub fn get_start_group_bucket(&self) -> &Vec<usize> {
        return self.precedence_buckets.get(START_GROUP_PRECEDENCE).unwrap();
    }

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
        match self.operation_priorities.get(&op) {
            None => (),
            Some(precedence) => match self.precedence_buckets.get_mut(*precedence) {
                None => (),
                Some(bucket) => {
                    bucket.push(index);
                }
            },
        }
    }

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
