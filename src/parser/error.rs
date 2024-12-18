use std::{
	fmt::Display,
	num::{ParseFloatError, ParseIntError},
};

use crate::lexer::{error::LexerError, token::Token};

#[derive(Debug, Clone)]
pub enum ParserError {
	UnexpectedEOF,
	UnexpectedToken(Token),
	Lexer(LexerError),
	ParseInt(ParseIntError),
	ParseFloat(ParseFloatError),
	ExpectedCloseParen,
}

impl From<LexerError> for ParserError {
	fn from(value: LexerError) -> Self {
		Self::Lexer(value)
	}
}
impl From<ParseIntError> for ParserError {
	fn from(value: ParseIntError) -> Self {
		Self::ParseInt(value)
	}
}
impl From<ParseFloatError> for ParserError {
	fn from(value: ParseFloatError) -> Self {
		Self::ParseFloat(value)
	}
}

impl Display for ParserError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ParserError::UnexpectedEOF => f.write_str("Unexpected end of input"),
			ParserError::UnexpectedToken(tk) => write!(f, "Unexpected token: {tk}"),
			ParserError::Lexer(lexer_error) => Display::fmt(lexer_error, f),
			ParserError::ParseInt(parse_int_error) => Display::fmt(parse_int_error, f),
			ParserError::ParseFloat(parse_float_error) => Display::fmt(parse_float_error, f),
			ParserError::ExpectedCloseParen => f.write_str("Expected a closing parenthesis"),
		}
	}
}
