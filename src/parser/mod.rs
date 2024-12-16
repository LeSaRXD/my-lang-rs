pub mod error;

use std::{fmt::Display, ops::Not};

use error::ParserError::{self, *};

use crate::{
	expression::{
		assignment::AssignmentExpression,
		binary::{BinaryExpression, BinaryOp},
		declaration::DeclarationExpression,
		unary::{UnaryExpression, UnaryOp},
		Expression,
	},
	helpers::iter_to_string,
	lexer::{
		token::Token::{self, *},
		Lexer,
	},
	numeric::Numeric,
};

type ParserResult = std::result::Result<Expression, ParserError>;

#[derive(Debug)]
pub struct Parser {
	idx: usize,
	tokens: Vec<Token>,
}

impl Display for Parser {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			r#"Display {{
idx: {},
tokens: {}
}}"#,
			self.idx,
			iter_to_string(&self.tokens)
		)
	}
}

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

	fn at(&self, offset: usize) -> Option<&Token> {
		self.tokens.get(self.idx + offset)
	}

	fn current(&self) -> Option<&Token> {
		self.at(0)
	}

	fn advance(&mut self, n: usize) {
		self.idx += n;
	}

	pub fn produce_ast(&mut self, src: &str, keep_tokens: bool) -> ParserResult {
		let new_tokens = Lexer::tokenize(src).map_err(|err| {
			self.clear(keep_tokens);
			err
		})?;
		self.tokens.extend(new_tokens);

		let mut program = Vec::new();
		let res = loop {
			match self.parse_expression() {
				Ok(expr) => program.push(expr),
				Err(err) => break Err(err),
			}
			match self.current() {
				None => break Ok(program),
				Some(Semicolon) => {
					self.advance(1);
					if self.eof() {
						program.push(Expression::Unit);
						break Ok(program);
					} else {
						continue;
					}
				}
				Some(tk) => break Err(UnexpectedToken(tk.to_owned())),
			}
		};

		self.clear(keep_tokens);
		res.map(Expression::Program)
	}

	fn parse_expression(&mut self) -> ParserResult {
		self.parse_equality()
	}

	fn parse_equality(&mut self) -> ParserResult {
		let mut left = self.parse_additive()?;
		while let Some(Equals) = self.current() {
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
		let mut left = self.parse_multiplicative()?;
		while let Some(operator) = self.current() {
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
		let mut left = self.parse_unary()?;
		while let Some(operator) = self.current() {
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
		let mut expr = None;
		while let Some(operator) = self.current() {
			let operator = match operator {
				Plus => UnaryOp::Plus,
				Minus => UnaryOp::Minus,
				_ => break,
			};
			self.advance(1);
			let right = self.parse_assignment()?;
			expr = Some(UnaryExpression {
				operator,
				right: Box::new(right),
			});
		}

		if let Some(expr) = expr {
			Ok(Expression::Unary(expr))
		} else {
			self.parse_assignment()
		}
	}

	fn parse_assignment(&mut self) -> ParserResult {
		match (self.current(), self.at(1)) {
			(Some(Identifier(ident)), Some(Assign)) => {
				let ident = Box::to_owned(ident);
				self.advance(2);
				let expr = self.parse_expression()?;
				Ok(Expression::Assignment(AssignmentExpression {
					ident,
					value: Box::new(expr),
				}))
			}
			_ => self.parse_declaration(),
		}
	}

	fn parse_declaration(&mut self) -> ParserResult {
		if let Some(Let) = self.current() {
			self.advance(1);

			let mutable = if let Some(Mutable) = self.current() {
				self.advance(1);
				true
			} else {
				false
			};

			match (self.current(), self.at(1)) {
				(Some(Identifier(ident)), Some(Assign)) => {
					let ident = Box::to_owned(ident);
					self.advance(2);
					let expr = self.parse_expression()?;
					Ok(Expression::Declaration(DeclarationExpression {
						ident,
						value: Box::new(expr),
						mutable,
					}))
				}
				(Some(Identifier(_)), Some(other)) => Err(UnexpectedToken(other.to_owned())),
				(Some(Identifier(_)), None) => Err(UnexpectedEOF),
				(Some(other), _) => Err(UnexpectedToken(other.to_owned())),
				(None, _) => Err(UnexpectedEOF),
			}
		} else {
			self.parse_primary()
		}
	}

	fn parse_primary(&mut self) -> ParserResult {
		let token = self.current().ok_or(UnexpectedEOF)?;

		let next = match token {
			LiteralNumber(num, true) => Expression::LiteralNumber(Numeric::Float(num.parse()?)),
			LiteralNumber(num, false) => Expression::LiteralNumber(Numeric::Int(num.parse()?)),
			LiteralString(st) => Expression::LiteralString(st.to_owned()),
			Identifier(ident) => Expression::Identifier(ident.to_owned()),
			OpenParen => {
				self.advance(1);
				let expr = self.parse_expression()?;
				if matches!(self.current(), Some(CloseParen)).not() {
					return Err(ExpectedCloseParen);
				}
				expr
			}
			Unit => Expression::Unit,
			unexpected => return Err(UnexpectedToken(unexpected.to_owned())),
		};

		self.advance(1);
		Ok(next)
	}

	fn clear(&mut self, keep_tokens: bool) {
		if keep_tokens {
			self.idx = self.tokens.len();
		} else {
			self.tokens.clear();
			self.idx = 0;
		}
	}
}
