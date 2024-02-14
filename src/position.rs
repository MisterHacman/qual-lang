use std::fmt;

#[derive(Clone)]
pub struct Position {
	pub index: usize,
	pub line: usize,
	pub column: usize,
}
impl Position {
	pub fn get_next(self, buf: &[u8]) -> Self {
		if buf[self.index] == b'\n' {
			Position { index: self.index + 1, line: self.line + 1, column: 1 }
		} else {
			Position { index: self.index + 1, line: self.line, column: self.column + 1 }
		}
	}
	pub fn next(&mut self, buf: &[u8]) {
		if buf[self.index] == b'\n' {
			self.line += 1;
			self.column = 0;
		} else {
			self.column += 1;
		};
		self.index += 1;
	}
}
impl fmt::Debug for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{},{}]", self.line, self.column)
	}
}
impl fmt::Display for Position {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

pub const START_POSITION: Position = Position { index: 0, line: 1, column: 1 };
