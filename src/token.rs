use std::path::Path;

use crate::{error::Error, read_file};

#[derive(Debug, Clone)]
pub struct Token {
    pub start: u32,
    pub length: u32,
}

pub fn file_position(
    filename: &Box<Path>,
    index: usize,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let func = "file_position";
    let path: Box<Path> = Path::new(file!()).into();

    let buf = read_file(filename)?;
    if index >= buf.len() {
        return Err(Box::new(Error::Code {
            err: "`index` parameter out of bounds",
            path,
            func,
        }));
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

pub fn get_line<'a>(
    filename: &Box<Path>,
    row: usize,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let func = "get_line";
    let path: Box<Path> = Path::new(file!()).into();

    let buf = read_file(filename)?;
    Ok(buf
        .split(|char| char == &b'\n')
        .nth(row)
        .ok_or(Box::new(Error::Code {
            err: "`row` parameter out of bounds",
            path,
            func,
        }))?
        .to_vec())
}
