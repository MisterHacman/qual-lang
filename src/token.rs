const PATH: &str = file!();

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Token {
    pub start: u32,
    pub length: u32,
}

pub fn file_position(buf: &[u8], index: usize) -> Result<(u32, u32), Error<'static>> {
    const FUNC: &str = "file_position";

    if index >= buf.len() {
        return Err(Error::new_code("`index` parameter out of bounds", PATH, FUNC));
    }

    let (mut line, mut column) = (0, 0);
    for i in 0..index {
        if buf[i] == b'\n' {
            line += 1;
            column = 0;
            continue;
        }
        column += 1;
    }
    Ok((line, column))
}

pub fn get_line<'a>(buf: &[u8], row: u32) -> Result<Vec<u8>, Error<'static>> {
    const FUNC: &str = "get_line";

    Ok(buf
        .split(|char| char == &b'\n')
        .nth(row as usize)
        .ok_or(Error::new_code("`row` parameter out of bounds", PATH, FUNC))?
        .to_vec())
}
