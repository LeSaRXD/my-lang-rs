use std::fmt::Display;

use crate::lexer::token::Token;

use super::Expression;

#[derive(Debug, Clone)]
pub struct DeclarationExpression {
	pub ident: Box<str>,
	pub value: Box<Expression>,
}

impl Display for DeclarationExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {} = {}", Token::Let, self.ident, self.value)
	}
}
