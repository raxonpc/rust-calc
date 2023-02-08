mod lexer;
mod parser;

use std::error::Error;
use lexer::*;
use parser::*;
use std::io::{ stdin, Write, stdout };

fn main() {
    let mut buffer = String::new();

    loop {
        print!("> ");
        stdout().flush();
        buffer.clear();
        stdin().read_line(&mut buffer);

        let result = Parser::new(Lexer::new(buffer.trim()).scan().unwrap()).parse().unwrap();
        println!("{}", result);
    }
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

