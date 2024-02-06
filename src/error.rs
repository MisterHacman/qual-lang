use crate::error::ErrNum::*;

use crate::position::Position;
use std::io;
use std::fmt::Display;

#[derive(Debug)]
pub enum ErrNum {
	NoInputFile,
	IOError,
	SyntaxError,
}

impl Display for ErrNum {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self)
	}
}

#[derive(Debug)]
pub struct Error<'a> {
	pub num: ErrNum,
	pub data: &'a str,
	pub pos: &'a Option<Position<'a>>,
}

impl<'a> Error<'a> {
	pub fn new(num: ErrNum, data: &'a str, pos: &'a Option<Position<'a>>) -> Self {
		Error { num, data, pos }
	}
}

impl<'a> Display for Error<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.pos {
			Some(pos) => write!(f, "Error {}: {} at [{},{}]", self.num, self.data, pos.line, pos.col),
			None => write!(f, "Error {}: {}", self.num, self.data),
		}
	}
}

impl<'a> From<&'a io::Error> for Error<'a> {
	fn from(value: &'a io::Error) -> Self {
		Error::new(IOError, &value.to_string(), &None)
	}
}
