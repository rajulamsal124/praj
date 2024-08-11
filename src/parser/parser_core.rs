use crate::lexer::token::Token;
use crate::parser::ast::{Expr, BinOp, Literal, ParseError};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token(&[Token::Plus, Token::Minus]) {
            let operator = match self.previous().unwrap() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => return Err(ParseError::UnexpectedToken),
            };

            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token(&[Token::Star, Token::Slash]) {
            let operator = match self.previous().unwrap() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                _ => return Err(ParseError::UnexpectedToken),
            };

            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    // fn unary(&mut self) -> Result<Expr, ParseError> {
    //     if self.match_token(&[Token::Minus]) {
    //         let right = self.unary()?;
    //         return Ok(Expr::Binary(
    //             Box::new(Expr::Literal(Literal::Number(0.0))),
    //             BinOp::Sub,
    //             Box::new(right),
    //         ));
    //     }
    
    //     self.primary()
    // }
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[Token::Minus]) {
            let right = self.unary()?;
            return Ok(Expr::Binary(
                Box::new(Expr::Literal(Literal::Number(0.0))), // Introduces 0.0 only for unary negation
                BinOp::Sub,
                Box::new(right),
            ));
        }
    
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if let Some(token) = self.advance() {
            match token {
                Token::Number(value) => Ok(Expr::Literal(Literal::Number(*value))),
                Token::StringLiteral(value) => Ok(Expr::Literal(Literal::String(value.clone()))),
                Token::True => Ok(Expr::Literal(Literal::Boolean(true))),
                Token::False => Ok(Expr::Literal(Literal::Boolean(false))),
                Token::Identifier(name) => Ok(Expr::Variable(name.clone())),
                Token::LeftParen => {
                    let expr = self.expression()?;
                    if self.match_token(&[Token::RightParen]) {
                        Ok(expr)
                    } else {
                        Err(ParseError::UnexpectedToken)
                    }
                }
                _ => {
                    // Print the unexpected token for debugging
                    println!("Unexpected token: {:?}", token);
                    Err(ParseError::UnexpectedToken)
                }
            }
        } else {
            Err(ParseError::UnexpectedEOF)
        }
    }
    
    

    fn match_token(&mut self, types: &[Token]) -> bool {
        if let Some(token) = self.peek() {
            if types.contains(token) {
                self.advance();
                return true;
            }
        }
        false
    }

   

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Option<&Token> {
        if self.current == 0 {
            None
        } else {
            self.tokens.get(self.current - 1)
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn is_at_end(&self) -> bool {
        self.peek() == Some(&Token::EOF)
    }
}
