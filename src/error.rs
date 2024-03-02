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
	fn from(mut value: SyntaxError) -> Self {
		let data = value.data;
		let mut pos = value.start;
		let file_pos = pos.show(value.newlines, false);
		let buf_str = String::from_utf8(value.buf.to_vec()).unwrap().replace("\t", " ");

		let start_ln = value.start.line(value.newlines, false);
		let start_col = value.start.column(value.newlines, false);
		let end_ln = value.end.line(value.newlines, true);
		let end_col = value.end.column(value.newlines, true);

		println!("{} {}", pos.show(value.newlines, false), value.end.show(value.newlines, true));

		let code = 
			&buf_str
			.lines()
			.collect::<Vec<_>>()
			[start_ln - 1 ..= end_ln - 1];

		if start_ln != end_ln {
			let max_margin = end_ln.to_string().len();
			let max_margin_text = " ".repeat(max_margin);
			let margin: Vec<_> = (start_ln ..= end_ln).map(|i| " ".repeat(max_margin - i.to_string().len())).collect();
			let ln: Vec<usize> = (start_ln ..= end_ln).collect();
			Error(format!(
				"Syntax Error: {data}\
				\nat {file_pos}\
				\n{max_margin_text} |\
				{}",
				(0 .. code.len()).map(|i| format!(
					"\n{ln_num}{ln_margin} | {ln_code}\
					\n{max_margin_text} | {bf_err}{at_err}",
					ln_num = ln[i],
					ln_margin = margin[i],
					ln_code = code[i],
					bf_err = if i == 0 { " ".repeat(start_col) } else { "".to_owned() },
					at_err = match i {
						j if j == code.len() - 1 => "^".repeat(end_col),
						0 => "^".repeat(code[i].len() - start_col + 1),
						_ => "^".repeat(code[i].len()),
					},
				)).collect::<String>(),
			))
		} else {
			let margin = " ".repeat(start_ln.to_string().len());
			let ln = value.start.line(value.newlines, false);
			let bf_err = " ".repeat(start_col);
			let at_err = "^".repeat(value.end.0 - value.start.0);
			Error(format!(
				"Syntax Error: {data}\
				\nat {file_pos}\
				\n{margin} |\
				\n{ln} | {}\
				\n{margin} | {bf_err}{at_err}",
				code[0],
			))
		}
	}
}
