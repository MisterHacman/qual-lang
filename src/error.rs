use std::fmt::{Debug, Display};

use colored::Colorize;
use strumbra::SharedString;

use crate::file::file_position;

#[derive(Debug)]
enum ErrorTag {
    Code,
    CmdlineError,
    SyntaxError,
}

impl Display for ErrorTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tag_str = match self {
            Self::Code => "Code Error",
            Self::CmdlineError => "Command Line Error",
            Self::SyntaxError => "Syntax Error",
        };
        write!(f, "{}", tag_str)
    }
}

pub enum Error<'a> {
    Code(String),
    CmdlineError(String),
    SyntaxError {
        err: &'a str,
        buf: SharedString,
        token_index: u32,
        start_index: u32,
        filename: String,
        line_offsets: Vec<u32>,
    },
}

#[macro_export]
macro_rules! code_err {
    ($result:expr, $info:literal) => {
        $result.map_err(|err| Error::code($info, Some(err), file!(), line!(), column!()))?
    };
    ($info:literal) => {
        Error::code($info, None::<Error>, file!(), line!(), column!())
    };
}

impl<'a> Error<'a> {
    pub fn code(info: &str, err: Option<impl std::error::Error>, filename: &str, line: u32, column: u32) -> Self {
        let tag = ErrorTag::Code.to_string().red().bold();
        let info_str = format!(": {info}").bold();
        let arrow = "-->".blue();
        let bar = "|".blue();
        let err_str = match err {
            None => String::from(""),
            Some(err) => format!("\n\n{bar} {err}"),
        };
        Error::Code(format!("{tag}{info_str}\n  {arrow} {filename}:{line}:{column}{err_str}"))
    }

    fn parse_code_error(&self) -> String {
        match self {
            Self::Code(err) => err.to_string(),
            _ => unreachable!("{}", code_err!("expected code error")),
        }
    }

    fn parse_cmdline_error(&self) -> String {
        let Self::CmdlineError(err) = self else {
            unreachable!("{}", code_err!("expected command line error"));
        };
        let tag = ErrorTag::CmdlineError.to_string().red().bold();
        let err_str = format!(": {err}").bold();
        format!("{tag}{err_str}")
    }

    fn parse_syntax_error(&self) -> String {
        let Self::SyntaxError {
            err,
            buf,
            token_index,
            start_index,
            filename,
            line_offsets,
        } = self
        else {
            unreachable!("{}", code_err!("expected syntax error"));
        };

        let end = token_index + buf.len() as u32;

        let (start_row, start_column) = file_position(*start_index, line_offsets.to_vec());
        let (end_row, mut end_column) = file_position(end, line_offsets.to_vec());
        if end_column == start_column {
            end_column += 1
        }

        let tag = ErrorTag::SyntaxError.to_string().red().bold();
        let err_str = format!(": {err}").bold();
        let arrow = "-->".blue();
        let main_error = format!(
            "{tag}{err_str}\n  {arrow} {filename}:{start_row}:{start_column}",
            start_row = start_row + 1,
            start_column = start_column + 1
        );

        let bar = "|".blue();

        let start_marker = "^".red();
        let marker_padding = " ".repeat(start_column as usize).red();

        if start_row == end_row {
            return format!(
                "{main_error}\n{bar}\n{bar} {buf}\n{bar} {marker_padding}{start_marker}{markers}",
                markers = "~".repeat((end_column - start_column - 1) as usize).red()
            );
        }

        let lines = buf
            .split('\n')
            .enumerate()
            .map(|(index, line)| match index {
                0 => format!(
                    "{bar} {line}\n{bar} {marker_padding}{start_marker}{markers}",
                    markers = "~".repeat(line.len() - start_column as usize).red()
                ),
                _ if index as u32 == end_row - start_row => {
                    format!(
                        "{bar} {line}\n{bar} {markers}",
                        markers = "~".repeat(line.len() as usize).red()
                    )
                }
                _ => format!(
                    "{bar} {line}\n{bar} {markers}",
                    markers = "~".repeat(end_column as usize).red()
                ),
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!("{main_error}\n{lines}")
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Code(_) => write!(f, "{}", self.parse_code_error()),
            Self::CmdlineError(_) => write!(f, "{}", self.parse_cmdline_error()),
            Self::SyntaxError {
                err: _,
                buf: _,
                token_index: _,
                start_index: _,
                filename: _,
                line_offsets: _,
            } => write!(f, "{}", self.parse_syntax_error()),
        }
    }
}

impl<'a> Debug for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl<'a> std::error::Error for Error<'a> {}
