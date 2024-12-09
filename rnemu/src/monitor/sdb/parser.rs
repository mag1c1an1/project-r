use std::fmt;

use super::{
    expr::{BinaryOp, BinaryOpTy, Expr, UnaryOp},
    tokenizer::{Token, TokenType},
};

pub enum ParseError {
    UnexpectedToken(Token),
    TokenMismatch {
        expected: TokenType,
        found: Token,
        maybe_on_err_string: Option<String>,
    },
    InvalidTokenInUnaryOp {
        token_type: TokenType,
    },
    InvalidTokenInBinaryOp {
        token_type: TokenType,
    },
    ExpectedExpression {
        token_type: TokenType,
    },
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ParseError::UnexpectedToken(tok) => write!(f, "Unexpected token {:?}", tok.ty,),
            ParseError::TokenMismatch {
                maybe_on_err_string,
                expected,
                found,
            } => {
                write!(f, "Expected token {:?} but found {:?}", expected, found.ty,)?;
                if let Some(on_err_string) = maybe_on_err_string {
                    write!(f, ": {}", on_err_string)?;
                }
                fmt::Result::Ok(())
            }
            ParseError::ExpectedExpression { token_type } => {
                write!(f, "Expected expression, but found token {:?} ", token_type)
            }
            ParseError::InvalidTokenInUnaryOp { token_type } => {
                write!(f, "Invalid token in unary op {:?}", token_type)
            }
            ParseError::InvalidTokenInBinaryOp { token_type } => {
                write!(f, "Invalid token in binary op {:?} ", token_type)
            }
        }
    }
}

