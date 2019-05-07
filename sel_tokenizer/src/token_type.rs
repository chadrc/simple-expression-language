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
    BackTick,
    Equal,
    NotEqual,
    KeysEqual,
    KeysNotEqual,
    ValuesEqual,
    ValuesNotEqual,
    Contains,
    NotContains,
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
    StartAssociativeList,
    EndAssociativeList,
    StartExpressionBlock,
    EndExpressionBlock,
    LineEnd,
    CommentAnnotation,
    DocumentAnnotation,
    Annotation,
    PipeFirstRight,
    PipeFirstLeft,
    PipeLastRight,
    PipeLastLeft,
    Partial,
    MatchTrue,
    MatchFalse,
    Stream,
    Collect,
    SeedCollect,
    Unknown,
}
