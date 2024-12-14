use std::fmt::Display;

use super::node::Expression;

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
	Plus,
	Minus,
}

impl Display for UnaryOp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use UnaryOp::*;

		f.write_str(match self {
			Plus => "+",
			Minus => "-",
		})
	}
}

#[derive(Debug, Clone)]
pub struct UnaryExpression {
	pub operator: UnaryOp,
	pub right: Box<Expression>,
}

impl Display for UnaryExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}", self.operator, self.right)
	}
}
