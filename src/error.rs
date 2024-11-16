const PATH: &str = file!();

use std::fmt::{Debug, Display};

use colored::Colorize;

use crate::token::{file_position, get_line, Token};

enum ErrorTag {
    Code,
    CmdlineError,
    SyntaxError,
}

impl Debug for ErrorTag {
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
        start: &'a Token,
        end: &'a Token,
        filename: &'a str,
    },
}

impl<'a> From<Box<dyn std::error::Error>> for Error<'a> {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        const FUNC: &str = "Error::from";

        if value.is::<Error>() {
            let Ok(err) = value.downcast::<Error>() else {
                return Error::new_code("logic error", PATH, FUNC);
            };
            *err
        } else {
            Error::new_code("expected error enum", PATH, FUNC)
        }
    }
}

impl<'a> Error<'a> {
    pub fn new_code(err: &str, path: &str, func: &str) -> Self {
        Error::Code(format!("{:?}: {err}\n  --> {path:?}:{func}", ErrorTag::Code))
    }

    fn parse_code_error(&self) -> String {
        const FUNC: &str = "Error::parse_code_error";

        match self {
            Self::Code(err) => format!("{:?}: {err}", ErrorTag::Code),
            _ => unreachable!(
                "{}",
                Self::new_code(&format!("expected code error, found {self}"), PATH, FUNC)
            ),
        }
    }

    fn parse_cmdline_error(&self) -> String {
        const FUNC: &str = "Error::parse_cmdline_error";

        match self {
            Self::CmdlineError(err) => format!("{:?}: {err}", ErrorTag::CmdlineError),
            _ => unreachable!(
                "{}",
                Self::new_code(&format!("expected command line error, found {self}"), PATH, FUNC)
            ),
        }
    }

    fn parse_syntax_error(&self) -> String {
        const FUNC: &str = "Error::parse_syntax_error";

        match self {
            Self::SyntaxError {
                err,
                start,
                end,
                filename,
            } => {
                let Ok((start_row, start_column)) = file_position(PATH, start.start as usize) else {
                    return format!("{}", Error::new_code("failed to read input file", PATH, FUNC));
                };
                let Ok((end_row, end_column)) = file_position(PATH, end.start as usize) else {
                    return format!("{}", Error::new_code("failed to read input file", PATH, FUNC));
                };

                let line_nums = (start_row..end_row).collect::<Vec<_>>();
                let max_line_num_len = (end_row + 1).ilog10();

                let lines = match (start_row..=end_row)
                    .map(|row| {
                        Ok(String::from_utf8(get_line(filename, row)?)
                            .map_err(|_err| Error::new_code("failed to retrieve bytes as string", PATH, FUNC))?)
                    })
                    .collect::<Result<Vec<_>, Error>>()
                {
                    Ok(ok) => ok,
                    Err(err) => return err.to_string(),
                };

                let mut start_marker_indent = "".into();
                let mut start_markers = "".into();
                let mut end_marker_indent = "".into();
                let end_markers;

                if start_row != end_row {
                    start_marker_indent = " ".repeat(start_column as usize);
                    start_markers = "v".repeat(lines[0].len() - start_column as usize);
                    end_markers = "^".repeat(end_column as usize + 1);
                } else {
                    end_marker_indent = " ".repeat(end_column as usize);
                    end_markers = "^".repeat(start.length as usize);
                }

                self.show_error(
                    ErrorTag::SyntaxError,
                    err,
                    filename,
                    (start_row as u32, start_column as u32),
                    line_nums,
                    max_line_num_len,
                    lines.to_vec(),
                    (&start_marker_indent, &start_markers),
                    (&end_marker_indent, &end_markers),
                )
            }
            _ => format!("{}", Error::new_code("expected syntax error", PATH, FUNC)),
        }
    }
    fn show_error(
        &self,
        tag: ErrorTag,
        err: &str,
        filename: &str,
        (row, column): (u32, u32),
        line_nums: Vec<u32>,
        max_line_num_len: u32,
        lines: Vec<String>,
        (start_marker_indent, start_markers): (&str, &str),
        (end_marker_indent, end_markers): (&str, &str),
    ) -> String {
        let tag = format!("{tag:?}").red().bold();
        let err = format!(": {err}").bold();
        let main_err = format!("{tag}{err}");

        let arrow = "-->".blue();
        let err_pos = format!("{tag}{err}\n  {arrow} {filename}:{row}{column}");

        let bar_indent = " ".repeat(max_line_num_len as usize);
        let line_bar = "|".blue();

        let err_top = "Γ".red();
        let err_bar = "|".red();
        let err_bottom = "L".red();

        let top_preview = format!("{bar_indent} {line_bar} {err_top} {start_marker_indent}{start_markers}");
        let bottom_preview = format!("{bar_indent} {line_bar} {err_bottom} {end_marker_indent}{end_markers}");

        let err_lines = line_nums
            .iter()
            .zip(lines)
            .map(|(line_num, line)| {
                let bar_indent = " ".repeat((max_line_num_len - line_num.ilog10()) as usize);
                let line_num_str = format!("{line_num}{bar_indent}").blue();
                format!("{line_num_str} {line_bar} {err_bar} {line}")
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
