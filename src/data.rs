use crate::data::DataType::*;

use crate::lexer::Token;
use crate::lexer::TokenData::*;
use crate::error::*;
use crate::error::ErrNum::*;
use crate::position::*;

use std::str::from_utf8;

#[derive(Debug, PartialEq, Eq, Hash)]
enum DataType {
	Num,
	Char,
	Str,
	Tuple,
	List,
	Set,
	Map,
}

#[derive(Debug)]
pub struct Data<'a> {
	pub data_type: DataType,
	pub start: &'a Position<'a>,
	pub end: &'a Position<'a>,
}

const DATA_PREFIXES: &[(DataType, (u8, Option<u8>))] = &[
	(Char, (b'\'', None)),
	(Str, (b'"', None)),
	(Tuple, (b'(', None)),
	(List, (b'[', None)),
	(Set,  (b'#', Some(b'['))),
	(Map,  (b'#', Some(b'{'))),
];
const DATA_POSTFIXES: &[(DataType, u8)] = &[
	(Char, b'\''),
	(Str, b'"'),
	(Tuple, b')'),
	(List, b']'),
	(Set, b']'),
	(Map, b'}'),
];

pub fn parse_data<'a>(buf: &'a[u8], pos: &'a Position<'a>) -> Option<Data<'a>> {
	if buf[pos.index].is_ascii_digit() || buf[pos.index] == b'-' {
		return Some(parse_number(buf, &pos))
	};
	let mut data: Option<&DataType> = None;
	for (data_type, prefix) in DATA_PREFIXES.iter() {
		match prefix {
			(char, None) if buf[pos.index] == *char => data = Some(data_type),
			(first, Some(second)) if (first, second) == (&buf[pos.index], &buf[pos.index+1]) => data = Some(data_type),
			_ => (),
		};
	};
	data?;
	Some(parse_block(buf, &mut pos, data.unwrap()))
}

fn parse_block<'a>(buf: &[u8], pos: &'a mut Position<'a>, data_type: &DataType) -> Data<'a> {
	let data_index = DATA_PREFIXES.iter().position(|(key, _)| data_type == key).unwrap();
	let start: Position = pos.clone();
	pos.next();
	let mut text: Vec<u8> = vec![];
	while pos.index < buf.len() {
		if buf[pos.index] == DATA_POSTFIXES[data_index].1 {
			break;
		};
		text.push(buf[pos.index]);
		pos.next();
	};
	Data { data_type: Num, start: pos, end: &pos }
}

fn parse_number<'a>(buf: &'a[u8], pos: &'a Position<'a>) -> Data<'a> {
	let start: Position = pos.clone();
	pos.next();
	let mut text: Vec<u8> = vec![];
	while pos.index < buf.len() {
		if !buf[pos.index].is_ascii_digit() { break; }
		text.push(buf[pos.index]);
		pos.next();
	};
	Data { data_type: Num, start: pos, end: pos }
}
