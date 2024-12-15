use std::fmt::Display;

use super::Expression;

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
	Add,
	Subtract,
	Multiply,
	Divide,
	Modulo,
	Equals,
}

impl Display for BinaryOp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use BinaryOp::*;

		f.write_str(match self {
			Add => "+",
			Subtract => "-",
			Multiply => "*",
			Divide => "/",
			Modulo => "%",
			Equals => "==",
		})
	}
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
	pub left: Box<Expression>,
	pub operator: BinaryOp,
	pub right: Box<Expression>,
}

impl Display for BinaryExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {} {}", self.left, self.operator, self.right)
	}
}
