const PATH: &str = file!();

use std::{str, usize};

use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Keyword,
    Id,
    Str,
    Int,
    Float,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub start: u32,
    pub end: u32,
}

struct Buffer {
    buf: String,
    pub curr_char: char,
    pub index: u32,
}
impl Buffer {
    fn next_char(&mut self) {
        self.index = self.index.wrapping_add(1);
        self.curr_char = *self.buf.as_bytes().get(self.index as usize).or(Some(&b'\0')).unwrap() as char;
    }

    fn prev_char(&mut self, ch: char) {
        self.index -= 1;
        self.curr_char = ch;
    }

    fn get_token<'a>(&'a self, token: Token) -> Result<&'a str, Error<'a>> {
        const FUNC: &str = "Buffer::get_token";

        self.buf.get(token.start as usize..=token.end as usize).ok_or(Error::code(
            "failed to retrieve token string",
            PATH,
            FUNC,
        ))
    }
}

pub struct Lexer {
    buf: Buffer,
    pub filename: String,
}
impl Lexer {
    pub fn new(buf: Vec<u8>, filename: String) -> Result<Self, Error<'static>> {
        const FUNC: &str = "Parser::new";

        Ok(Lexer {
            buf: Buffer {
                buf: String::from_utf8(buf).map_err(|err| Error::code(&err.to_string(), PATH, FUNC))?,
                curr_char: '\0',
                index: u32::MAX,
            },
            filename,
        })
    }

    pub fn next_token(&mut self) -> Result<Token, Error<'static>> {
        // EOF token
        if self.buf.curr_char == '\0' {
            return Ok(Token {
                token_type: TokenType::EOF,
                start: self.buf.index,
                end: self.buf.index,
            });
        }
        // Go to next non whitespace character
        while self.buf.curr_char.is_whitespace() {
            self.buf.next_char();
        }
        if self.buf.curr_char.is_ascii_alphabetic() || matches!(self.buf.curr_char, '_' | '.') {
            return self.get_id_token();
        }
        if self.buf.curr_char == '"' {
            return self.get_str_token();
        }
        if self.buf.curr_char.is_ascii_digit() {
            return self.get_num_token();
        }
        // Invalid character
        Err(Error::SyntaxError {
            err: "invalid character",
            start: self.buf.index,
            end: self.buf.index + 1,
            filename: self.filename.clone(),
        })
    }

    fn get_id_token(&mut self) -> Result<Token, Error<'static>> {
        let start = self.buf.index;

        self.buf.next_char();
        while self.buf.curr_char.is_ascii_alphanumeric() || self.buf.curr_char == '_' {
            self.buf.next_char();
        }

        Ok(Token {
            token_type: TokenType::Id,
            start,
            end: self.buf.index - 1,
        })
    }

    fn get_str_token(&mut self) -> Result<Token, Error<'static>> {
        let start = self.buf.index;

        self.buf.next_char();
        while self.buf.curr_char != '"' {
            // Check if we reach new line or EOF, then error
            if self.buf.curr_char == '\n' || self.buf.curr_char == '\0' {
                return Err(Error::SyntaxError {
                    err: "expected trailing `\"` to terminate string literal",
                    start,
                    end: self.buf.index,
                    filename: self.filename.clone(),
                });
            }
            self.buf.next_char();
        }
        // Skip trailing `"`
        self.buf.next_char();
        Ok(Token {
            token_type: TokenType::Str,
            start,
            end: self.buf.index - 1,
        })
    }

    fn get_num_token(&mut self) -> Result<Token, Error<'static>> {
        let start = self.buf.index;
        let mut floating_point = false;
        let mut floating_point_pos = start;

        self.buf.next_char();
        while self.buf.curr_char.is_ascii_digit() || (self.buf.curr_char == '.' && !floating_point) {
            // Floating point number
            if self.buf.curr_char == '.' {
                floating_point = true;
                floating_point_pos = self.buf.index;
            }
            self.buf.next_char();
        }
        // Check if last is digit is `.`, if so, remove it
        if floating_point_pos + 1 == self.buf.index {
            self.buf.prev_char('.');
        }
        Ok(Token {
            token_type: if floating_point { TokenType::Float } else { TokenType::Int },
            start,
            end: self.buf.index - 1,
        })
    }
}
