#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ParseState {
    NoToken,
    EndOfToken,
    ParsingInteger,
    ParsingDecimal,
    ParsingSingleQuotedString,
    ParsingDoubleQuotedString,
    ParsingIdentifier,
    ParsingNamespace,
    ParsingPrime,
    EscapeCharacter,
    ParsingExclusiveRange,
    ParsingSymbol,
    ParsingDot,
    ParsingUntilEndLine,
}
