mod parse_state;
mod symbol_tree;
mod token;
mod token_type;
mod tokenizer;

pub use token::Token;
pub use token_type::TokenType;
pub use tokenizer::Tokenizer;

#[cfg(test)]
mod tests {
    use super::*;
    use token::Token;
    use token_type::TokenType;

    #[test]
    fn tokenize_integer_expression() {
        let tokens: Vec<Token> = tokens_from_str("4");

        assert_eq!(tokens.len(), 1);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "4");
    }

    #[test]
    fn tokenize_two_digit_integer() {
        let tokens: Vec<Token> = tokens_from_str("43");

        assert_eq!(tokens.len(), 1);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "43");
    }

    #[test]
    fn tokenize_integer_dot_identifier() {
        let tokens: Vec<Token> = tokens_from_str("3.field");

        assert_eq!(tokens.len(), 3);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "3");
        assert_token(tokens.get(1).unwrap(), TokenType::Dot, ".");
        assert_token(tokens.get(2).unwrap(), TokenType::Identifier, "field");
    }

    #[test]
    fn tokenize_decimal_number() {
        let tokens: Vec<Token> = tokens_from_str("3.14");

        assert_eq!(tokens.len(), 1);
        assert_token(tokens.get(0).unwrap(), TokenType::Decimal, "3.14");
    }

    #[test]
    fn tokenize_decimal_number_starting_dot() {
        let tokens: Vec<Token> = tokens_from_str(".01");

        assert_eq!(tokens.len(), 1);
        assert_token(tokens.get(0).unwrap(), TokenType::Decimal, ".01");
    }

    #[test]
    fn tokenize_string_single_quote() {
        let tokens: Vec<Token> = tokens_from_str("'Hello World'");

        assert_eq!(tokens.len(), 1);
        assert_token(
            tokens.get(0).unwrap(),
            TokenType::SingleQuotedString,
            "Hello World",
        );
    }

    #[test]
    fn tokenize_string_single_quote_with_escape() {
        let tokens: Vec<Token> = tokens_from_str("'Hello\\' World'");

        assert_eq!(tokens.len(), 1);
        assert_token(
            tokens.get(0).unwrap(),
            TokenType::SingleQuotedString,
            "Hello' World",
        );
    }

    #[test]
    fn tokenize_string_double_quote() {
        let tokens: Vec<Token> = tokens_from_str("\"Hello World\"");

        assert_eq!(tokens.len(), 1);
        assert_token(
            tokens.get(0).unwrap(),
            TokenType::DoubleQuotedString,
            "Hello World",
        );
    }

    #[test]
    fn tokenize_string_double_quote_with_escape() {
        let tokens: Vec<Token> = tokens_from_str("\"Hello\\\" World\"");

        assert_eq!(tokens.len(), 1);
        assert_token(
            tokens.get(0).unwrap(),
            TokenType::DoubleQuotedString,
            "Hello\" World",
        );
    }

    #[test]
    fn tokenize_exclusive_range() {
        let tokens: Vec<Token> = tokens_from_str("1..10");

        assert_eq!(tokens.len(), 3);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "1");
        assert_token(tokens.get(1).unwrap(), TokenType::ExclusiveRange, "..");
        assert_token(tokens.get(2).unwrap(), TokenType::Integer, "10");
    }

    #[test]
    fn tokenize_exclusive_range_open_begin() {
        let tokens: Vec<Token> = tokens_from_str("..10");

        assert_eq!(tokens.len(), 2);
        assert_token(tokens.get(0).unwrap(), TokenType::ExclusiveRange, "..");
        assert_token(tokens.get(1).unwrap(), TokenType::Integer, "10");
    }

    #[test]
    fn tokenize_exclusive_range_open_end() {
        let tokens: Vec<Token> = tokens_from_str("1..");

        assert_eq!(tokens.len(), 2);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "1");
        assert_token(tokens.get(1).unwrap(), TokenType::ExclusiveRange, "..");
    }

    #[test]
    fn tokenize_exclusive_range_open() {
        let tokens: Vec<Token> = tokens_from_str("..");

        assert_eq!(tokens.len(), 1);
        assert_token(tokens.get(0).unwrap(), TokenType::ExclusiveRange, "..");
    }

    #[test]
    fn tokenize_inclusive_range() {
        let tokens: Vec<Token> = tokens_from_str("1...10");

        assert_eq!(tokens.len(), 3);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "1");
        assert_token(tokens.get(1).unwrap(), TokenType::InclusiveRange, "...");
        assert_token(tokens.get(2).unwrap(), TokenType::Integer, "10");
    }

    #[test]
    fn tokenize_inclusive_range_open_begin() {
        let tokens: Vec<Token> = tokens_from_str("...10");

        assert_eq!(tokens.len(), 2);
        assert_token(tokens.get(0).unwrap(), TokenType::InclusiveRange, "...");
        assert_token(tokens.get(1).unwrap(), TokenType::Integer, "10");
    }

    #[test]
    fn tokenize_inclusive_range_open_end() {
        let tokens: Vec<Token> = tokens_from_str("1...");

        assert_eq!(tokens.len(), 2);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "1");
        assert_token(tokens.get(1).unwrap(), TokenType::InclusiveRange, "...");
    }

    #[test]
    fn tokenize_inclusive_range_open() {
        let tokens: Vec<Token> = tokens_from_str("...");

        assert_eq!(tokens.len(), 1);
        assert_token(tokens.get(0).unwrap(), TokenType::InclusiveRange, "...");
    }

    #[test]
    fn tokenize_unit() {
        let tokens: Vec<Token> = tokens_from_str("()");

        assert_eq!(tokens.len(), 1);
        assert_token(tokens.get(0).unwrap(), TokenType::Unit, "()");
    }

    #[test]
    fn tokenize_boolean_true() {
        let tokens: Vec<Token> = tokens_from_str("true");

        assert_eq!(tokens.len(), 1);
        assert_token(tokens.get(0).unwrap(), TokenType::Boolean, "true");
    }

    #[test]
    fn tokenize_boolean_false() {
        let tokens: Vec<Token> = tokens_from_str("false");

        assert_eq!(tokens.len(), 1);
        assert_token(tokens.get(0).unwrap(), TokenType::Boolean, "false");
    }

    #[test]
    fn tokenize_addition_expression() {
        let tokens: Vec<Token> = tokens_from_str("4 + 5");

        assert_eq!(tokens.len(), 3);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "4");
        assert_token(tokens.get(1).unwrap(), TokenType::PlusSign, "+");
        assert_token(tokens.get(2).unwrap(), TokenType::Integer, "5");
    }

    #[test]
    fn tokenize_addition_expression_no_space() {
        assert_4_5_binary_operation("+", TokenType::PlusSign);
    }

    #[test]
    fn tokenize_subtraction() {
        assert_4_5_binary_operation("-", TokenType::MinusSign);
    }

    #[test]
    fn tokenize_multiplication() {
        assert_4_5_binary_operation("*", TokenType::MultiplicationSign);
    }

    #[test]
    fn tokenize_division() {
        assert_4_5_binary_operation("/", TokenType::DivisionSign);
    }

    #[test]
    fn tokenize_integer_division() {
        assert_4_5_binary_operation("//", TokenType::IntegerDivisionSign);
    }

    #[test]
    fn tokenize_modulus() {
        assert_4_5_binary_operation("%", TokenType::ModulusSign);
    }

    #[test]
    fn tokenize_exponential() {
        assert_4_5_binary_operation("**", TokenType::ExponentialSign);
    }

    #[test]
    fn tokenize_transformation_operator() {
        assert_4_5_binary_operation("`", TokenType::TransformationSign);
    }

    #[test]
    fn tokenize_bitwise_or() {
        assert_4_5_binary_operation("|", TokenType::BitwiseOrSign);
    }

    #[test]
    fn tokenize_bitwise_and() {
        assert_4_5_binary_operation("&", TokenType::BitwiseAndSign);
    }

    #[test]
    fn tokenize_bitwise_xor() {
        assert_4_5_binary_operation("^", TokenType::BitwiseXorSign);
    }

    #[test]
    fn tokenize_bitwise_left_shift() {
        assert_4_5_binary_operation("<<", TokenType::BitwiseLeftShiftSign);
    }

    #[test]
    fn tokenize_bitwise_right_shift() {
        assert_4_5_binary_operation(">>", TokenType::BitwiseRightShiftSign);
    }

    #[test]
    fn tokenize_negation() {
        let tokens: Vec<Token> = tokens_from_str("-2");

        assert_eq!(tokens.len(), 2);
        assert_token(tokens.get(0).unwrap(), TokenType::MinusSign, "-");
        assert_token(tokens.get(1).unwrap(), TokenType::Integer, "2");
    }

    #[test]
    fn tokenize_equality() {
        assert_4_5_binary_operation("==", TokenType::Equal);
    }

    #[test]
    fn tokenize_inequality() {
        assert_4_5_binary_operation("!=", TokenType::NotEqual);
    }

    #[test]
    fn tokenize_greater_than() {
        assert_4_5_binary_operation(">", TokenType::GreaterThan);
    }

    #[test]
    fn tokenize_greater_than_or_equal() {
        assert_4_5_binary_operation(">=", TokenType::GreaterThanOrEqual);
    }

    #[test]
    fn tokenize_less_than() {
        assert_4_5_binary_operation("<", TokenType::LessThan);
    }

    #[test]
    fn tokenize_less_than_or_equal() {
        assert_4_5_binary_operation("<=", TokenType::LessThanOrEqual);
    }

    #[test]
    fn tokenize_logical_and() {
        assert_4_5_binary_operation("&&", TokenType::LogicalAnd);
    }

    #[test]
    fn tokenize_logical_xor() {
        assert_4_5_binary_operation("^^", TokenType::LogicalXOR);
    }

    #[test]
    fn tokenize_logical_or() {
        assert_4_5_binary_operation("||", TokenType::LogicalOr);
    }

    #[test]
    fn tokenize_logical_not() {
        let tokens: Vec<Token> = tokens_from_str("!true");

        assert_eq!(tokens.len(), 2);
        assert_token(tokens.get(0).unwrap(), TokenType::NotSign, "!");
        assert_token(tokens.get(1).unwrap(), TokenType::Boolean, "true");
    }

    #[test]
    fn tokenize_input_symbol() {
        let tokens: Vec<Token> = tokens_from_str("$");
        assert_token(tokens.get(0).unwrap(), TokenType::Input, "$");
    }

    #[test]
    fn tokenize_result_symbol() {
        let tokens: Vec<Token> = tokens_from_str("?");
        assert_token(tokens.get(0).unwrap(), TokenType::CurrentResult, "?");
    }

    #[test]
    fn tokenize_dot_symbol() {
        let tokens: Vec<Token> = tokens_from_str(".");
        assert_token(tokens.get(0).unwrap(), TokenType::Dot, ".");
    }

    #[test]
    fn tokenize_comma_symbol() {
        let tokens: Vec<Token> = tokens_from_str(",");
        assert_token(tokens.get(0).unwrap(), TokenType::Comma, ",");
    }

    #[test]
    fn tokenize_new_line() {
        let tokens = tokens_from_str("\n");
        assert_token(tokens.get(0).unwrap(), TokenType::LineEnd, "\n");
    }

    #[test]
    fn tokenize_start_group() {
        let tokens = tokens_from_str("(");
        assert_token(tokens.get(0).unwrap(), TokenType::StartGroup, "(");
    }

    #[test]
    fn tokenize_end_group() {
        let tokens = tokens_from_str(")");
        assert_token(tokens.get(0).unwrap(), TokenType::EndGroup, ")");
    }

    #[test]
    fn tokenize_symbol_operator() {
        let tokens = tokens_from_str(":");
        assert_token(tokens.get(0).unwrap(), TokenType::Symbol, ":");
    }

    #[test]
    fn tokenize_pair_operator() {
        let tokens = tokens_from_str("=");
        assert_token(tokens.get(0).unwrap(), TokenType::Pair, "=");
    }

    #[test]
    fn tokenize_identifier() {
        let tokens = tokens_from_str("value");
        assert_token(tokens.get(0).unwrap(), TokenType::Identifier, "value");
    }

    #[test]
    fn tokenize_identifier_like_true() {
        let tokens = tokens_from_str("trueish");
        assert_token(tokens.get(0).unwrap(), TokenType::Identifier, "trueish");
    }

    #[test]
    fn tokenize_identifier_like_false() {
        let tokens = tokens_from_str("falseish");
        assert_token(tokens.get(0).unwrap(), TokenType::Identifier, "falseish");
    }

    #[test]
    fn tokenize_identifier_with_underscores() {
        let tokens = tokens_from_str("_my_value");
        assert_token(tokens.get(0).unwrap(), TokenType::Identifier, "_my_value");
    }

    #[test]
    fn tokenize_identifier_with_nums() {
        let tokens = tokens_from_str("value12");
        assert_token(tokens.get(0).unwrap(), TokenType::Identifier, "value12");
    }

    #[test]
    fn all_token_count() {
        let tokens = tokens_from_str(
            "100 3.13 true 'string' \"string\"\n1..10 1...10 + - * / % ** == != < <= > >= && || ^^ | & ^ ~ << >> ` !false () $ ?",
        );
        assert_eq!(tokens.len(), 39);
    }

    // Test utils
    fn tokens_from_str(s: &str) -> Vec<Token> {
        let input = String::from(s);
        let tokenizer = Tokenizer::new(&input);
        return tokenizer.collect();
    }

    fn assert_token(token: &Token, token_type: TokenType, token_str: &str) {
        assert_eq!(token.get_token_type(), token_type);
        assert_eq!(token.get_token_str(), token_str);
    }

    fn assert_4_5_binary_operation(op: &str, op_token_type: TokenType) {
        let tokens: Vec<Token> =
            tokens_from_str(&("4".to_owned() + &op.to_owned() + &"5".to_owned()));

        assert_eq!(tokens.len(), 3);
        assert_token(tokens.get(0).unwrap(), TokenType::Integer, "4");
        assert_token(tokens.get(1).unwrap(), op_token_type, op);
        assert_token(tokens.get(2).unwrap(), TokenType::Integer, "5");
    }
}
