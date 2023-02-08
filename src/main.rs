mod lexer;
mod parser;

use lexer::*;
use parser::*;

fn main() {
    let input = "2 + 2 * 2".to_string();
    let lexer = Lexer::new(&input);
    let tokens = lexer.scan().unwrap();
    let mut parser = Parser::new(tokens);
    let tree = parser.parse().unwrap();

    dbg!(tree);
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;
    #[test]
    fn should_scan() {
        let input = "2 + (3 * -5) / (9 + -5)".to_string();
        let tokens = vec![
            Token::Value(2.0),
            Token::Plus,
            Token::LeftPar,
            Token::Value(3.0),
            Token::Multiply,
            Token::Minus,
            Token::Value(5.0),
            Token::RightPar,
            Token::Slash,
            Token::LeftPar,
            Token::Value(9.0),
            Token::Plus,
            Token::Minus,
            Token::Value(5.0),
            Token::RightPar,
            Token::Eof
        ];

        let lexer = Lexer::new(&input);
        assert_eq!(lexer.scan().unwrap(), tokens);
    }
}

