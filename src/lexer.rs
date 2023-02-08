#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LexingError {
    InvalidCharacter
}

impl std::fmt::Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use LexingError::*;

        match *self {
            InvalidCharacter => write!(f, "Invalid character")
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Value(f64),       // 1 2 3 ...
    Plus,
    Minus,
    Multiply,
    Slash,
    LeftPar,          // (
    RightPar,         // )
    Eof,              // EOF
}

pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer { input: input.to_string() }
    }

    pub fn scan(&self) -> Result<Vec<Token>, LexingError> {
        let mut output = Vec::<Token>::new();

        let mut iter = self.input.chars();
        let mut number_buf: Option<f64> = None;
        'lexer_loop:
        loop {
            let c = iter.next();
            let token: Option<Token> = match c {
                Some(c) => match c {
                    _ if c.is_numeric() => {
                        number_buf = match number_buf {
                            Some(num) => Some(num * 10.0 + c.to_digit(10).unwrap() as f64),
                            None => Some(c.to_digit(10).unwrap() as f64)
                        };
                        None
                    }
                    '+' | '-' | '*' | '/' | '(' | ')' | '=' | ' ' => {
                        if let Some(num) = number_buf {
                            output.push(Token::Value(num));
                            number_buf = None;
                        }

                        match c {
                            '+' => Some(Token::Plus),
                            '-' => Some(Token::Minus),
                            '*' => Some(Token::Multiply),
                            '/' => Some(Token::Slash),
                            '(' => Some(Token::LeftPar),
                            ')' => Some(Token::RightPar),
                            ' ' => continue,
                            _ => panic!("Should never happen") // never happens
                        }
                    },
                    _ => {
                        return Err(LexingError::InvalidCharacter);
                    }
                }
                None => {
                    if let Some(num) = number_buf {
                        output.push(Token::Value(num));
                        number_buf = None;
                    }
                    output.push(Token::Eof);
                    break 'lexer_loop;
                }
            };

            if let Some(token) = token {
                output.push(token);
            }
        }

        Ok(output)
    }
}

