use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Token {
	LiteralNumber(Box<str>, bool),
	LiteralString(Box<str>),
	Identifier(Box<str>),
	Let,
	Assign,
	OpenParen,
	CloseParen,
	Plus,
	Minus,
	Star,
	Slash,
	Percent,
	Equals,
	Unit,
	Semicolon,
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use Token::*;
		match self {
			LiteralNumber(num, _) => f.write_str(num),
			LiteralString(st) => write!(f, "{st:?}"),
			Identifier(ident) => f.write_str(ident),
			Let => f.write_str("let"),
			Assign => f.write_str("="),
			OpenParen => f.write_str("("),
			CloseParen => f.write_str(")"),
			Plus => f.write_str("+"),
			Minus => f.write_str("-"),
			Star => f.write_str("*"),
			Slash => f.write_str("/"),
			Percent => f.write_str("%"),
			Equals => f.write_str("=="),
			Unit => f.write_str("_"),
			Semicolon => f.write_str(";"),
		}
	}
}
