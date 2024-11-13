use std::{
    fmt::{Debug, Display},
    path::Path,
};

use crate::token::{file_position, get_line, Token};

#[derive(Debug)]
pub enum Error<'a> {
    Code {
        err: &'a str,
        path: Box<Path>,
        func: &'a str,
    },
    CmdlineError(&'a str),
    SyntaxError {
        err: &'a str,
        token: &'a Token,
        path: Box<Path>,
    },
}

impl<'a> Error<'a> {
    fn parse_code_error(&self) -> String {
        let func = "Error::parse_code_error";
        let path: Box<Path> = Path::new(file!()).into();

        match self {
            Self::Code {
                err,
                path: file,
                func,
            } => {
                let Some(file_str) = file.to_str() else {
                    return format!(
                        "{}",
                        Self::Code {
                            err: "failed to retrieve path as string",
                            path,
                            func,
                        },
                    );
                };
                format!("Error in compiler code: {err}\n  --> {file_str}:{func}")
            }
            _ => format!(
                "{}",
                Self::Code {
                    err: "expected code error",
                    path,
                    func,
                }
            ),
        }
    }

    fn parse_syntax_error(&self) -> String {
        let func = "Error::parse_syntax_error";
        let path: Box<Path> = Path::new(file!()).into();

        match self {
            Self::SyntaxError {
                err,
                token,
                path: file,
            } => {
                let Some(file_str) = file.to_str() else {
                    return format!(
                        "{}",
                        Error::Code {
                            err: "failed to retrieve path as string".into(),
                            path,
                            func,
                        },
                    );
                };
                let Ok((row, column)) = file_position(&file, token.start as usize) else {
                    return format!(
                        "{}",
                        Error::Code {
                            err: "failed to read input file",
                            path,
                            func,
                        },
                    );
                };
                let spaces = " ".repeat((row + 1).ilog10() as usize);
                let Ok(line) = get_line(&file, row) else {
                    return format!(
                        "{}",
                        Error::Code {
                            err: "failed to read input file",
                            path,
                            func,
                        },
                    );
                };
                let marker_indent = " ".repeat((column + 1).ilog10() as usize);
                let markers = "^".repeat(token.length as usize);
                format!("Syntax Error: {err}\n  --> {file_str}:{row}:{column}\n{spaces} |\n{row} | {line:?}\n{spaces} | {marker_indent}{markers}")
            }
            _ => format!(
                "{}",
                Error::Code {
                    err: "expected syntax error",
                    path,
                    func,
                },
            ),
        }
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Code {
                err: _,
                path: _,
                func: _,
            } => write!(f, "{}", self.parse_code_error()),
            Self::CmdlineError(err) => write!(f, "Command Line Error: {err}"),
            Self::SyntaxError {
                err: _,
                token: _,
                path: _,
            } => write!(f, "{}", self.parse_syntax_error()),
        }
    }
}

impl<'a> std::error::Error for Error<'a> {}
