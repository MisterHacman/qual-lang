//pub mod parser;
pub mod lexer;
pub mod error;
pub mod position;

//use crate::parser::{ Block, parse };
use crate::lexer::{ Token, tokenize };
use crate::error::{ Error, NormalError, ErrType::* };
use crate::position::TAB_SIZE;

use std::fs::File;
use std::io::{ prelude::Read, Error as IOErr };
use std::env::{ args, current_dir };

fn main() {
	let mut buf: Option<String> = None;
	match run(&mut buf) {
		Ok(()) => (),
		Err(err) => eprintln!("{}", err),
	};
}

fn run(buf: &mut Option<String>) -> Result<(), Error> {
	let args: Vec<String> = args().collect();
	if args.len() < 2 { Err(NormalError("expected input file".to_owned(), CommandError))? };

	let mut options: Vec<String> = vec![];
	for arg in args.clone() {
		if arg.as_bytes()[0] == b'-' {
			options.push(arg[1..].to_owned());
		}
	}

	let working_directory = current_dir()?;
	let file_contents =
		read_file(working_directory.to_str().unwrap(), &args[1])?.replace("\t", &"\t".repeat(TAB_SIZE))
		.strip_suffix("\n")
		.unwrap()
		.to_string();
	*buf = Some(file_contents);
	let mut newlines: Vec<usize> =
		buf
		.as_ref()
		.unwrap()
		.chars()
		.enumerate()
		.filter_map(|(i, ch)| if ch == '\n' { Some(i) } else { None })
		.collect::<Vec<_>>();
	newlines.reverse();
	newlines.push(0);
	newlines.reverse();

	let tokens: Vec<Token> = tokenize(buf.as_ref().unwrap().as_bytes(), &newlines)?;
	println!("{:#?}", tokens);

	Ok(())
}

fn read_file<'a>(directory: &'a str, file_name: &'a str) -> Result<String, IOErr> {
	let mut file: File = File::open(format!("{}/{}", directory, file_name))?;
	let mut buf: String = "".to_owned();
	file.read_to_string(&mut buf)?;
	Ok(buf)
}
