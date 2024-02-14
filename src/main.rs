pub mod parser;
pub mod lexer;
pub mod error;
pub mod position;

use crate::parser::{ Block, parse };
use crate::lexer::{ Token, tokenize };
use crate::error::{ Error, NormalError, ErrType::* };

use std::fs::File;
use std::io::{ prelude::Read, Error as IOErr };
use std::env::{ args, current_dir };
use std::path::PathBuf;

use std::collections::HashMap;

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

	let working_directory: PathBuf = current_dir()?;
	let file_contents: String = read_file(working_directory.to_str().unwrap(), &args[1])?;
	*buf = Some(file_contents);

	let tokens: Vec<Token> = tokenize(buf.as_ref().unwrap().as_bytes())?;
	if options.contains(&"p".to_owned()) {
		for token in tokens {
			println!("{}", token.data);
		};
	} else {
		println!("{:#?}", tokens);
	};

	let nodes: HashMap<String, Vec<Block>> = parse(tokens)?;
	Ok(())
}

fn read_file<'a>(directory: &'a str, file_name: &'a str) -> Result<String, IOErr> {
	let mut file: File = File::open(format!("{}/{}", directory, file_name))?;
	let mut buf: String = "".to_owned();
	file.read_to_string(&mut buf)?;
	Ok(buf)
}
