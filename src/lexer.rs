use crate::lexer::TokenData::*;

use crate::data::*;
use crate::error::*;
use crate::error::ErrNum::*;
use crate::position::*;

use std::str::from_utf8;

const KEYWORDS: [&[u8]; 2] = [b"=", b"::"];

#[derive(Debug)]
pub enum TokenData<'a> {
	Ident(&'a str),
	Keyword(&'a str),
	Literal(&'a str),
}

#[derive(Debug)]
pub struct Token<'a> {
	data: TokenData<'a>,
	start: &'a Position<'a>,
	end: &'a Position<'a>,
}

pub fn tokenize<'a>(buf: &'a[u8]) -> Result<Vec<Token<'a>>, Error<'a>> {
	let mut tokens: Vec<Token> = vec![];
	let mut pos: &mut Position = &mut BEGIN;
	let mut token: Token;
	while pos.index < buf.len() {
		token = get_token(buf, pos)?;
		pos = &mut token.end;
		tokens.push(token);
	};
	Ok(tokens)
}

fn get_token<'a>(buf: &[u8], pos: &'a mut Position<'a>) -> Result<Token<'a>, Error<'a>> {
	if buf[pos.index].is_ascii_whitespace() {
		let mut next_pos: Position = pos.clone();
		next_pos.next();
		return get_token(buf, &mut next_pos);
	};
	if let Some(data) = parse_data(buf, &pos) {
		return Ok(Token {
			data: Literal(from_utf8(&buf[data.start.index..=data.end.index]).unwrap()),
			start: data.start,
			end: data.end
		})
	};
	let token: &[u8] = buf.split(|char| char.is_ascii_whitespace()).collect::<Vec<_>>().first().unwrap();
	get_id_token(token, pos)
}

fn next_id(buf: &[u8], index: usize) -> &[u8] {
	let mut i: usize = index;
	while !buf[i].is_ascii_whitespace() { i += 1 }
	&buf[index..i]
}

fn get_id_token<'a>(token: &[u8], pos: &'a Position<'a>) -> Result<Token<'a>, Error<'a>> {
	let end: &Position = pos.advance(token.len());
	match token {
		id if KEYWORDS.contains(&id) => Ok(Token {
			data: Keyword(from_utf8(id).unwrap()),
			start: pos,
			end,
		}),
		id => Ok(Token { data: Ident(from_utf8(id).unwrap()), start: pos, end: end }),
	}
}
