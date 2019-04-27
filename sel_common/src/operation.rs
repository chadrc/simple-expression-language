#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Operation {
    Touch,
    Input,
    CurrentResult,
    Addition,
    Subtraction,
    Multiplication,
    Modulo,
    Division,
    IntegerDivision,
    Exponential,
    Negation,
    ExclusiveRange,
    InclusiveRange,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equality,
    Inequality,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    LogicalXOR,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXOR,
    BitwiseNot,
    BitwiseLeftShift,
    BitwiseRightShift,
    Group,
    Symbol,
    Pair,
    None,
}
