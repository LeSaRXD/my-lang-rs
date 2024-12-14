use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum LexerError {
	UnexpectedChar(char),
	UnclosedString(Box<str>),
}

impl Display for LexerError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			LexerError::UnexpectedChar(ch) => write!(f, "Unexpected character: '{ch}'"),
			LexerError::UnclosedString(st) => write!(f, "Unclosed string literal: {st}"),
		}
	}
}
