use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum LexerError {
	UnexpectedChar(char),
	UnclosedString(Box<str>),
	UnsupportedEscape(char),
}

impl Display for LexerError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use LexerError::*;

		match self {
			UnexpectedChar(ch) => write!(f, "Unexpected character: '{ch}'"),
			UnclosedString(st) => write!(f, "Unclosed string literal: \"{st:?}",),
			UnsupportedEscape(ch) => write!(f, "Unsupported escape sequence: \\{ch}"),
		}
	}
}
