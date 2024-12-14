pub mod error;
pub mod token;

use error::LexerError;
use token::Token;

pub struct Lexer;

static KEYWORDS: phf::Map<&str, Token> = phf::phf_map! {
	"let" => Token::Let,
	"_" => Token::Unit,
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
