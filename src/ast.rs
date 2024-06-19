use std::fmt;

pub enum Expr {
    Num(f64),
    Op(Box<Expr>, Operation, Box<Expr>),
	Fac(Box<Expr>),
}
impl fmt::Display for Expr {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let s = match self {
			Self::Num(n) => n.to_string(),
			Self::Op(lhs, op, rhs) => format!("({lhs}{op} {rhs})"),
			Self::Fac(n) => format!("{n}!")
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