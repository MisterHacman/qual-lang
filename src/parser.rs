const PATH: &str = file!();

use std::iter;

use crate::{
    error::Error,
    lexer::{Lexer, TokenType},
};

trait Node {}

pub struct ItemNode {}
impl Node for ItemNode {}

pub fn parse(mut lexer: Lexer) -> Result<impl Iterator<Item = ItemNode>, Error<'static>> {
    const FUNC: &str = "Parser::new";

    let token = lexer.next_token()?;

    match token.token_type {
        TokenType::Keyword => (),
        TokenType::EOF => return Ok(iter::empty::<ItemNode>()),
        _ => {
            return Err(Error::SyntaxError {
                err: "expected item, not identifier",
                start: token.start,
                end: token.end,
                filename: lexer.filename,
            })
        }
    }

    Ok(iter::empty::<ItemNode>())
}
