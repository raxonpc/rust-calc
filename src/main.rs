mod lexer;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;
    #[test]
    fn should_scan() {
        let input = "2 + (3 * -5) / (9 +- 5)=".to_string();
        let tokens = vec![
            Token::Value(2.0),
            Token::Operator(Operation::Add),
            Token::LeftParent,
            Token::Value(3.0),
            Token::Operator(Operation::Multiply),
            Token::Value(-5.0),
            Token::RightParent,
            Token::Operator(Operation::Divide),
            Token::LeftParent,
            Token::Value(9.0),
            Token::Operator(Operation::Add),
            Token::Value(-5.0),
            Token::RightParent,
            Token::Equals,
            Token::Eof
        ];

        let lexer = Lexer::new(&input);
        assert_eq!(lexer.scan().unwrap(), tokens);
    }
}

