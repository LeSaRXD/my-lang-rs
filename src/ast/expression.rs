use std::fmt::{Debug, Display};

use super::{binary_expr::BinaryExpression, number::Number, unary_expr::UnaryExpression};

#[derive(Debug, Clone)]
pub enum Expression {
	LiteralNumber(Number),
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
			LiteralNumber(num) => Display::fmt(num, f),
			LiteralString(st) => Debug::fmt(st, f),
			Identifier(ident) => f.write_str(ident),
			Unary(unary) => Display::fmt(unary, f),
			Binary(binary) => Display::fmt(binary, f),
			Unit => f.write_str("_"),
		}
	}
}

pub type Program = Vec<Expression>;
