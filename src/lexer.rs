use crate::error::SyntaxError;
use crate::position::{ Position, START_POSITION };

use std::fmt;


pub type Number = String;
pub type Name = String;

#[derive(Debug, Clone)]
pub enum TokenData {
	Number(Number),
	Identifier(Name),
	Keyword(Name),
	Symbol{index: usize},
	Literal(Literal),
}
impl fmt::Display for TokenData {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

pub type Tuple = String;
pub type List = String;
pub type Array = String;
pub type Set = String;
pub type Map = String;

#[derive(Debug, Clone)]
pub enum Literal {
	Char(char),
	String(String),
	Tuple(Tuple),
	List(List),
	Array(Array),
	Set(Set),
	Map(Map),
}
impl fmt::Display for Literal {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

const ID_SYMBOLS: &[u8] = &[b'_', b'?', b'!', b'.', b'@'];
const KEYWORDS: &[&str] = &["let", "val", "fn", "proc", "end", "use"];
const SYMBOLS: &[&str] = &[":", "=", "->"];

pub struct Token {
	pub data: TokenData,
	pub start: Position,
	pub end: Position,
}
impl fmt::Debug for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {} > {}", self.data, self.start, self.end)
	}
}
impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

pub fn tokenize(buf: &[u8]) -> Result<Vec<Token>, SyntaxError> {
	let mut tokens: Vec<Token> = vec![];
	let mut position: Position = START_POSITION;
	while position.index < buf.len() {
		match buf[position.index] {
			b' ' | b'\t' | b'\n' => { position.next(buf); continue }
			b'0'..=b'9' => {
				let (data, end) = get_number(buf, &mut position.clone());
				tokens.push(Token {
					data,
					start: position,
					end: end.clone(),
				});
				position = end;
				continue;
			},
			_ => (),
		};
		if let Some((data, end)) = get_literal(buf, &mut position.clone()) {
			tokens.push(Token {
				data: data.clone(),
				start: position.clone(),
				end: end.clone(),
			});
			position = end;
			continue;
		};
		if let Some((data, end)) = get_identifier(buf, &mut position.clone()) {
			tokens.push(Token {
				data,
				start: position,
				end: end.clone(),
			});
			position = end;
			continue;
		};
		if let Some((data, end)) = get_symbol(buf, &mut position.clone()) {
			tokens.push(Token {
				data,
				start: position,
				end: end.clone(),
			});
			position = end;
			continue;
		};
		return Err(SyntaxError {
			data: "invalid character".to_owned(),
			start: position.clone(),
			end: position.get_next(buf).get_next(buf),
			buf
		});
	};
	Ok(tokens)
}

fn get_number(buf: &[u8], position: &mut Position) -> (TokenData, Position) {
	let mut data: String = "".to_owned();
	while position.index < buf.len() && buf[position.index].is_ascii_digit() {
		data.push(buf[position.index] as char);
		position.next(buf);
	};
	(TokenData::Number(data), position.clone())
}


fn get_literal(buf: &[u8], position: &mut Position) -> Option<(TokenData, Position)> {
	let index_opt: Option<usize> = is_literal(buf, position.clone());
	if index_opt.is_none() { return None };
	let index = index_opt.unwrap();
	position.next(buf);
	let mut data: String = "".to_owned();
	let mut layer: usize = 0;
	while position.index < buf.len() {
		if buf[position.index] == LITERAL_PAIRS[index].1 {
			if layer == 0 {
				return Some((TokenData::Literal{index, data}, position.clone().get_next(buf)))
			} else {
				layer -= 1;
			};
		};
		if &buf[position.index] == LITERAL_PAIRS[index].0.as_bytes().last().unwrap() {
			layer += 1;
		};
		data.push(buf[position.index] as char);
		position.next(buf);
	};
	None
}

fn is_literal(buf: &[u8], position: Position) -> Option<usize> {
	for (i, pair) in LITERAL_PAIRS.iter().enumerate() {
		if position.index + pair.0.len() >= buf.len() { continue };
		if pair.0.as_bytes() == &buf[position.index..position.index + pair.0.len()] {
			return Some(i)
		}
	};
	None
}

fn get_identifier(buf: &[u8], position: &mut Position) -> Option<(TokenData, Position)> {
	let mut data: Name = "".to_owned();
	while position.index < buf.len() {
		if !buf[position.index].is_ascii_alphanumeric() && !ID_SYMBOLS.contains(&buf[position.index]) {
			break;
		};
		data.push(buf[position.index] as char);
		position.next(buf);
	};
	if data.len() == 0 { return None };
	if KEYWORDS.contains(&data.as_str()) {
		Some((TokenData::Keyword(data), position.clone()))
	} else {
		Some((TokenData::Identifier(data), position.clone()))
	}
}

fn get_symbol(buf: &[u8], position: &mut Position) -> Option<(TokenData, Position)> {
	let mut data: Name = "".to_owned();
	while position.index < buf.len() && data.len() <= 3 {
		if let Some(index) = SYMBOLS.iter().position(|&symbol| symbol == data.as_str()) {
			return Some((TokenData::Symbol{index}, position.clone().get_next(buf)))
		};
		data.push(buf[position.index] as char);
		position.next(buf);
	};
	None
}
