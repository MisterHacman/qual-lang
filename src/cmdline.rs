use std::env::Args;

use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CmdlineArg {
    Option(String),
    File(String),
}

pub fn get_cmdline_args(mut cmdline_args: Args) -> Result<impl Iterator<Item = CmdlineArg>, Error<'static>> {
    cmdline_args.next();
    Ok(cmdline_args.map(|arg| parse_arg(arg)))
}

fn parse_arg(arg: String) -> CmdlineArg {
    if arg.chars().next().unwrap() != '-' {
        return CmdlineArg::File(arg);
    }
    CmdlineArg::Option(arg[1..].into())
}
