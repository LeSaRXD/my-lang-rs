use std::fmt::Display;

use super::Expression;

#[derive(Debug, Clone)]
pub struct AssignmentExpression {
	pub ident: Box<str>,
	pub value: Box<Expression>,
}

impl Display for AssignmentExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} = {}", self.ident, self.value)
	}
}
