const PATH: &str = file!();

use std::fmt::{Debug, Display};

use colored::{ColoredString, Colorize};

use crate::file::{file_position, get_line, read_file};

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

#[derive(Debug)]
pub enum Error<'a> {
    Code(String),
    CmdlineError(String),
    SyntaxError {
        err: &'a str,
        start: u32,
        end: u32,
        filename: String,
    },
}

impl<'a> From<Box<dyn std::error::Error>> for Error<'a> {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        const FUNC: &str = "Error::from";

        if value.is::<Error>() {
            let Ok(err) = value.downcast::<Error>() else {
                return Error::code("logic error", PATH, FUNC);
            };
            *err
        } else {
            Error::code(&value.to_string(), PATH, FUNC)
        }
    }
}

impl<'a> From<std::io::Error> for Error<'a> {
    fn from(value: std::io::Error) -> Self {
        const FUNC: &str = "Error::from";

        Error::code(&value.to_string(), PATH, FUNC)
    }
}

macro_rules! err {
    ($x:expr) => {
        match $x {
            Ok(ok) => ok,
            Err(err) => return err.to_string(),
        }
    };
}

impl<'a> Error<'a> {
    pub fn code(err: &str, path: &str, func: &str) -> Self {
        let tag = ErrorTag::Code.to_string().red().bold();
        let err_str = format!(": {err}").bold();
        let arrow = "-->".blue();
        Error::Code(format!("{tag}{err_str}\n  {arrow} {path}:{func}"))
    }

    fn parse_code_error(&self) -> String {
        const FUNC: &str = "Error::parse_code_error";

        match self {
            Self::Code(err) => err.to_string(),
            _ => unreachable!("{}", Self::code(&format!("expected code error, found {self}"), PATH, FUNC)),
        }
    }

    fn parse_cmdline_error(&self) -> String {
        const FUNC: &str = "Error::parse_cmdline_error";

        match self {
            Self::CmdlineError(err) => format!("{}: {err}", ErrorTag::CmdlineError),
            _ => unreachable!(
                "{}",
                Self::code(&format!("expected command line error, found {self}"), PATH, FUNC)
            ),
        }
    }

    fn parse_syntax_error(&self) -> String {
        const FUNC: &str = "Error::parse_syntax_error";

        let Self::SyntaxError {
            err,
            start,
            end,
            filename,
        } = self
        else {
            return format!("{}", Error::code("expected syntax error", PATH, FUNC));
        };

        let buf = err!(read_file(filename.into()));
        let (start_row, start_column) = err!(file_position(&buf, *start as usize));
        let (end_row, end_column) = err!(file_position(&buf, *end as usize));

        let line_nums = (start_row + 1..=end_row + 1).collect::<Vec<_>>();
        let max_line_num_len = (end_row + 1).ilog10() + 1;
        let lines = err!((start_row..=end_row)
            .map(|row| {
                Ok(String::from_utf8(get_line(&buf, row)?)
                    .map_err(|_err| Error::code("failed to retrieve bytes as string", PATH, FUNC))?)
            })
            .collect::<Result<Vec<_>, Error>>());

        let mut start_marker_indent = "".into();
        let mut start_markers = "".into();
        let mut end_marker_indent = "".into();
        let end_markers;
        if start_row != end_row {
            start_marker_indent = " ".repeat(start_column as usize);
            if start_column != 0 {
                let max_line_len = lines.iter().map(|line| line.len()).max().unwrap();
                start_markers = format!("⌄{}", "—".repeat(max_line_len - start_column as usize - 1)).red();
            }
            end_markers = format!("{}^", "—".repeat(end_column as usize)).red();
        } else {
            end_marker_indent = " ".repeat(end_column as usize);
            end_markers = "^".repeat((end - start) as usize).red();
        }

        self.show_error(
            ErrorTag::SyntaxError.to_string().red().bold(),
            err.bold(),
            filename,
            (start_row as u32, start_column as u32),
            line_nums,
            max_line_num_len,
            lines.to_vec(),
            (&start_marker_indent, start_markers),
            (&end_marker_indent, end_markers),
        )
    }

    fn show_error(
        &self,
        tag: ColoredString,
        err: ColoredString,
        filename: &str,
        (row, column): (u32, u32),
        line_nums: Vec<u32>,
        max_line_num_len: u32,
        lines: Vec<String>,
        (start_marker_indent, start_markers): (&str, ColoredString),
        (end_marker_indent, end_markers): (&str, ColoredString),
    ) -> String {
        let main_err = format!("{tag}\x1b[1m:\x1b[0m {err}");

        let arrow = "-->".blue();
        let err_pos = format!("  {arrow} {filename}:{row}:{column}");

        let bar_indent = " ".repeat(max_line_num_len as usize);
        let line_bar = "│".blue();

        let err_top = "┌".red();
        let err_bar = "│ ".red();
        let err_bottom = "└–".red();

        let top_preview = format!(
            "{bar_indent} {line_bar} {err_top}{start_marker_indent}{start_markers}",
            err_top = if column == 0 { err_top } else { "".into() },
        );
        let bottom_preview = format!(
            "{bar_indent} {line_bar} {err_bottom}{end_marker_indent}{end_markers}",
            err_bottom = if column == 0 { err_bottom } else { "".into() },
        );

        let err_lines = line_nums
            .iter()
            .zip(lines)
            .map(|(line_num, line)| {
                let bar_indent = " ".repeat((max_line_num_len - line_num.ilog10() - 1) as usize);
                let line_num_str = format!("{line_num}{bar_indent}").blue().bold();
                format!(
                    "{line_num_str} {line_bar} {err_bar}{line}",
                    err_bar = if column == 0 { err_bar.clone() } else { "".into() },
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!("{main_err}\n{err_pos}\n{top_preview}\n{err_lines}\n{bottom_preview}")
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Code(_) => write!(f, "{}", self.parse_code_error()),
            Self::CmdlineError(_) => write!(f, "{}", self.parse_cmdline_error()),
            Self::SyntaxError {
                err: _,
                start: _,
                end: _,
                filename: _,
            } => write!(f, "{}", self.parse_syntax_error()),
        }
    }
}

impl<'a> std::error::Error for Error<'a> {}
