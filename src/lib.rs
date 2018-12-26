struct EvalResult {
    value: i64,
}

enum TokenType {
    Unknown,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    BooleanLiteral,
}

struct Token {
    token_type: TokenType,
    token_str: String,
}

fn make_number_token(input: &String) -> Token {
    let token_string: &str = match input.find(" ") {
        Some(index) => {
            // make slice from start to found space
            &input[..index]
        }
        None => {
            // end of input, entire thing is number string
            &input[..]
        }
    };

    // try integer cast
    match token_string.parse::<i64>() {
        Ok(num) => {
            // success make Token
            Token { token_type: TokenType::IntegerLiteral, token_str: String::from(token_string) }
        }
        Err(e) => {
            // try float case
            match token_string.parse::<f64>() {
                Ok(num) => {
                    Token { token_type: TokenType::FloatLiteral, token_str: String::from(token_string) }
                }
            }
        }
    }
}

fn get_token(input: &String) -> Token {
    let mut end_index = -1;

    let mut current_index = 0;

    // don't care about surrounding white space
    let trimmed = input.trim();

    if trimmed.len() == 0 {
        // panic?
    }

    // breaking above if out of bounds
    // so safe to just unwrap here
    let c: &str = trimmed.get(0.into()).unwrap();

    // make type assumption based on first character
    // make a token from it
    if c {
        return make_number_token(input)
    }

    Token { token_type: TokenType::Unknown, token_str: String::from("") }
}

pub fn eval(input: &String) -> EvalResult {
    let token = get_token(input);
    let value = match token.token_type {
        TokenType::IntegerLiteral => token.token_str.parse::<i64>().unwrap(),
        _ => -1,
    };

    EvalResult { value }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_integer_literal_returns_value() {
        let result = eval(&String::from("4"));
        assert_eq!(result.value, 4);
    }

    #[test]
    fn number_float_literal_returns_value() {
        let result = eval(&String::from("3.14"));
        panic!("Not implemented");
    }

    #[test]
    fn number_decimal_no_leading_literal_returns_value() {
        let result = eval(&String::from(".33334"));
        panic!("Not implemented");
    }

    #[test]
    fn double_quote_string_literal_returns_value() {
        let result = eval(&String::from("\"test expression string\""));
        panic!("Not implemented");
    }

    #[test]
    fn single_quote_string_literal_returns_value() {
        let result = eval(&String::from("'test expression string'"));
        panic!("Not implemented");
    }

    #[test]
    fn back_tick_string_literal_returns_value() {
        let result = eval(&String::from("`test expression string`"));
        panic!("Not implemented");
    }

    #[test]
    fn boolean_true_literal_returns_value() {
        let result = eval(&String::from("true"));
        panic!("Not implemented");
    }

    #[test]
    fn boolean_false_literal_returns_value() {
        let result = eval(&String::from("false"));
        panic!("Not implemented");
    }
}