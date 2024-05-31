use std::fmt;

mod lexer;
use lexer::Token;

pub fn calculate(input_str: &str) -> Result<f64, CalcError> {
    // lexical analysis
    let tokens: Vec<Token>;
    match lexer::parse(input_str) {
        Ok(t) => {
            tokens = t;
        }
        Err(e) => {
            return Err(e);
        }
    }
    for token in tokens {
        print!("{}>", token);
    }
    println!();
    Ok(1.0)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalcErrorType {
    SyntaxError,
}
impl fmt::Display for CalcErrorType {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CalcErrorType::SyntaxError => write!(formatter, "Syntax error"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalcError {
    error_type: CalcErrorType,
    msg: String,
}
impl fmt::Display for CalcError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}: {}\n", self.error_type, self.msg)
    }
}
