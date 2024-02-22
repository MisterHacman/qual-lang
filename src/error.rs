use crate::position::Position;

use std::fmt::{ Debug, Display };
use std::io::Error as IOErr;

pub enum ErrType {
	CommandError,
	IOError,
	LexerError,
	ParserError,
	TranspilerError,
}
impl Debug for ErrType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			ErrType::CommandError => "Command Error",
			ErrType::IOError => "IO Error",
			ErrType::LexerError => "Syntax Error",
			ErrType::ParserError => "Parse Error",
			ErrType::TranspilerError => "Transpile Error",
			_ => "Undefined Error",
		})
	}
}

pub struct Error(String);
impl Debug for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Debug::fmt(self, f)
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
	pub newlines: &'a[usize],
}
impl<'a> From<SyntaxError<'a>> for Error {
	fn from(value: SyntaxError) -> Self {
		let data = value.data;
		let pos = value.start;
		let ln_margin = " ".repeat(value.start.line(value.newlines).to_string().len());
		let ln = value.start.line(value.newlines);
		let buf_str = String::from_utf8(value.buf.to_vec()).unwrap();
		let code = buf_str.lines().collect::<Vec<_>>()[value.start.line(value.newlines) - 1];
		let bf_err = " ".repeat(value.start.column(value.newlines));
		let at_err = "^".repeat(value.end.0 - value.start.0 - 1);
		Error(format!(
				"SyntaxError: {data} at {pos}\
				\n{ln_margin} |\
				\n{ln} | {code}\
				\n{ln_margin} | {bf_err}{at_err}"
		))
	}
}
