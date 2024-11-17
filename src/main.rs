mod cmdline;
mod error;
mod file;
mod lexer;
mod parser;

use std::env;

use cmdline::{get_cmdline_args, CmdlineArg};
use error::Error;
use file::read_file;
use lexer::Lexer;
use parser::parse;

fn main() -> Result<(), Error<'static>> {
    let mut cmdline_args = get_cmdline_args(env::args())?;

    let Some(CmdlineArg::File(filename)) = cmdline_args.next() else {
        println!("No input files");
        return Ok(());
    };

    let buf = read_file(filename.clone())?;

    let mut lexer = Lexer::new(buf, filename.clone())?;

    let ast = parse(lexer, filename)?;

    Ok(())
}
