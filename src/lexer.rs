use crate::error::SyntaxError;
use crate::position::Position;

use std::fmt::{ Debug, Display };
use std::str::from_utf8;

pub type Parenthesis = char;
pub type Number = String;
pub type Character = String;
pub type Name = String;

#[derive(Debug, Clone)]
pub enum TokenData {
	Parenthesis(Parenthesis),
	Number(Number),
	Character(Character),
	String(String),
	Identifier(Name),
	Keyword(Name),
	Symbol { index: usize },
}
impl Display for TokenData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(self, f)
	}
}

pub type Tuple = String;
pub type List = String;
pub type Array = String;
pub type Set = String;
pub type Map = String;

const KEYWORDS: &[&str] = &["import", "@main", "proc", "fn", "val", "var", "final", "const", "let", "end"];
const SYMBOLS: &[&str] = &[":", "=", "->"];

pub struct Token {
	pub data: TokenData,
	pub start: Position,
	pub end: Position,
}
impl Debug for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {} > {}", self.data, self.start, self.end)
	}
}
impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Debug::fmt(self, f)
	}
}

pub fn tokenize<'a>(buf: &'a[u8], newlines: &'a[usize]) -> Result<Vec<Token>, SyntaxError<'a>> {
	let mut tokens: Vec<Token> = vec![];
	let mut position: Position = Position(0);
	while position.0 < buf.len() {
		match buf[position.0] {
			b' ' | b'\t' | b'\n' => position.0 += 1,
			ch @ (b'(' | b')' | b'[' | b']' | b'{' | b'}') => {
				tokens.push(Token {
					data: TokenData::Parenthesis(ch as char),
					start: position,
					end: position + 1,
				});
				position.0 += 1;
			},
			b'0'..=b'9' => {
				let (data, end) = get_number(buf, &mut position.clone());
				tokens.push(Token {
					data,
					start: position,
					end,
				});
				position = end;
			},
			b'\'' => {
				let (data, end) = get_character(buf, &mut position.clone(), newlines)?;
				tokens.push(Token {
					data,
					start: position,
					end,
				});
				position = end;
			},
			b'\"' => {
				let (data, end) = get_string(buf, &mut position.clone(), newlines)?;
				tokens.push(Token {
					data,
					start: position,
					end,
				});
				position = end;
			},
			b'.' | b'@' | b'_' | b'a'..=b'z' | b'A'..=b'Z' => {
				let (data, end) = get_identifier(buf, &mut position.clone());
				tokens.push(Token {
					data,
					start: position,
					end,
				});
				position = end;
			}
			_ => {
				if let Some((data, end)) = get_symbol(buf, &mut position.clone()) {
					tokens.push(Token {
						data,
						start: position,
						end,
					});
					position = end;
					continue;
				};
				return Err(SyntaxError {
					data: "invalid character".to_owned(),
					start: position,
					end: Position(position.0 + 2),
					buf,
					newlines,
				})
			},
		};
	};
	Ok(tokens)
}

fn get_number(buf: &[u8], position: &mut Position) -> (TokenData, Position) {
	let mut data: Number = "".to_owned();
	while position.0 < buf.len() && buf[position.0].is_ascii_digit() {
		data.push(buf[position.0] as char);
		position.0 += 1;
	};
	(TokenData::Number(data), *position)
}

