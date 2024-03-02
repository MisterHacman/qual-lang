use crate::error::SyntaxError;
use crate::position::{ Position, TAB_SIZE };

use std::fmt::{ Debug, Display };

pub type Parenthesis = char;
pub type Number = String;
pub type Character = String;
pub type Name = String;

#[derive(Clone)]
pub enum TokenData {
	Parenthesis(Parenthesis),
	Number(Number),
	Character(Character),
	String(String),
	Identifier(Name),
	Keyword(Name),
	Symbol { index: usize },
}
impl Debug for TokenData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let (kind, data) = match self {
			TokenData::Parenthesis(data) => ("Paren", data.to_string()),
			TokenData::Number(data) => ("Number", data.to_owned()),
			TokenData::Character(data) => ("Char", format!("\'{}\'", data)),
			TokenData::String(data) => ("Str", format!("\"{}\"", data.to_owned())),
			TokenData::Identifier(data) => ("Id", data.to_owned()),
			TokenData::Keyword(data) => ("Key", data.to_owned()),
			TokenData::Symbol { index } => ("Sym", SYMBOLS[*index].to_owned()),
		};
		write!(f, "{}, {}", kind, data)
	}
}
impl Display for TokenData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Debug::fmt(self, f)
	}
}

pub type Tuple = String;
pub type List = String;
pub type Array = String;
pub type Set = String;
pub type Map = String;

const KEYWORDS: &[&str] = &["import", "proc", "fn", "let", "val", "var", "const", "final"];
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
	let mut position: Position = Position::new(0);
	while position.0 < buf.len() {
		match buf[position.0] {
			b' ' | b'\n' => position.0 += 1,
			b'\t' => position.0 += TAB_SIZE,
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
			},
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
					end: Position::new(position.0 + 2),
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
	if position.0 >= buf.len() { return Err(SyntaxError {
		data: "character literal expected end quote".to_owned(),
		start,
		end: *position,
		buf,
		newlines
	}) };
	match buf[position.0] {
		b'\\' => {
			position.0 += 1;
			if position.0 >= buf.len() { return Err(SyntaxError { 
				data: "unfinished character escape".to_owned(),
				start: start + 1,
				end: *position + 1,
				buf,
				newlines,
			}) };
			let (escape, end) = &get_character_escape(buf, start + 1, position, newlines)?;
			data.push_str(&escape);
			position.0 = end.0;
		},
		ch @ (b'\t' | b'\n' | b'\r') => return Err(SyntaxError {
			data: "invalid whitespace in character literal, use character escape".to_owned(),
			start: start + 1,
			end: *position + if ch == b'\t' { TAB_SIZE } else { 1 },
			buf,
			newlines,
		}),
		b'\'' => return Err(SyntaxError {
			data: "character literal with empty quotes".to_owned(),
			start,
			end: *position + 1,
			buf,
			newlines,
		}),
		ch => data.push(ch as char),
	}
	position.0 += 1;
	if position.0 >= buf.len() {
		return Err(SyntaxError {
			data: "character literal expected end quote".to_owned(),
			start,
			end: Position::new(buf.len()),
			buf,
			newlines,
		})
	};
	if buf[position.0] != b'\'' {
		return Err(SyntaxError {
			data: "character literal expected end quote".to_owned(),
			start,
			end: *position,
			buf,
			newlines
		})
	};
	Ok((TokenData::Character(data), *position + 1))
}

fn get_string<'a>(buf: &'a[u8], position: &mut Position, newlines: &'a[usize]) -> Result<(TokenData, Position), SyntaxError<'a>> {
	let start = *position;
	let mut data: String = "".to_owned();
	position.0 += 1;
	while position.0 < buf.len() {
		match buf[position.0] {
			b'\\' => {
				if position.0 + 1 >= buf.len() { return Err(SyntaxError {
					data: "unfinished character escape".to_owned(),
					start,
					end: *position,
					buf,
					newlines
				}) };
				let (escape, end) = &get_character_escape(buf, *position, &mut (*position + 1), newlines)?;
				data.push_str(&escape);
				position.0 = end.0;
			},
			ch @ (b'\t' | b'\n' | b'\r') => return Err(SyntaxError {
				data: "invalid whitespace in string literal, use character escape".to_owned(),
				start: *position,
				end: *position + if ch == b'\t' { TAB_SIZE } else { 1 },
				buf,
				newlines,
			}),
			b'\"' => break,
			_ => data.push(buf[position.0] as char),
		};
		position.0 += 1;
	};
	if position.0 >= buf.len() { return Err(SyntaxError {
		data: "string literal expected end quote, found eof".to_owned(),
		start,
		end: Position::new(buf.len()),
		buf,
		newlines,
	}) };
	position.0 += 1;
	Ok((TokenData::String(data), *position))
}

fn get_character_escape<'a>(
	buf: &'a[u8],
	start: Position,
	position: &mut Position,
	newlines: &'a[usize]
) -> Result<(String, Position), SyntaxError<'a>> {
	match buf[position.0] {
		ch @ (b'\\' | b'\'' | b't' | b'n' | b'r' | b'0') => Ok(("\\".to_owned() + &(ch as char).to_string(), *position)),
		ch @ (b'\n' | b'\r') => Ok((format!("{:?}", ch as char).replace("'", ""), *position)),
		b'x' => {
			position.0 += 1;
			if position.0 + 1 >= buf.len() { return Err(SyntaxError {
				data: "unfinished ascii code".to_owned(),
				start,
				end: *position,
				buf,
				newlines,
			}) };
			let ascii_code = format!("{}{}", buf[position.0] as char, buf[position.0 + 1] as char);
			let _number = match u8::from_str_radix(&ascii_code, 16) {
				Ok(ok) => ok,
				Err(_err) => return Err(SyntaxError {
					data: format!("0x{} is not a valid ascii code", ascii_code),
					start,
					end: *position,
					buf,
					newlines,
				})
			};
			Ok(("\\x".to_owned() + &ascii_code, *position + 1))
		},
		ch => Err(SyntaxError {
			data: format!("{:?} is not a valid character escape", ch as char),
			start,
			end: *position + 1,
			buf,
			newlines,
		})
	}
}

fn get_identifier(buf: &[u8], position: &mut Position) -> (TokenData, Position) {
	let mut data: Name = (buf[position.0] as char).to_string();
	while position.0 + 1 < buf.len() {
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
	position.0 += 1;
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
