use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Token {
	LiteralNumber(Box<str>, bool),
	LiteralString(Box<str>),
	Identifier(Box<str>),
	Let,
	Assign,
	Equals,
	OpenParen,
	CloseParen,
	Plus,
	Minus,
	Star,
	Slash,
	Percent,
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
			Equals => f.write_str("=="),
			OpenParen => f.write_str("("),
			CloseParen => f.write_str(")"),
			Plus => f.write_str("+"),
			Minus => f.write_str("-"),
			Star => f.write_str("*"),
			Slash => f.write_str("/"),
			Percent => f.write_str("%"),
		}
	}
}

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

pub struct Lexer;

static KEYWORDS: phf::Map<&str, Token> = phf::phf_map! {
	"let" => Token::Let,
};

impl Lexer {
	fn is_identifier_char(ch: &char) -> bool {
		matches!(ch, '0'..='9' | 'a'..='z' | 'A'..='Z' | '_')
	}

	pub fn tokenize(source: &str) -> Result<Vec<Token>, LexerError> {
		use Token::*;

		let mut chars = source.chars().peekable();
		let mut tokens = Vec::new();

		while let Some(curr) = chars.next() {
			let next_token = match curr {
				'=' => {
					if chars.next_if(|&ne| ne == '=').is_some() {
						Equals
					} else {
						Assign
					}
				}
				'(' => OpenParen,
				')' => CloseParen,
				'+' => Plus,
				'-' => Minus,
				'*' => Star,
				'/' => Slash,
				'%' => Percent,
				'0'..='9' | '.' => {
					let mut acc = curr.to_string();
					let mut has_decimal_point = curr == '.';

					while let Some(ne) = chars
						.next_if(|&ne| ne.is_ascii_digit() || (ne == '.' && !has_decimal_point))
					{
						if ne == '.' {
							has_decimal_point = true;
						}
						acc.push(ne);
					}

					LiteralNumber(acc.into_boxed_str(), has_decimal_point)
				}
				'a'..='z' | 'A'..='Z' | '_' => {
					let mut acc = curr.to_string();
					while let Some(ne) = chars.next_if(Self::is_identifier_char) {
						acc.push(ne);
					}

					if let Some(tk) = KEYWORDS.get(&acc) {
						tk.to_owned()
					} else {
						Identifier(acc.into_boxed_str())
					}
				}
				'"' => {
					let mut acc = String::new();
					let mut escaped = false;
					let mut closed = false;
					for ne in chars.by_ref() {
						if ne == '\r' {
							continue;
						}

						if escaped {
							match ne {
								'"' => acc.push('"'),
								'\\' => acc.push('\\'),
								'n' | '\n' => acc.push('\n'),
								'r' => acc.push('\r'),
								't' => acc.push('\t'),
								'b' => acc.push('\x08'),
								_ => (),
							}
							escaped = false;
							continue;
						}

						match ne {
							'\n' => return Err(LexerError::UnclosedString(acc.into_boxed_str())),
							'"' => {
								closed = true;
								break;
							}
							'\\' => escaped = true,
							other => acc.push(other),
						}
					}
					if !closed {
						return Err(LexerError::UnclosedString(acc.into_boxed_str()));
					}
					LiteralString(acc.into_boxed_str())
				}
				' ' | '\t' | '\n' => continue,
				other => return Err(LexerError::UnexpectedChar(other)),
			};
			tokens.push(next_token);
		}
		Ok(tokens)
	}
}
