use std::fmt::{ Debug, Display };
use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Position(pub usize);

impl Position {
	pub fn line(&self, new_lines: &[usize]) -> usize {
		let index = match new_lines.binary_search(&self.0) {
			Ok(index) => index,
			Err(index) => index,
		};
		index + 1
	}
	pub fn column(&self, new_lines: &[usize]) -> usize {
		let index = match new_lines.binary_search(&self.0) {
			Ok(index) => index,
			Err(index) => index,
		};
		index - new_lines[index - 1]
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
		Position(self.0 + rhs)
	}
}

