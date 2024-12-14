use std::fmt::{Debug, Display};

use super::{binary_expr::BinaryExpression, unary_expr::UnaryExpression};
use crate::numeric::Numeric;

#[derive(Debug, Clone)]
pub enum Expression {
	Program(Vec<Expression>),
	LiteralNumber(Numeric),
	LiteralString(Box<str>),
	Identifier(Box<str>),
	Unary(UnaryExpression),
	Binary(BinaryExpression),
	Unit,
}

impl Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use Expression::*;

		match self {
			Program(exprs) => exprs.iter().try_for_each(|e| writeln!(f, "{e}")),
			LiteralNumber(num) => Display::fmt(num, f),
			LiteralString(st) => Debug::fmt(st, f),
			Identifier(ident) => f.write_str(ident),
			Unary(unary) => Display::fmt(unary, f),
			Binary(binary) => Display::fmt(binary, f),
			Unit => f.write_str("_"),
		}
	}
}
