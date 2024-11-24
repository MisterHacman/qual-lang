use std::str::Chars;

use strumbra::SharedString;

use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    EOF,
    Float,
    Id,
    Int,
    Keyword,
    Paren,
    Str,
}

const PARENS: &str = "()[]{}";

#[derive(Debug, Clone)]
pub struct Token {
    pub tag: TokenType,
    pub buf: SharedString,
    pub start_index: u32,
}

struct Buffer<'a> {
    buf: Chars<'a>,
    pub curr_char: char,
    pub curr_index: u32,
    next_char: Option<char>,
}
impl<'a> Default for Buffer<'a> {
    fn default() -> Self {
        Self {
            buf: "".chars(),
            curr_char: '\0',
            curr_index: 0,
            next_char: None,
        }
    }
}
impl<'a> Buffer<'a> {
    fn next_char(&mut self) {
        self.curr_index = self.curr_index.wrapping_add(1);
        if self.next_char.is_none() {
            self.curr_char = self.buf.next().or(Some('\0')).unwrap();
        } else {
            self.curr_char = self.next_char.unwrap();
            self.next_char = None;
        }
    }

    // Can only be used once at a time
    fn prev_char(&mut self, prev_char: char) {
        self.curr_index -= 1;
        self.next_char = Some(self.curr_char);
        self.curr_char = prev_char;
    }
}

pub struct Lexer<'a> {
    buf: Buffer<'a>,
    pub filename: String,
}
impl<'a> Lexer<'a> {
    pub fn new(buf: &'a Vec<u8>, filename: String) -> Result<Self, Error<'static>> {
        let mut lexer = Lexer {
            buf: Buffer {
                buf: std::str::from_utf8(buf)
                    .map_err(|err| Error::code("invalid bytecode", Some(err), file!(), line!(), column!()))?
                    .chars(),
                curr_char: '\0',
                curr_index: u32::MAX,
                ..Default::default()
            },
            filename,
        };
        lexer.buf.next_char();
        Ok(lexer)
    }

    pub fn next_token(&mut self, line_offsets: Vec<u32>) -> Result<Token, Error<'static>> {
        while self.buf.curr_char.is_ascii_whitespace() {
            self.buf.next_char();
        }

        if self.buf.curr_char == '\0' {
            return Ok(Token {
                tag: TokenType::EOF,
                buf: SharedString::try_from("")
                    .map_err(|err| Error::code("failed to convert to umbra string", Some(err), file!(), line!(), column!()))?,
                start_index: self.buf.curr_index,
            });
        }

        if self.buf.curr_char.is_ascii_alphabetic() || matches!(self.buf.curr_char, '_' | '.') {
            return self.get_id_token();
        }
        if self.buf.curr_char == '"' {
            return self.get_str_token(line_offsets);
        }
        if self.buf.curr_char.is_ascii_digit() {
            return self.get_num_token();
        }
        if PARENS.contains(self.buf.curr_char) {
            let buf = SharedString::try_from(String::from(self.buf.curr_char))
                .map_err(|err| Error::code("failed to convert to umbra string", Some(err), file!(), line!(), column!()))?;
            let start_index = self.buf.curr_index;
            self.buf.next_char();
            return Ok(Token {
                tag: TokenType::Paren,
                buf,
                start_index,
            });
        }

        Err(Error::SyntaxError {
            err: "invalid character",
            buf: SharedString::try_from(format!("{:?}", self.buf.curr_char))
                .map_err(|err| Error::code("failed to convert to umbra string", Some(err), file!(), line!(), column!()))?,
            start_index: self.buf.curr_index,
            filename: self.filename.clone(),
            line_offsets,
        })
    }

    fn get_id_token(&mut self) -> Result<Token, Error<'static>> {
        let start = self.buf.curr_index;
        let mut string = String::from(self.buf.curr_char);

        self.buf.next_char();
        while self.buf.curr_char.is_ascii_alphanumeric() || self.buf.curr_char == '_' {
            string.push(self.buf.curr_char);
            self.buf.next_char();
        }

        let mut token = Token {
            tag: TokenType::Id,
            start_index: start,
            buf: SharedString::try_from(string)
                .map_err(|err| Error::code("failed to convert to umbra string", Some(err), file!(), line!(), column!()))?,
        };
        match token.buf.as_str() {
            "import" | "fn" | "const" | "mutable" => token.tag = TokenType::Keyword,
            _ => (),
        }
        Ok(token)
    }

    fn get_str_token(&mut self, line_offsets: Vec<u32>) -> Result<Token, Error<'static>> {
        let start_index = self.buf.curr_index;
        let mut string = String::from("\"");

        self.buf.next_char();
        while self.buf.curr_char != '"' {
            // Check if we reach new line or EOF, then error
            if self.buf.curr_char == '\n' || self.buf.curr_char == '\0' {
                return Err(Error::SyntaxError {
                    err: "expected trailing `\"` to terminate string literal",
                    buf: SharedString::try_from(string).map_err(|err| {
                        Error::code("failed to convert to umbra string", Some(err), file!(), line!(), column!())
                    })?,
                    start_index,
                    filename: self.filename.clone(),
                    line_offsets,
                });
            }
            string.push(self.buf.curr_char);
            self.buf.next_char();
        }
        string.push('"');
        self.buf.next_char();
        Ok(Token {
            tag: TokenType::Str,
            start_index,
            buf: SharedString::try_from(string)
                .map_err(|err| Error::code("failed to convert to umbra string", Some(err), file!(), line!(), column!()))?,
        })
    }

    fn get_num_token(&mut self) -> Result<Token, Error<'static>> {
        let start = self.buf.curr_index;
        let mut string = String::from(self.buf.curr_char);
        let mut floating_point = false;

        self.buf.next_char();
        while self.buf.curr_char.is_ascii_digit() || (self.buf.curr_char == '.' && !floating_point) {
            // Floating point number
            if self.buf.curr_char == '.' {
                floating_point = true;
            }
            string.push(self.buf.curr_char);
            self.buf.next_char();
        }
        // Check if last digit is `.`, if so, skip it and move buffer back
        if string.chars().last().unwrap() == '.' {
            string.pop();
            self.buf.prev_char('.');
        }
        Ok(Token {
            tag: if floating_point { TokenType::Float } else { TokenType::Int },
            start_index: start,
            buf: SharedString::try_from(string)
                .map_err(|err| Error::code("failed to convert to umbra string", Some(err), file!(), line!(), column!()))?,
        })
    }
}
