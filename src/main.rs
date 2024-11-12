mod cmdline;
mod error;
mod token;

use std::{env::args, fs::File, io::Read};

use cmdline::CmdlineArg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmdline_args = CmdlineArg::new(args())?;

    println!("{:?}", cmdline_args);

    Ok(())
}

pub fn read_file(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let buf = &mut [];

    let _file_len = file.read(buf)?;

    Ok(String::from_utf8(buf.to_vec())?)
}
