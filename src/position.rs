use std::fmt::{ Debug, Display };
use std::ops::{ Add, Sub };

pub const TAB_SIZE: usize = 8;

#[derive(Clone, Copy)]
pub struct Position (
	pub usize,
	Option<usize>,
	Option<usize>,
);

impl Position {
	pub fn new(index: usize) -> Position {
		Position(index, None, None)
	}
	pub fn line(&mut self, newlines: &[usize], end: bool) -> usize {
		if self.1.is_some() { return self.1.unwrap() };
		let index = match newlines.binary_search(&self.0) {
			Ok(index) => index,
			Err(index) => index,
		};
		self.1 =
			if self.0 == 0 { Some(1) }
			else if end && self.0 - newlines[index - 1] == 1 { Some(index - 1) }
			else { Some(index) };
		self.1.unwrap()
	}
	pub fn column(&mut self, newlines: &[usize], end: bool) -> usize {
		if self.2.is_some() { return self.2.unwrap() };
		self.2 = Some(self.0 - newlines[self.line(newlines, end) - 1]);
		self.2.unwrap()
	}
	pub fn show(&mut self, newlines: &[usize], end: bool) -> String {
		format!("{}:{}", self.line(newlines, end), self.column(newlines, end))
	}
}

impl Debug for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
impl Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Debug::fmt(self, f)
	}
}

impl Add<usize> for Position {
	type Output = Position;
	fn add(self, rhs: usize) -> Self::Output {
		Position::new(self.0 + rhs)
	}
}
impl Sub<usize> for Position {
	type Output = Position;
	fn sub(self, rhs: usize) -> Self::Output {
		Position::new(self.0 - rhs)
	}
}