#[derive(Default)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression_statement()
    }

    fn expression_statement(&mut self) -> Result<Expr, ParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        self.or()
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.matches(TokenType::Or) {
            let right = self.and()?;
            expr = Expr::Logical(Box::new(expr), super::expr::LogicalOp::Or, Box::new(right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.matches(TokenType::And) {
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(expr), super::expr::LogicalOp::And, Box::new(right));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_one_of(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator_token = self.previous().clone();
            let right = Box::new(self.comparison()?);

            let binop_maybe = Parser::op_token_to_binop(&operator_token);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = Expr::Binary(left, binop, right);
                }
                Err(err) => return Err(err),
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.addition()?;

        // while self.match_one_of(vec![
        //     scanner::TokenType::Greater,
        //     scanner::TokenType::GreaterEqual,
        //     scanner::TokenType::Less,
        //     scanner::TokenType::LessEqual,
        // ]) {
        //     let operator_token = self.previous().clone();
        //     let right = Box::new(self.addition()?);
        //     let binop_maybe = Parser::op_token_to_binop(&operator_token);

        //     match binop_maybe {
        //         Ok(binop) => {
        //             let left = Box::new(expr);
        //             expr = expr::Expr::Binary(left, binop, right);
        //         }
        //         Err(err) => return Err(err),
        //     }
        // }
        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.multiplication()?;

        while self.match_one_of(vec![TokenType::Minus, TokenType::Plus]) {
            let operator_token = self.previous().clone();
            let right = Box::new(self.multiplication()?);
            let binop_maybe = Parser::op_token_to_binop(&operator_token);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = Expr::Binary(left, binop, right);
                }
                Err(err) => return Err(err),
            }
        }
        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_one_of(vec![TokenType::Slash, TokenType::Star]) {
            let operator_token = self.previous().clone();
            let right = Box::new(self.unary()?);
            let binop_maybe = Parser::op_token_to_binop(&operator_token);

            match binop_maybe {
                Ok(binop) => {
                    let left = Box::new(expr);
                    expr = Expr::Binary(left, binop, right);
                }
                Err(err) => return Err(err),
            }
        }
        Ok(expr)
    }
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_one_of(vec![TokenType::Star, TokenType::Minus]) {
            let operator_token = self.previous().clone();
            let right = Box::new(self.unary()?);
            let unary_op_maybe = Parser::op_token_to_unary_op(&operator_token);

            return match unary_op_maybe {
                Ok(unary_op) => Ok(Expr::Unary(unary_op, right)),
                Err(err) => Err(err),
            };
        }
        self.primary()
    }

    fn _call(&mut self) -> Result<Expr, ParseError> {
        todo!()
    }

    fn consume(&mut self, tok: TokenType, on_err_str: &str) -> Result<&Token, ParseError> {
        if self.check(tok) {
            return Ok(self.advance());
        }
        Err(ParseError::TokenMismatch {
            expected: tok,
            found: self.peek().clone(),
            maybe_on_err_string: Some(on_err_str.into()),
        })
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.matches(TokenType::Number) {
            match &self.previous().literal {
                Some(super::tokenizer::Literal::Number(n)) => {
                    return Ok(Expr::Literal(super::expr::Literal::Number(*n)))
                }
                Some(l) => panic!(
                    "internal error in parser: when parsing number, found literal {:?}",
                    l
                ),
                None => panic!("internal error in parser: when parsing number, found no literal"),
            }
        }
        if self.matches(TokenType::Identifier) {
            match &self.previous().literal {
                Some(super::tokenizer::Literal::Identifier(s)) => {
                    return Ok(Expr::Literal(super::expr::Literal::Register(s.clone())))
                }
                Some(l) => panic!(
                    "internal error in parser: when parsing identifier, found literal {:?}",
                    l
                ),
                None => panic!("internal error in parser: when parsing number, found no literal"),
            }
        }
        if self.matches(TokenType::LeftParen) {
            let expr = Box::new(self.expression()?);
            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
            return Ok(Expr::Grouping(expr));
        }
        Err(ParseError::ExpectedExpression {
            token_type: self.peek().ty,
        })
    }

    fn op_token_to_unary_op(tok: &Token) -> Result<UnaryOp, ParseError> {
        match tok.ty {
            TokenType::Minus => Ok(UnaryOp {
                ty: super::expr::UnaryOpTy::Minus,
            }),
            TokenType::Star => Ok(UnaryOp {
                ty: super::expr::UnaryOpTy::Deref,
            }),
            _ => Err(ParseError::InvalidTokenInUnaryOp { token_type: tok.ty }),
        }
    }

    fn op_token_to_binop(tok: &Token) -> Result<BinaryOp, ParseError> {
        match tok.ty {
            TokenType::Minus => Ok(BinaryOp {
                ty: BinaryOpTy::Minus,
            }),
            TokenType::Plus => Ok(BinaryOp {
                ty: BinaryOpTy::Plus,
            }),
            TokenType::Slash => Ok(BinaryOp {
                ty: BinaryOpTy::Div,
            }),
            TokenType::Star => Ok(BinaryOp {
                ty: BinaryOpTy::Mul,
            }),
            TokenType::BangEqual => Ok(BinaryOp {
                ty: BinaryOpTy::NotEqual,
            }),
            TokenType::EqualEqual => Ok(BinaryOp {
                ty: BinaryOpTy::EqualEqual,
            }),
            _ => Err(ParseError::InvalidTokenInBinaryOp { token_type: tok.ty }),
        }
    }

    fn match_one_of(&mut self, types: Vec<TokenType>) -> bool {
        for ty in types.iter() {
            if self.matches(*ty) {
                return true;
            }
        }
        false
    }

    fn matches(&mut self, ty: TokenType) -> bool {
        if self.check(ty) {
            self.advance();
            return true;
        }
        false
    }

    fn check(&self, ty: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().ty == ty
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ty == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}

// parse tokens to expr
pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    let mut p = Parser {
        tokens,
        ..Default::default()
    };
    let expr_or_err = p.parse();
    match expr_or_err {
        Ok(expr) => {
            if !p.is_at_end() {
                let tok = &p.tokens[p.current];
                Err(ParseError::UnexpectedToken(tok.clone()))
            } else {
                Ok(expr)
            }
        }
        Err(e) => Err(e),
    }
}
