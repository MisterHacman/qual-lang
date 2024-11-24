use std::iter;

use crate::{
    error::Error,
    lexer::{Lexer, TokenType},
};

trait Node {}

pub struct ItemNode {}
impl Node for ItemNode {}

pub fn parse<'a>(
    mut lexer: Lexer,
    filename: String,
    line_offsets: Vec<u32>,
) -> Result<impl Iterator<Item = ItemNode>, Error<'a>> {
    let token = lexer.next_token()?;

    match token.tag {
        TokenType::Keyword => (),
        TokenType::Eof => return Ok(iter::empty::<ItemNode>()),
        _ => {
            return Err(Error::SyntaxError {
                err: "expected item, not identifier",
                buf: token.buf,
                token_index: token.start_index,
                start_index: token.start_index,
                filename,
                line_offsets,
            })
        }
    }

    Ok(iter::empty::<ItemNode>())
}
