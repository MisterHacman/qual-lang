use std::env::Args;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CmdlineArg {
    Option(String),
    File(String),
}

impl CmdlineArg {
    pub fn new(cmdline_args: Args) -> Result<Vec<CmdlineArg>, Box<dyn std::error::Error>> {
        cmdline_args
            .skip(1)
            .fold(Ok(vec![]), |acc, arg| Ok([acc?, vec![CmdlineArg::parse_arg(arg)?]].concat()))
    }

    fn parse_arg(arg: String) -> Result<CmdlineArg, Box<dyn std::error::Error>> {
        if arg.chars().next().unwrap() != '-' {
            return Ok(CmdlineArg::File(arg));
        }
        Ok(CmdlineArg::Option(arg.chars().skip(1).collect()))
    }
}
