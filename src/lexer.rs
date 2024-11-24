use std::{
    fmt::Debug,
    str::{self, Chars},
};

use strumbra::SharedString;

use crate::{code_err, error::Error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Char,
    Eof,
    Float,
    Id,
    Int,
    Keyword,
    Paren,
    Str,
}

const PARENS: &str = "()[]{}";

#[derive(Clone)]
pub struct Token {
    pub tag: TokenType,
    pub buf: SharedString,
    pub start_index: u32,
}
impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.buf.as_bytes().get(0).or(Some(&b'\0')).unwrap() != &b'\0' {
            write!(f, "{:?}: {}", self.tag, self.buf)
        } else {
            write!(f, "{:?}", self.tag)
        }
    }
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
            curr_index: u32::MAX,
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
    line_offsets: Vec<u32>,
}
impl<'a> Lexer<'a> {
    pub fn new(buf: &'a Vec<u8>, filename: String, line_offsets: Vec<u32>) -> Result<Self, Error<'static>> {
        let mut lexer = Lexer {
            buf: Buffer {
                buf: code_err!(str::from_utf8(buf), "invalid bytecode").chars(),
                ..Default::default()
            },
            filename,
            line_offsets,
        };
        lexer.buf.next_char();
        Ok(lexer)
    }

    pub fn next_token(&mut self) -> Result<Token, Error<'static>> {
        while self.buf.curr_char.is_ascii_whitespace() {
            self.buf.next_char();
        }

        if self.buf.curr_char == '\0' {
            return Ok(Token {
                tag: TokenType::Eof,
                buf: Self::convert_umbra(String::from(self.buf.curr_char))?,
                start_index: self.buf.curr_index,
            });
        }

        if self.buf.curr_char.is_ascii_alphabetic() || matches!(self.buf.curr_char, '_' | '.') {
            return self.get_id_token();
        }
        if self.buf.curr_char == '"' {
            return self.get_str_token();
        }
        if self.buf.curr_char == '\'' {
            return self.get_char_token();
        }
        if self.buf.curr_char.is_ascii_digit() {
            return self.get_num_token();
        }
        if PARENS.contains(self.buf.curr_char) {
            let buf = Self::convert_umbra(String::from(self.buf.curr_char))?;
            let start_index = self.buf.curr_index;
            self.buf.next_char();
            return Ok(Token {
                tag: TokenType::Paren,
                buf,
                start_index,
            });
        }

        self.syntax_error(
            "invalid character",
            String::from(self.buf.curr_char),
            self.buf.curr_index,
            self.buf.curr_index,
        )?;
        unreachable!()
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
            buf: Self::convert_umbra(string)?,
        };
        match token.buf.as_str() {
            "import" | "fn" | "const" | "mutable" => token.tag = TokenType::Keyword,
            _ => (),
        }
        Ok(token)
    }

    fn get_str_token(&mut self) -> Result<Token, Error<'static>> {
        let start_index = self.buf.curr_index;
        let mut string = String::from("\"");

        self.buf.next_char();
        while self.buf.curr_char != '"' {
            // Check if we reach new line or EOF, then error
            if self.buf.curr_char == '\n' || self.buf.curr_char == '\0' {
                self.syntax_error("expected trailing `\"`", string.clone(), start_index, self.buf.curr_index)?
            }
            if self.buf.curr_char == '\\' {
                self.buf.next_char();
                self.get_char_escape(&mut string, start_index)?;
                continue;
            }
            string.push(self.buf.curr_char);
            self.buf.next_char();
        }
        string.push('"');
        self.buf.next_char();
        Ok(Token {
            tag: TokenType::Str,
            start_index,
            buf: Self::convert_umbra(string)?,
        })
    }

    fn get_char_token(&mut self) -> Result<Token, Error<'static>> {
        let start_index = self.buf.curr_index;
        let mut string = String::from("'");

        self.buf.next_char();
        match self.buf.curr_char {
            '\'' => {
                string.push('\'');
                self.syntax_error("expected character", string.clone(), start_index, self.buf.curr_index)?
            }
            '\\' => {
                self.buf.next_char();
                self.get_char_escape(&mut string, start_index)?;
            }
            ch => {
                string.push(ch);
                self.buf.next_char();
            }
        };

        string.push(self.buf.curr_char);
        if self.buf.curr_char != '\'' {
            self.syntax_error("expected trailing `'`", string.clone(), start_index, self.buf.curr_index)?
        }

        self.buf.next_char();
        Ok(Token {
            tag: TokenType::Char,
            buf: Self::convert_umbra(string)?,
            start_index,
        })
    }

    fn get_char_escape(&mut self, string: &mut String, start_index: u32) -> Result<(), Error<'static>> {
        string.push('\\');
        match self.buf.curr_char {
            ch @ ('"' | '\\' | '0' | 't' | 'n' | 'r') => {
                string.push(ch);
                self.buf.next_char();
            }
            'x' => {
                string.push('x');
                self.buf.next_char();

                let first = self.buf.curr_char;
                string.push(first);
                if !self.buf.curr_char.is_ascii_hexdigit() {
                    self.syntax_error(
                        "expected two hex values after `\\x`",
                        string.to_string(),
                        start_index,
                        self.buf.curr_index,
                    )?;
                };
                self.buf.next_char();

                let second = self.buf.curr_char;
                string.push(second);
                if !self.buf.curr_char.is_ascii_hexdigit() {
                    self.syntax_error(
                        "expected two hex values after `\\x`",
                        string.to_string(),
                        start_index,
                        self.buf.curr_index,
                    )?;
                }
                self.buf.next_char();
            }
            _ => self.syntax_error(
                "invalid character escape",
                string.to_string(),
                start_index,
                self.buf.curr_index,
            )?,
        };
        Ok(())
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
            buf: Self::convert_umbra(string)?,
        })
    }

    fn syntax_error(
        &self,
        info: &'static str,
        string: String,
        token_index: u32,
        start_index: u32,
    ) -> Result<(), Error<'static>> {
        Err(Error::SyntaxError {
            err: info,
            buf: Self::convert_umbra(string)?,
            token_index,
            start_index,
            filename: self.filename.clone(),
            line_offsets: self.line_offsets.clone(),
        })
    }

    fn convert_umbra(string: String) -> Result<SharedString, Error<'static>> {
        Ok(code_err!(SharedString::try_from(string), "failed to convert to umbra string"))
    }
}
