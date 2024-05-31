use std::fmt;

use lexr::lex_rule;

use crate::{ CalcError, CalcErrorType };

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Variable(String),
    LeftParenthesis,
    RightParenthesis,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Error(CalcError),
}

impl fmt::Display for Token {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            Token::Number(num) => write!(formatter, "Num({num})"),
            Token::Addition => write!(formatter, "Plus"),
            Token::Subtraction => write!(formatter, "Minus"),
            Token::Multiplication => write!(formatter, "Times"),
            Token::Division => write!(formatter, "Divide"),
            _ => write!(formatter, "___"),
        }
    }
}

pub fn parse(input_str: &str) -> Result<Vec<Token>, CalcError> {
    lex_rule! {lex -> Token {
        ws => |_| continue,
        r"[0-9]+([.][0-9]+)?" => |s| {
            Token::Number(s.parse::<f64>().expect("Failed to parse number token"))
        },
        r"\+" => |_| Token::Addition,
        r"\-" => |_| Token::Subtraction,
        r"\*" => |_| Token::Multiplication,
        "/" => |_| Token::Division,
        eof => |_| break,
        _ => |s| {
            println!("Uknown token");
            Token::Error(CalcError{
                error_type: CalcErrorType::SyntaxError,
                msg: format!("Invalid token \'{s}\'").to_string()
            })
        },

    }}
    let tokens = lex(input_str).into_token_vec();
    for token in &tokens {
        if let Token::Error(e) = token {
            return Err(e.clone());
        }
    }
    return Ok(tokens);
}
