use std::env::Args;

use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CmdlineArg {
    Option(String),
    File(String),
}

impl CmdlineArg {
    pub fn new(cmdline_args: Args) -> Result<Vec<CmdlineArg>, Error<'static>> {
        cmdline_args.skip(1).map(|arg| Ok(Self::parse_arg(arg)?)).collect()
    }

    fn parse_arg(arg: String) -> Result<CmdlineArg, Error<'static>> {
        if arg.chars().next().unwrap() != '-' {
            return Ok(CmdlineArg::File(arg));
        }
        Ok(CmdlineArg::Option(arg.chars().skip(1).collect()))
    }
}
