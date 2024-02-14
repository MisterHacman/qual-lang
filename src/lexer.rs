use crate::lexer::{ TokenType::*, LiteralType::* };

use crate::error::SyntaxError;
use crate::position::{ Position, START_POSITION };

use std::fmt;

#[derive(Debug)]
pub enum TokenType {
	Number,
	Literal(LiteralType),
	Identifier,
	Keyword,
	Symbol,
}
impl fmt::Display for TokenType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

const ID_SYMBOLS: &[u8] = &[b'_', b'?', b'!', b'.', b'@'];
const KEYWORDS: &[&str] = &["let", "val", "fn", "proc", "end", "use"];
const SYMBOLS: &[&str] = &[":", "=", "->"];

#[derive(Debug, Clone)]
pub enum LiteralType {
	Character,
	NormalString,
	Tuple,
	List,
	Array,
	HashSet,
	HashMap,
}
impl fmt::Display for LiteralType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

const LITERAL_PAIRS: &[(&str, u8, LiteralType)] = &[
	("'", b'\'', Character),
	("\"", b'"', NormalString),
	("(", b')', Tuple),
	("[", b']', List),
	("{", b'}', Array),
	("#(", b')', HashSet),
	("#[", b']', HashMap),
];

pub struct Token {
	pub kind: TokenType,
	pub data: String,
	pub start: Position,
	pub end: Position,
}
impl fmt::Debug for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} \"{}\" {} > {}", self.kind, self.data, self.start, self.end)
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
					kind: Number,
					data,
					start: position,
					end: end.clone(),
				});
				position = end;
				continue;
			},
			_ => (),
		};
		if let Some((data, end, index)) = get_literal(buf, &mut position.clone()) {
			tokens.push(Token {
				kind: Literal(LITERAL_PAIRS[index].2.clone()),
				data: data.clone(),
				start: position.clone(),
				end: end.clone(),
			});
			position = end;
			continue;
		};
		if let Some((data, end, is_keyword)) = get_identifier(buf, &mut position.clone()) {
			tokens.push(Token {
				kind: if is_keyword { Keyword } else { Identifier },
				data,
				start: position,
				end: end.clone(),
			});
			position = end;
			continue;
		};
		if let Some((index, end)) = get_symbol(buf, &mut position.clone()) {
			tokens.push(Token {
				kind: Symbol,
				data: SYMBOLS[index].to_owned(),
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

fn get_number(buf: &[u8], position: &mut Position) -> (String, Position) {
	let mut data: String = "".to_owned();
	while position.index < buf.len() && buf[position.index].is_ascii_digit() {
		data.push(buf[position.index] as char);
		position.next(buf);
	};
	(data, position.clone())
}


fn get_literal(buf: &[u8], position: &mut Position) -> Option<(String, Position, usize)> {
	let index_opt: Option<usize> = is_literal(buf, position.clone());
	if index_opt.is_none() { return None };
	let index = index_opt.unwrap();
	position.next(buf);
	let mut data: String = "".to_owned();
	let mut layer: usize = 0;
	while position.index < buf.len() {
		if buf[position.index] == LITERAL_PAIRS[index].1 {
			if layer == 0 {
				return Some((data, position.clone().get_next(buf), index))
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

fn get_identifier(buf: &[u8], position: &mut Position) -> Option<(String, Position, bool)> {
	let mut data: String = "".to_owned();
	while position.index < buf.len() {
		if !buf[position.index].is_ascii_alphanumeric() && !ID_SYMBOLS.contains(&buf[position.index]) {
			break;
		};
		data.push(buf[position.index] as char);
		position.next(buf);
	};
	if data.len() == 0 { return None };
	Some((data.clone(), position.clone(), if KEYWORDS.contains(&data.as_str()) { true } else { false }))
}

fn get_symbol(buf: &[u8], position: &mut Position) -> Option<(usize, Position)> {
	let mut data: String = "".to_owned();
	while position.index < buf.len() && data.len() <= 3 {
		if let Some(index) = SYMBOLS.iter().position(|&symbol| symbol == data.as_str()) {
			return Some((index, position.clone().get_next(buf)))
		};
		data.push(buf[position.index] as char);
		position.next(buf);
	};
	None
}
