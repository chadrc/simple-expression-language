#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ParseState {
    NoToken,
    EndOfToken,
    ParsingInteger,
    ParsingDecimal,
    ParsingSingleQuotedString,
    ParsingDoubleQuotedString,
    EscapeCharacter,
    ParsingExclusiveRange,
    ParsingSymbol,
}
