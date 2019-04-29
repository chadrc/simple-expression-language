#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    Integer,
    Decimal,
    SingleQuotedString,
    DoubleQuotedString,
    ExclusiveRange,
    InclusiveRange,
    Boolean,
    PlusSign,
    MinusSign,
    MultiplicationSign,
    DivisionSign,
    IntegerDivisionSign,
    ModulusSign,
    ExponentialSign,
    BitwiseAndSign,
    BitwiseXorSign,
    BitwiseOrSign,
    BitwiseLeftShiftSign,
    BitwiseRightShiftSign,
    TransformationSign,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    LogicalAnd,
    LogicalOr,
    NotSign,
    LogicalXOR,
    Unit,
    Input,
    CurrentResult,
    Dot,
    Comma,
    Identifier,
    Symbol,
    Pair,
    StartGroup,
    EndGroup,
    LineEnd,
    Unknown,
}
