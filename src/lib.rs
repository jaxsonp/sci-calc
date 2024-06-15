use core::num;
use std::fmt;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

pub mod context;
use context::*;

pub struct CSVParser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct CalculatorParser;

pub fn calculate(input_str: &str, ctx: &mut Context) -> Result<f64, CalcError> {

	let assignment_var: Option<String>;
	let result: Result<f64, CalcError> = {
		let mut pairs = match CalculatorParser::parse(Rule::target, input_str) {
			Ok(p) => { p }
			Err(e) => {
				return Err(CalcError {
					error_type: CalcErrorType::SyntaxError,
					msg: e.to_string()
				});
			}
		};
		let tree = match pairs.next() {
			Some(r) => { r }
			None => {
				return Err(CalcError {
					error_type: CalcErrorType::ParserError,
					msg: "Failed to generate syntax tree".to_string()
				});
			}
		};
		// extracting assignment information from parsing result
		let mut root: Pair<Rule>;
		match &tree.as_rule() {
			Rule::assignment => {
				let inner = tree.into_inner();
				let span = inner.clone().into_iter().next().unwrap().as_span();
				assignment_var = Some(input_str[span.start()..span.end()].to_string());
				root = inner.into_iter().nth(2).unwrap();
			}
			Rule::add_expr => {
				assignment_var = None;
				root = tree;
			}
			_ => {
				return Err(CalcError {
					error_type: CalcErrorType::ParserError,
					msg: "Invalid syntax tree (invalid root)".to_string()
				});
			}
		}

		fn evaluate_val(pair: Pair<Rule>, input_str: &str, ctx: &Context) -> Result<f64, CalcError> {
			match pair.as_rule() {
				Rule::NUM => {
					let span = pair.as_span();
					let num_str = &input_str[span.start()..span.end()];
					return Ok(num_str.parse::<f64>().expect("Failed to parse number"));
				}
				Rule::IDENT => {
					let span = pair.as_span();
					let name = &input_str[span.start()..span.end()];
					return ctx.lookup_var(name.to_string());
				}
				_ => {
					match parse_tree(pair, input_str, &ctx) {
						Ok(n) => { Ok(n) }
						Err(e) => {
							return Err(e);
						}
					}
				}
			}
		}

		// recursively parsing AST
		fn parse_tree(pair: Pair<Rule>, input_str: &str, ctx: &Context) -> Result<f64, CalcError> {
			let mut inner = pair.into_inner();
			let lhs_pair = inner.next().unwrap();
			let lhs: f64 = match evaluate_val(lhs_pair, input_str, ctx) {
				Ok(n) => { n }
				Err(e) => {
					return Err(e);
				}
			};
			let operator: Rule;
			match inner.next() {
				Some(pair) => {
					match pair.as_rule() {
						Rule::PLUS | Rule::MINUS | Rule::MULTIPLY | Rule::DIVIDE | Rule::CARET => { 
							operator = pair.as_rule();
						}
						_ => {
							return Err(CalcError {
								error_type: CalcErrorType::ParserError,
								msg: "Invalid syntax tree (expecting operator)".to_string()
							})
						}
					}
				}
				None => {
					// this pair is not an expression, returning
					return Ok(lhs);
				}
			}
			let rhs: f64 = match inner.next() {
				Some(rhs_pair) => {
					match evaluate_val(rhs_pair, input_str, ctx) {
						Ok(n) => { n }
						Err(e) => {
							return Err(e);
						}
					}
				}
				None => {
					return Err(CalcError {
						error_type: CalcErrorType::ParserError,
						msg: "Invalid syntax tree (missing RHS)".to_string()
					})
				}
			};
			// doin da math
			let output = match operator {
				Rule::PLUS => {
					lhs + rhs
				}
				Rule::MINUS => {
					lhs - rhs
				}
				Rule::MULTIPLY => {
					lhs * rhs
				}
				Rule::DIVIDE => {
					lhs / rhs
				}
				Rule::CARET => {
					lhs.powf(rhs)
				}
				_ => { unreachable!(); }
			};

			Ok(output)
		}
		parse_tree(root, input_str, &ctx)
	};
	match &result {
		Ok(n) => {
			if let Some(var_name) = assignment_var {
				if let Err(e) = ctx.assign_var(var_name, *n) {
					return Err(e);
				}
			}
			ctx.history.push(HistEntry::new(input_str.to_string(), Some(n.clone())));
			
		}
		Err(_) => {
			ctx.history.push(HistEntry::new(input_str.to_string(), None));
		}
	};
	result
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalcErrorType {
	SyntaxError,
	ParserError,
	UndefinedVariable,
	AssignmentError,
}
impl fmt::Display for CalcErrorType {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Self::SyntaxError => write!(formatter, "Syntax error"),
			Self::ParserError => write!(formatter, "Parser error"),
			Self::UndefinedVariable => write!(formatter, "Undefined variable"),
			Self::AssignmentError => write!(formatter, "Assignment error"),
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
