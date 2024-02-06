const LINE_FEED: u8 = b'\n';

#[derive(Debug, Clone)]
pub struct Position<'a> {
	pub index: usize,
	pub line: usize,
	pub col: usize,
	pub buf: &'a[u8],
}

impl<'a> Position<'a> {
	pub fn next(&'a mut self) -> &'a mut Self {
		self.index += 1;
		self.col += 1;
		if self.buf[self.index] == LINE_FEED {
			self.line += 1;
			self.col = 0;
		};
		self
	}
	pub fn advance(&'a mut self, length: usize) -> &'a Self {
		let buf: &[u8] = &self.buf[self.index..self.index+length];
		let newlines: usize = buf.iter().filter(|char| **char == b'\n').count();
		self.line += newlines;
		if newlines != 0 {
			self
		} else {
			self.col = buf.iter().rev().position(|char| *char == b'\n').unwrap();
			self
		}
	}
}

pub const BEGIN: Position = Position { index: 0, line: 1, col: 1, buf: &[] };
