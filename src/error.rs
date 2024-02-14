use crate::error::ErrType::*;

use crate::position::Position;

use std::fmt;
use std::io::{ Error as IOErr };

pub enum ErrType {
	CommandError,
	IOError,
	LexerError,
	ParserError,
	TranspilerError,
}
impl fmt::Debug for ErrType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			CommandError => "Command Error",
			IOError => "IO Error",
			LexerError => "Syntax Error",
			ParserError => "Parse Error",
			TranspilerError => "Transpile Error",
			_ => "Undefined Error",
		})
	}
}

pub struct Error(String);
impl fmt::Debug for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

impl From<IOErr> for Error {
	fn from(value: IOErr) -> Self {
		Error(value.to_string())
	}
}

pub struct NormalError(pub String, pub ErrType);
impl From<NormalError> for Error {
	fn from(value: NormalError) -> Self {
		Error(format!("{:?} {:?}", value.1, value.0))
	}
}

pub struct SyntaxError<'a> {
	pub data: String,
	pub start: Position,
	pub end: Position,
	pub buf: &'a[u8],
}
impl<'a> From<SyntaxError<'a>> for Error {
	fn from(value: SyntaxError) -> Self {
		let data = value.data;
		let pos = value.start.clone();
		let ln_margin = " ".repeat(value.start.line.to_string().len());
		let ln = value.start.line;
		let buf_str = String::from_utf8(value.buf.to_vec()).unwrap();
		let code = buf_str.lines().collect::<Vec<_>>()[value.start.line - 1];
		let bf_err = " ".repeat(value.start.column);
		let at_err = "^".repeat(value.end.index - value.start.index - 1);
		Error(format!(
				"SyntaxError: {data} at {pos}\
				\n{ln_margin} |\
				\n{ln} | {code}\
				\n{ln_margin} | {bf_err}{at_err}"
		))
	}
}
