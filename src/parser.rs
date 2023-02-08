use std::fmt;
use std::ops::{Deref, DerefMut};
use crate::lexer::{ Token };

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ParsingError {
    ExpectedANumber,
    MismatchedParentheses,
    MismatchedMinusSign,
    ExpectedAPrefix,
    ExpectedAnOperator,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use ParsingError::*;

        match *self {
            ExpectedANumber => write!(f, "Expected a number"),
            MismatchedParentheses => write!(f, "Missing ')' parenthesis"),
            MismatchedMinusSign => write!(f, "Mismatched '-' sign"),
            ExpectedAPrefix => write!(f, "Expected a prefix"),
            ExpectedAnOperator => write!(f, "Expected an operator")
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Value(f64),
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
}

struct TokenProvider {
    tokens: Vec<Token>,
    current_index: usize,
}

impl TokenProvider {
    fn new(tokens: Vec<Token>) -> Self {
        TokenProvider {
            tokens,
            current_index: 0
        }
    }

    fn get_next_token(&mut self) -> Option<&Token> {
        self.current_index += 1;
        self.tokens.get(self.current_index - 1)
    }

    fn get_peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.current_index)
    }
}

pub struct Parser {
    token_provider: TokenProvider
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            token_provider: TokenProvider::new(tokens)
        }
    }

    fn precedence(token: &Token) -> i32 {
        match *token {
            Token::Plus | Token::Minus => 10,
            Token:: Multiply | Token::Slash => 20,
            _ => 0
        }
    }

    fn parse_number(&mut self) -> Result<Box<Node>, ParsingError> {
        let token = self.token_provider.get_next_token();
        if let Some(token) = token {
            match token {
                Token::Value(num) => Ok(Box::new(Node::Value(*num))),
                _ => Err(ParsingError::ExpectedANumber)
            }
        } else {
            Err(ParsingError::ExpectedANumber)
        }
    }

    fn parse_prefix(&mut self, left: Token) -> Result<Box<Node>, ParsingError> {
        match left {
            Token::Value(num) => Ok(Box::new(Node::Value(num))),
            Token::LeftPar => {
                let inner = self.parse_expression(0)?;
                if let None = self.token_provider.get_next_token() {
                    Err(ParsingError::MismatchedParentheses)
                } else {
                    Ok(inner)
                }
            },
            Token::Minus => {
                let mut next = self.parse_number().or(Err(ParsingError::MismatchedMinusSign))?;
                if let Node::Value(val) = next.deref_mut() {
                    *val = -*val;
                    Ok(next)
                } else { panic!() }
            },
            _ => Err(ParsingError::ExpectedAPrefix)
        }
    }

    fn parse_infix(&mut self, left: Box<Node>, token: Token) -> Result<Box<Node>, ParsingError> {
        match token {
            Token::Plus => Ok(Box::new(Node::Add(left, self.parse_expression(Self::precedence(&token))?))),
            Token::Minus => Ok(Box::new(Node::Subtract(left, self.parse_expression(Self::precedence(&token))?))),
            Token::Multiply => Ok(Box::new(Node::Multiply(left, self.parse_expression(Self::precedence(&token))?))),
            Token::Slash => Ok(Box::new(Node::Divide(left, self.parse_expression(Self::precedence(&token))?))),
            _ => Err(ParsingError::ExpectedAnOperator)
        }
    }

    fn parse_expression(&mut self, precedence: i32) -> Result<Box<Node>, ParsingError> {
        let token = self.token_provider.get_next_token().ok_or(ParsingError::ExpectedANumber)?.to_owned();
        let mut left = self.parse_prefix( token)?;


        while precedence < Self::precedence(self.token_provider.get_peek_token().ok_or(ParsingError::ExpectedAnOperator)?) {
            let token = self.token_provider.get_next_token().ok_or(ParsingError::ExpectedANumber)?.to_owned();
            left = self.parse_infix(left, token)?;
        }
        Ok(left)
    }

    fn eval_tree(head: &Box<Node>) -> f64 {
        match head.deref() {
            Node::Add(lhs, rhs) => Self::eval_tree(lhs) + Self::eval_tree(rhs),
            Node::Subtract(lhs, rhs) => Self::eval_tree(lhs) - Self::eval_tree(rhs),
            Node::Multiply(lhs, rhs) => Self::eval_tree(lhs) * Self::eval_tree(rhs),
            Node::Divide(lhs, rhs) => Self::eval_tree(lhs) / Self::eval_tree(rhs),
            Node::Value(num) => *num,
            _ => panic!("Tree consists of invalid token")
        }
    }

    pub fn parse(&mut self) -> Result<f64, ParsingError> {
        let head = self.parse_expression(0)?;
        Ok(Self::eval_tree(&head))
    }
}