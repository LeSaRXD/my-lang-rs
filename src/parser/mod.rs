pub mod error;

use std::ops::Not;

use error::ParserError;

use crate::{
	ast::{
		binary_expr::{BinaryExpression, BinaryOp},
		expression::Expression,
		unary_expr::{UnaryExpression, UnaryOp},
	},
	lexer::{token::Token, Lexer},
	numeric::Numeric,
};

pub struct Parser {
	idx: usize,
	tokens: Vec<Token>,
}

type ParserResult = std::result::Result<Expression, ParserError>;
impl Parser {
	pub fn new() -> Self {
		Self {
			idx: 0,
			tokens: Vec::new(),
		}
	}

	fn eof(&self) -> bool {
		self.idx >= self.tokens.len()
	}
	fn at(&self) -> Option<&Token> {
		self.tokens.get(self.idx)
	}
	fn advance(&mut self, n: usize) {
		self.idx += n;
	}

	fn parse_expression(&mut self) -> ParserResult {
		self.parse_equality()
	}

	fn parse_equality(&mut self) -> ParserResult {
		use Token::*;

		let mut left = self.parse_additive()?;
		while let Some(Equals) = self.at() {
			self.advance(1);
			let right = self.parse_additive()?;
			let expr = BinaryExpression {
				left: Box::new(left),
				right: Box::new(right),
				operator: BinaryOp::Equals,
			};
			left = Expression::Binary(expr);
		}
		Ok(left)
	}

	fn parse_additive(&mut self) -> ParserResult {
		use Token::*;

		let mut left = self.parse_multiplicative()?;
		while let Some(operator) = self.at() {
			let operator = match operator {
				Plus => BinaryOp::Add,
				Minus => BinaryOp::Subtract,
				_ => break,
			};
			self.advance(1);
			let right = self.parse_multiplicative()?;
			let expr = BinaryExpression {
				left: Box::new(left),
				right: Box::new(right),
				operator,
			};
			left = Expression::Binary(expr);
		}
		Ok(left)
	}

	fn parse_multiplicative(&mut self) -> ParserResult {
		use Token::*;

		let mut left = self.parse_unary()?;
		while let Some(operator) = self.at() {
			let operator = match operator {
				Star => BinaryOp::Multiply,
				Slash => BinaryOp::Divide,
				Percent => BinaryOp::Modulo,
				_ => break,
			};
			self.advance(1);
			let right = self.parse_unary()?;
			let expr = BinaryExpression {
				left: Box::new(left),
				right: Box::new(right),
				operator,
			};
			left = Expression::Binary(expr);
		}
		Ok(left)
	}

	fn parse_unary(&mut self) -> ParserResult {
		use Token::*;

		let mut expr = None;
		while let Some(operator) = self.at() {
			let operator = match operator {
				Plus => UnaryOp::Plus,
				Minus => UnaryOp::Minus,
				_ => break,
			};
			self.advance(1);
			let right = self.parse_unary()?;
			expr = Some(UnaryExpression {
				operator,
				right: Box::new(right),
			});
		}
		if let Some(expr) = expr {
			Ok(Expression::Unary(expr))
		} else {
			self.parse_primary()
		}
	}

	fn parse_primary(&mut self) -> ParserResult {
		use Token::*;

		let token = self.at().ok_or(ParserError::UnexpectedEOF)?;

		let (next, advance_by) = match token {
			LiteralNumber(num, true) => {
				(Expression::LiteralNumber(Numeric::Float(num.parse()?)), 1)
			}
			LiteralNumber(num, false) => (Expression::LiteralNumber(Numeric::Int(num.parse()?)), 1),
			LiteralString(st) => (Expression::LiteralString(st.to_owned()), 1),
			Identifier(ident) => (Expression::Identifier(ident.to_owned()), 1),
			OpenParen => {
				self.advance(1);
				let expr = self.parse_expression()?;
				if matches!(self.at(), Some(CloseParen)).not() {
					return Err(ParserError::ExpectedCloseParen);
				}
				(expr, 1)
			}
			Unit => (Expression::Unit, 1),
			Let => todo!(),
			Assign => todo!(),
			unexpected => return Err(ParserError::UnexpectedToken(unexpected.to_owned())),
		};

		self.advance(advance_by);
		Ok(next)
	}

	pub fn produce_ast(&mut self, src: &str) -> Result<Vec<Expression>, ParserError> {
		let new_tokens = Lexer::tokenize(src).map_err(|err| {
			self.clear();
			err
		})?;
		self.tokens.extend(new_tokens);

		let mut program = Vec::new();
		loop {
			program.push(self.parse_expression()?);
			match self.at() {
				None => break,
				Some(Token::Semicolon) => {
					self.advance(1);
					if self.eof() {
						program.push(Expression::Unit);
						break;
					} else {
						continue;
					}
				}
				Some(tk) => return Err(ParserError::UnexpectedToken(tk.to_owned())),
			};
		}
		self.clear();
		Ok(program)
	}

	fn clear(&mut self) {
		self.tokens.clear();
		self.idx = 0;
	}
}
