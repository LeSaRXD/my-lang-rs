use std::fmt::Display;

use crate::lexer::token::Token::Let;

use super::Expression;

#[derive(Debug, Clone)]
pub struct DeclarationExpression {
	pub ident: Box<str>,
	pub value: Box<Expression>,
	pub mutable: bool,
}

impl Display for DeclarationExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}{} {} = {}",
			Let,
			if self.mutable { "~" } else { "" },
			self.ident,
			self.value
		)
	}
}
