use core::fmt;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, one_of},
    combinator::{map, map_res, recognize},
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated},
    IResult, Parser,
};

use crate::common::SWord;

// <expr> ::= <decimal-number>
//   | <hexadecimal-number>    # 以"0x"开头
//   | <reg_name>              # 以"$"开头
//   | "(" <expr> ")"
//   | <expr> "+" <expr>
//   | <expr> "-" <expr>
//   | <expr> "*" <expr>
//   | <expr> "/" <expr>
//   | <expr> "==" <expr>
//   | <expr> "!=" <expr>
//   | <expr> "&&" <expr>
//   | "*" <expr>              # 指针解引用

// Expression tokens.

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    Minus,
    Plus,
    Slash,
    Star,

    // One or two character tokens.
    BangEqual,
    EqualEqual,
    // Literals.
    Identifier,
    Number,

    // Keywords.
    Eof,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Identifier(String),
    Number(i32),
}

#[derive(Clone, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub literal: Option<Literal>,
}

impl Token {
    fn new(ty: TokenType, literal: Option<Literal>) -> Self {
        Self { ty, literal }
    }
    fn operator(c: char) -> Self {
        match c {
            '+' => Token::new(TokenType::Plus, None),
            '-' => Token::new(TokenType::Minus, None),
            '*' => Token::new(TokenType::Star, None),
            '/' => Token::new(TokenType::Slash, None),
            _ => panic!("wrong operator"),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token {{ ty: {:?}, literal: {:?} }}",
            self.ty, self.literal,
        )
    }
}

fn hex_value(input: &str) -> IResult<&str, Token> {
    map_res(
        preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(many1(terminated(
                one_of("_0123456789abcdefABCDEF"),
                many0(char('_')),
            ))),
        ),
        |out: &str| {
            SWord::from_str_radix(&str::replace(&out, "_", ""), 16)
                .map(|v| Token::new(TokenType::Number, Some(Literal::Number(v))))
        },
    )
    .parse(input)
}

fn decimal_value(input: &str) -> IResult<&str, Token> {
    map_res(
        recognize(many1(terminated(one_of("_0123456789"), many0(char('_'))))),
        |out: &str| {
            SWord::from_str_radix(&str::replace(&out, "_", ""), 10)
                .map(|v| Token::new(TokenType::Number, Some(Literal::Number(v))))
        },
    )
    .parse(input)
}

fn skip_witherspace(input: &str) -> IResult<&str, ()> {
    map(multispace0, |_| ())(input)
}
fn operator(input: &str) -> IResult<&str, Token> {
    map(
        alt((char('+'), char('-'), char('*'), char('/'))),
        Token::operator,
    )(input)
}
fn parens(input: &str) -> IResult<&str, Token> {
    alt((
        map(char('('), |_| Token::new(TokenType::LeftParen, None)),
        map(char(')'), |_| Token::new(TokenType::RightParen, None)),
    ))(input)
}

fn token(input: &str) -> IResult<&str, Token> {
    delimited(
        skip_witherspace,
        alt((hex_value, decimal_value, parens, operator)),
        skip_witherspace,
    )(input)
}

// pub enum TokenizeError {

// }

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    match many0(token)(input) {
        Ok((remain, mut o)) => {
            println!("Tokens: {:?}", o);
            if !remain.trim().is_empty() {
                return Err(format!("Tokenize falied, remain :{}", remain));
            }
            // add eof
            o.push(Token::new(TokenType::Eof, None));
            Ok(o)
        }
        Err(e) => Err(format!("Failed: {:?}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}

    #[test]
    fn hex_value_test() {
        let (i, o) = hex_value("0x_8_88_88_").unwrap();
        assert_eq!(
            o,
            Token::new(TokenType::Number, Some(Literal::Number(0x88888)))
        );
        // let (i, o) = hex_value("0x0").unwrap();
        // assert_eq!(o, 0);
        // let (i, o) = hex_value("0x_").unwrap();
        // assert_eq!(o, 0);
    }

    #[test]
    fn dec_value_test() {
        // let (i, o) = decimal_value("8888").unwrap();
        // assert_eq!(o, 8888);
    }

    #[test]
    fn tokenize_test() {
        let input = "3 + 5";
        match tokenize(input) {
            Ok((remain, o)) => {
                println!("Tokens: {:?}", o);
                if !remain.trim().is_empty() {
                    println!("remain: {:?}", remain)
                }
            }
            Err(e) => println!("Failed: {:?}", e),
        }
    }
}
