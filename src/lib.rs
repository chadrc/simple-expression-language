pub fn tokenize(input: &String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];

    let mut current_token = String::new();
    for c in input.chars() {
        if c.is_whitespace() {
            continue;
        }

        current_token.push(c);

        if current_token.len() > 0 {
            tokens.push(current_token);
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
        let result = tokenize(&input);
        assert_eq!(*result.get(0).unwrap(), "4");
    }

    #[test]
    fn tokenize_addition_expression() {
        let input = String::from("4 + 5");
        let result = tokenize(&input);

        assert_eq!(result.len(), 3);
        assert_eq!(*result.get(0).unwrap(), "4");
        assert_eq!(*result.get(1).unwrap(), "+");
        assert_eq!(*result.get(2).unwrap(), "5");
    }
}
