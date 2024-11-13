use std::{env::Args, path::Path};

use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CmdlineArg {
    Option(String),
    File(Box<Path>),
}

impl CmdlineArg {
    pub fn new(cmdline_args: Args) -> Result<Vec<CmdlineArg>, Box<dyn std::error::Error>> {
        cmdline_args.skip(1).fold(Ok(vec![]), |acc, string| {
            Ok([acc?, parse_arg(string)?.to_vec()].concat())
        })
    }
}

fn parse_arg(arg: String) -> Result<Vec<CmdlineArg>, Box<dyn std::error::Error>> {
    if arg.chars().next().unwrap() != '-' {
        let path = Path::new(&arg);
        return Ok(vec![CmdlineArg::File(path.into())]);
    }
    Ok(parse_option_arg(arg)?)
}

fn parse_option_arg(arg: String) -> Result<Vec<CmdlineArg>, Box<dyn std::error::Error>> {
    let mut arg_iter = arg.chars();
    arg_iter.next();

    let letter = arg_iter
        .next()
        .ok_or(Error::CmdlineError("expected options after `-`"))?;
    if letter == '-' {
        if arg.len() <= 1 {
            return Err(Box::new(Error::CmdlineError("expected options after `--`")));
        }
        return Ok(vec![CmdlineArg::Option(arg[2..].to_owned())]);
    }
    Ok(arg[1..]
        .as_bytes()
        .iter()
        .map(|letter: &u8| CmdlineArg::Option((*letter as char).to_string()))
        .collect::<Vec<_>>())
}
