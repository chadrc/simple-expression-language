#[derive(PartialEq, Debug)]
pub enum TokenType {
    Integer,
    PlusSign,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    token_type: TokenType,
    token_str: String,
}

pub fn tokenize(input: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut current_token = String::new();
    for c in input.chars() {
        if c.is_whitespace() {
            continue;
        }

        current_token.push(c);

        if current_token.len() > 0 {
            tokens.push(Token {
                token_type: TokenType::Integer,
                token_str: current_token,
            });
            current_token = String::new();
        }
    }

    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_integer_expression() {
        let input = String::from("4");
        let tokens = tokenize(&input);

        assert_eq!(tokens.len(), 1);

        let only = tokens.get(0).unwrap();

        assert_eq!(only.token_type, TokenType::Integer);
        assert_eq!(only.token_str, "4");
    }

    #[test]
    fn tokenize_addition_expression() {
        let input = String::from("4 + 5");
        let tokens = tokenize(&input);

        assert_eq!(tokens.len(), 3);

        assert_eq!(
            *tokens.get(0).unwrap(),
            Token {
                token_type: TokenType::Integer,
                token_str: String::from("4")
            }
        );
        assert_eq!(
            *tokens.get(1).unwrap(),
            Token {
                token_type: TokenType::PlusSign,
                token_str: String::from("+")
            }
        );

        assert_eq!(
            *tokens.get(2).unwrap(),
            Token {
                token_type: TokenType::Integer,
                token_str: String::from("5")
            }
        );
    }
}
