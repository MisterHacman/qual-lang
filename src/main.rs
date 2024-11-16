mod cmdline;
mod error;
mod token;

use std::{env::args, fs::File, io::Read};

use cmdline::CmdlineArg;
use error::Error;

fn main() -> Result<(), Error<'static>> {
    let cmdline_args = CmdlineArg::new(args())?;

    Ok(())
}

pub fn read_file(filename: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;

    let buf = &mut [];
    let _file_len = file.read(buf)?;

    Ok(buf.to_vec())
}
