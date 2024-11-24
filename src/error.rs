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
        start_index: u32,
        filename: String,
        line_offsets: Vec<u32>,
    },
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
            _ => unreachable!(
                "{}",
                Self::code(
                    &format!("expected code error, found {self}"),
                    None::<Error>,
                    file!(),
                    line!(),
                    column!()
                )
            ),
        }
    }

    fn parse_cmdline_error(&self) -> String {
        let Self::CmdlineError(err) = self else {
            unreachable!(
                "{}",
                Self::code(
                    &format!("expected command line error, found {self}"),
                    None::<Error>,
                    file!(),
                    line!(),
                    column!()
                )
            );
        };
        let tag = ErrorTag::CmdlineError.to_string().red().bold();
        let err_str = format!(": {err}").bold();
        format!("{tag}{err_str}")
    }

    fn parse_syntax_error(&self) -> String {
        let Self::SyntaxError {
            err,
            buf,
            start_index,
            filename,
            line_offsets,
        } = self
        else {
            unreachable!(
                "{}",
                Error::code("expected syntax error", None::<Error>, file!(), line!(), column!())
            );
        };

        let end = start_index + buf.len() as u32;

        let (start_row, start_column) = file_position(*start_index, line_offsets.to_vec());
        let (end_row, _end_column) = file_position(end, line_offsets.to_vec());

        let tag = ErrorTag::SyntaxError.to_string().red().bold();
        let err_str = format!(": {err}").bold();
        let arrow = "-->".blue();
        let main_error = format!(
            "{tag}{err_str}\n  {arrow} {filename}:{start_row}:{start_column}",
            start_row = start_row + 1,
            start_column = start_column + 1
        );

        let bar = "|".blue();

        if start_row == end_row {
            return format!("{main_error}\n{bar} {buf}");
        }

        let lines = buf
            .split('\n')
            .map(|line| format!("{bar} {line}"))
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
