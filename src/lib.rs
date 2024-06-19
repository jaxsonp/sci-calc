use std::fmt;

pub mod context;
use context::*;

mod parser;

mod lexer;

pub fn calculate(input_str: &str, ctx: &mut Context) -> Result<f64, CalcError> {

	let tokens = match lexer::tokenize(input_str) {
		Ok(tokens) => { tokens }
		Err(e) => { return Err(e); }
	};
	for t in tokens { print!("{t} "); }
	println!();

	parser::parse();
	Ok(0.0)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Ident(String),
	Literal(f64),
	LParen,
	RParen,
	Plus,
	Minus,
	Multiply,
	Divide,
	Percent,
	Caret,
	Exclamation,
	Comma,
}
impl fmt::Display for Token {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let s = match self {
			Self::Ident(name) => { format!("\"{name}\"") },
			Self::Literal(num) => { format!("{num}") },
			Self::LParen => { String::from("(") },
			Self::RParen => { String::from(")") },
			Self::Plus => { String::from("+") },
			Self::Minus => { String::from("-") },
			Self::Multiply => { String::from("*") },
			Self::Divide => { String::from("/") },
			Self::Percent => { String::from("%") },
			Self::Caret => { String::from("^") },
			Self::Exclamation => { String::from("!") },
			Self::Comma => { String::from(",") },
		};
		write!(formatter, "[{s}]")
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalcErrorType {
	SyntaxError,
	ParserError,
	UndefinedIdentifier,
	AssignmentError,
	ArgumentError,
	CalculationError,
}
impl fmt::Display for CalcErrorType {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		write!(formatter, "{}", match *self {
			Self::SyntaxError => { "Syntax error" },
			Self::ParserError => { "Parser error" },
			Self::UndefinedIdentifier => { "Undefined identifier" },
			Self::AssignmentError => { "Assignment error" },
			Self::ArgumentError => { "Argument error" },
			Self::CalculationError => { "Calculation error" },
		})
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
