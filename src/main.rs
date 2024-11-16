mod cmdline;
mod error;
mod token;

use std::{env::args, fs::File, io::Read};

use cmdline::CmdlineArg;
use error::Error;

fn main() -> Result<(), Error<'static>> {
    let cmdline_args = CmdlineArg::new(args())?;

    let CmdlineArg::File(filename) = &cmdline_args[0] else {
        println!("No input files");
        return Ok(());
    };

    let _buf = read_file(filename);

    Ok(())
}

pub fn read_file(filename: &str) -> Result<Vec<u8>, Error<'static>> {
    let mut file = File::open(filename)?;

    let mut buf = Vec::new();
    let _size = file.read_to_end(&mut buf)?;

    Ok(buf.to_vec())
}
