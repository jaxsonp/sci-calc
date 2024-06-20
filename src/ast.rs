use std::fmt;

use crate::CalcError;

pub enum Expr {
    Num(f64),
    Op(Box<Expr>, Operation, Box<Expr>),
	Func(String, Vec<Box<Expr>>),
	Var(String),
	Fac(Box<Expr>),
	Err(CalcError)
}
impl fmt::Display for Expr {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let s = match self {
			Self::Num(n) => n.to_string(),
			Self::Op(lhs, op, rhs) => format!("({lhs}{op}{rhs})"),
			Self::Func(name, args) => {
				let mut arg_list = String::new();
				for e in args.iter() {
					arg_list += e.to_string().as_str();
					arg_list.push(',');
				}
				arg_list.pop();
				format!("{name}({arg_list})")
			},
			Self::Var(name) => format!("{name}"),
			Self::Fac(n) => format!("{n}!"),
			Self::Err(e) => format!("{e}"),
		};
		write!(formatter, "{}", s)
	}
}

pub enum Operation {
	Add,
    Sub,
    Mul,
    Div,
	FloorDiv,
	Mod,
	Exp,
}
impl fmt::Display for Operation {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let s = match *self {
			Self::Add => "+",
			Self::Sub => "-",
			Self::Mul => "*",
			Self::Div => "/",
			Self::FloorDiv => "//",
			Self::Mod => "%",
			Self::Exp => "^",
		};
		write!(formatter, "{}", s)
	}
}