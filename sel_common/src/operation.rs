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
    Symbol,
    Pair,
    List,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equality,
    Inequality,
    LogicalAnd,
    LogicalOr,
    Not,
    LogicalXOR,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXOR,
    BitwiseLeftShift,
    BitwiseRightShift,
    DotAccess,
    Group,
    PipeFirstRight,
    PipeFirstLeft,
    PipeLastRight,
    PipeLastLeft,
    MatchTrue,
    MatchFalse,
    MatchEqual,
    MatchNotEqual,
    MatchLessThan,
    MatchLessThanEqual,
    MatchGreaterThan,
    MatchGreaterThanEqual,
    MatchKeysEqual,
    MatchKeysNotEqual,
    MatchValuesEqual,
    MatchValuesNotEqual,
    MatchContains,
    MatchNotContains,
    None,
}
