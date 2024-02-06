mod lexer;
mod data;
mod error;
mod position;

use lexer::*;
use lexer::TokenData::*;
use error::*;

use std::env;
use std::fs::File;
use std::{io, io::prelude::*};

fn read<'a>(filename: &str) -> Result<&'a[u8], io::Error> {
	let mut file: File = File::open(filename)?;
	let buf: &mut [u8] = &mut [];
	file.read(buf)?;
	Ok(buf)
}

fn main() -> Result<(), String> {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		eprintln!("");
		return Err(format!("{}", Error::new(ErrNum::NoInputFile, "Fatal Error: Missing input files", &None)));
	};
	let buf: &[u8] = match read(&args[1]) {
		Ok(ok) => ok,
		Err(err) => return Err(err.to_string()),
	};
	let tokens = match tokenize(buf) {
		Ok(ok) => ok,
		Err(err) => return Err(err.to_string()),
	};
	return Ok(());
}
