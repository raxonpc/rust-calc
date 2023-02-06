#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LexingError {
    InvalidOperator,
    InvalidCharacter
}

impl std::fmt::Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use LexingError::*;

        match *self {
            InvalidOperator => write!(f, "Invalid operator"),
            InvalidCharacter => write!(f, "Invalid character")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    pub fn new(c: char) -> Result<Self, LexingError> {
        match c {
            '+' => Ok(Operation::Add),
            '-' => Ok(Operation::Subtract),
            '*' => Ok(Operation::Multiply),
            '/' => Ok(Operation::Divide),
            _ => Err(LexingError::InvalidOperator),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Value(f64),          // 1 2 3 ...
    Operator(Operation), // + - * /
    LeftParent,          // (
    RightParent,         // )
    Eof,                 // EOF
    Equals,
}

pub struct Lexer {
    input: String,
}

fn get_index_from_back<T>(v: &Vec<T>, i: usize) -> Option<&T> {
    // 0 - last element, etc.
    if v.len() >= i + 1 {
        Some(v.get(v.len() - i - 1).unwrap())
    } else { None }
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
                        if let Some(token) = get_index_from_back(&output, 0) {
                            // check the last token (can be '+' or '-' - indicating a positive or a negative value)
                            if *token == Token::Operator(Operation::Subtract) || *token == Token::Operator(Operation::Add) {
                                // check the token before the last token ( if it's a number or ')' )
                                if let Some(token) = get_index_from_back(&output, 1) {
                                    match *token {
                                        Token::Operator(_) | Token::LeftParent => {
                                            number_buf = Some(-(c.to_digit(10).unwrap() as f64));
                                            output.pop();
                                            continue;
                                        }
                                        _ => ()
                                    }
                                }
                            }
                        }
                        number_buf = match number_buf {
                            Some(num) => Some(num * 10.0 + c.to_digit(10).unwrap() as f64),
                            None => Some(c.to_digit(10).unwrap() as f64)
                        };
                        None
                    }
                    '+' | '-' | '*' | '/' => {
                        if let Some(num) = number_buf {
                            output.push(Token::Value(num));
                            number_buf = None;
                        }

                        Some(Token::Operator(Operation::new(c)?))
                    },
                    '(' => {
                        if let Some(num) = number_buf {
                            output.push(Token::Value(num));
                            number_buf = None;
                        }
                        Some(Token::LeftParent)
                    }
                    ')' => {
                        if let Some(num) = number_buf {
                            output.push(Token::Value(num));
                            number_buf = None;
                        }
                        Some(Token::RightParent)
                    },
                    '=' => {
                        if let Some(num) = number_buf {
                            output.push(Token::Value(num));
                            number_buf = None;
                        }
                        Some(Token::Equals)
                    },
                    ' ' => {
                        if let Some(num) = number_buf {
                            output.push(Token::Value(num));
                            number_buf = None;
                        }
                        continue;
                    }
                    _ => return Err(LexingError::InvalidCharacter)
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

