use std::{
    fmt::{Debug, Display},
    path::Path,
};

use crate::token::Token;

#[derive(Debug)]
pub enum Error<'a> {
    CmdlineError(&'a str),
    SyntaxError {
        err: &'a str,
        token: Token,
        filename: Box<Path>,
    },
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CmdlineError(err) => write!(f, "Command Line Error: {}", err),
            Self::SyntaxError {
                err,
                token,
                filename,
            } => {
                let Some(filename_str) = filename.to_str() else {
                    return write!(f, "IO Error: failed to retrieve path");
                };
                let Ok((row, column)) = file_position(&filename, token.start as usize) else {
                    return write!(f, "IO Error: failed to read input file");
                };
                let spaces = " ".repeat((row + 1).ilog10() as usize);
                let Ok(line) = get_line(&filename, row) else {
                    return write!(f, "IO Error: failed to read input file");
                };
                let marker_indent = " ".repeat((column + 1).ilog10() as usize);
                let markers = "^".repeat(token.length as usize);
                write!(f, "Syntax Error: {err}\n --> {filename_str}:{row}:{column}\n{spaces} |\n{row} | {line}\n{spaces} | {marker_indent}{markers}\n{spaces} |")
            }
        }
    }
}

impl<'a> std::error::Error for Error<'a> {}

fn file_position(
    filename: &Box<Path>,
    index: usize,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    todo!()
}

fn get_line<'a>(filename: &Box<Path>, row: usize) -> Result<&'a str, Box<dyn std::error::Error>> {
    todo!()
}