fn get_character<'a>(buf: &'a[u8], position: &mut Position, newlines: &'a[usize]) -> Result<(TokenData, Position), SyntaxError<'a>> {
	let start = *position;
	let mut data: Character = "".to_owned();
	position.0 += 1;
	if buf[position.0] == b'\\' {
		position.0 += 1;
		match buf[position.0] {
			ch @ (b'\\' | b'\'' | b't' | b'n' | b'r' | b'0') => data.push(ch as char),
			b'x' => {
				position.0 += 1;
				if position.0 >= buf.len() + 3 {
					return Err(SyntaxError {
						data: "eof before end of character literal".to_owned(),
						start,
						end: *position,
						buf,
						newlines,
					})
				};
				let ascii_code = from_utf8(&buf[position.0..=position.0+1]).unwrap();
				let number = match u8::from_str_radix(ascii_code, 16) {
					Ok(ok) => ok,
					Err(err) => return Err(SyntaxError {
						data: format!("not a valid ascii code,\n{}", err.to_string()),
						start,
						end: *position,
						buf,
						newlines,
					}),
				};
				data.push_str(&("\\x".to_owned() + ascii_code))
			},
			ch => return Err(SyntaxError {
				data: format!("{} is not a valid escape in character literals", ch),
				start,
				end: *position,
				buf,
				newlines,
			}),
		};
	}
	else {
		match buf[position.0] {
			b'\t' | b'\n' | b'\r' => return Err(SyntaxError {
				data: "character literals can't be whitespace, use escapes instead".to_owned(),
				start,
				end: *position,
				buf,
				newlines,
			}),
			b'\'' => return Err(SyntaxError {
				data: "character literals can't be empty".to_owned(),
				start,
				end: *position,
				buf,
				newlines,
			}),
			ch => data.push(ch as char),
		}
	};
	position.0 += 1;
	if buf[position.0] != b'\'' {
		return Err(SyntaxError {
			data: "character literals must be terminated with `'`".to_owned(),
			start,
			end: *position,
			buf,
			newlines
		})
	};
	Ok((TokenData::Character(data), *position))
}

fn get_string<'a>(buf: &'a[u8], position: &mut Position, newlines: &'a[usize]) -> Result<(TokenData, Position), SyntaxError<'a>> {
	let start = *position;
	let mut data: String = "".to_owned();
	position.0 += 1;
	while position.0 < buf.len() && buf[position.0] != b'\"' {
		if buf[position.0] == b'\\' {
			match buf[position.0] {
				ch @ (b'\\' | b'\'' | b't' | b'n' | b'r') => data.push(ch as char),
				b'x' => {
					position.0 += 1;
					if position.0 >= buf.len() + 3 {
						return Err(SyntaxError {
							data: "eof before end of string literal".to_owned(),
							start,
							end: *position,
							buf,
							newlines,
						})
					};
					let ascii_code = from_utf8(&buf[position.0..=position.0+1]).unwrap();
					let number = match u8::from_str_radix(ascii_code, 16) {
						Ok(ok) => ok,
						Err(err) =>
							return Err(SyntaxError {
								data: format!("{} is not a valid ascii code", ascii_code),
								start,
								end: *position,
								buf,
								newlines,
							})
					};
					data.push_str(&("\\x".to_owned() + ascii_code))
				},
				ch => return Err(SyntaxError {
					data: format!("{} is not a valid escape in string literals", ch),
					start,
					end: *position,
					buf,
					newlines,
				})
			}
		}
	};
	position.0 += 1;
	Ok((TokenData::String(data), *position))
}

fn get_identifier(buf: &[u8], position: &mut Position) -> (TokenData, Position) {
	let mut data: Name = (buf[position.0] as char).to_string();
	while position.0 < buf.len() - 1 {
		position.0 += 1;
		if buf[position.0].is_ascii_alphanumeric() || buf[position.0] == b'_' {
			data.push(buf[position.0] as char);
			continue;
		}
		if buf[position.0] == b'!' || buf[position.0] == b'?' || buf[position.0] == b'\'' {
			data.push(buf[position.0] as char);
			break;
		}
	};
	if KEYWORDS.contains(&data.as_str()) {
		(TokenData::Keyword(data), *position)
	} else {
		(TokenData::Identifier(data), *position)
	}
}

fn get_symbol(buf: &[u8], position: &mut Position) -> Option<(TokenData, Position)> {
	let mut data: Name = "".to_owned();
	while position.0 < buf.len() && data.len() <= 3 {
		if let Some(index) = SYMBOLS.iter().position(|&symbol| symbol == data.as_str()) {
			return Some((TokenData::Symbol { index }, *position))
		};
		data.push(buf[position.0] as char);
		position.0 += 1;
	};
	None
}
