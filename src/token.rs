const PATH: &str = file!();

use crate::{error::Error, read_file};

#[derive(Debug, Clone)]
pub struct Token {
    pub start: u32,
    pub length: u32,
}

pub fn file_position(filename: &str, index: usize) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    const FUNC: &str = "file_position";

    let buf = read_file(filename)?;
    if index >= buf.len() {
        return Err(Box::new(Error::new_code("`index` parameter out of bounds", PATH, FUNC)));
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

pub fn get_line<'a>(filename: &str, row: u32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    const FUNC: &str = "get_line";

    let buf = read_file(filename)?;
    Ok(buf
        .split(|char| char == &b'\n')
        .nth(row as usize)
        .ok_or(Box::new(Error::new_code("`row` parameter out of bounds", PATH, FUNC)))?
        .to_vec())
}
