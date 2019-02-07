
pub fn tokenize(input: &String) -> &String {
    return input;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_integer_expression() {
        let input = &String::from("4");
        let result = tokenize(input);
        assert_eq!(result, "4");
    }
}