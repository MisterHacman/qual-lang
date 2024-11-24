mod cmdline;
mod error;
mod file;
mod lexer;
mod parser;

use std::{env, process::exit};

use cmdline::{get_cmdline_args, CmdlineArg};
use error::Error;
use file::{get_line_offsets, read_file};
use lexer::{Lexer, TokenType};
use parser::parse;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        exit(1);
    }
}
fn run() -> Result<(), Error<'static>> {
    let mut cmdline_args = get_cmdline_args(env::args())?;

    let Some(CmdlineArg::File(filename)) = cmdline_args.next() else {
        println!("No input files");
        return Ok(());
    };

    let buf = read_file(filename.clone())?;
    let line_offsets = get_line_offsets(
        String::from_utf8(buf.clone())
            .map_err(|err| Error::code("invalid bytecode", Some(err), file!(), line!(), column!()))?,
    );

    let mut lexer = Lexer::new(&buf, filename.clone())?;
    let mut tokens = vec![];
    loop {
        tokens.push(lexer.next_token(line_offsets.clone())?);
        if tokens.last().unwrap().tag == TokenType::EOF {
            break;
        }
    }

    println!("{tokens:#?}");

    //let ast = parse(lexer, filename, line_offsets)?;

    Ok(())
}
