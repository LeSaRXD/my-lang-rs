pub mod assignment;
pub mod binary;
pub mod declaration;
pub mod unary;

use assignment::AssignmentExpression;
use binary::BinaryExpression;
use declaration::DeclarationExpression;
use unary::UnaryExpression;

use crate::numeric::Numeric;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub enum Expression {
	Program(Vec<Expression>),
	LiteralNumber(Numeric),
	LiteralString(Box<str>),
	Identifier(Box<str>),
	Unary(UnaryExpression),
	Binary(BinaryExpression),
	Unit,
	Assignment(AssignmentExpression),
	Declaration(DeclarationExpression),
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
			Assignment(assignment) => Display::fmt(assignment, f),
			Declaration(declaration) => Display::fmt(declaration, f),
		}
	}
}
