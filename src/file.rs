use std::{fs::File, io::Read, iter};

use crate::error::Error;

pub fn read_file(filename: String) -> Result<Vec<u8>, Error<'static>> {
    let mut file =
        File::open(filename).map_err(|err| Error::code("failed to open file", Some(err), file!(), line!(), column!()))?;

    let mut buf = Vec::new();
    let _size = file
        .read_to_end(&mut buf)
        .map_err(|err| Error::code("failed to read file", Some(err), file!(), line!(), column!()))?;

    Ok(buf.to_vec())
}

pub fn get_line_offsets(buf: String) -> Vec<u32> {
    iter::once(0)
        .chain(buf.chars().enumerate().filter(|(_, ch)| *ch == '\n').map(|(i, _)| i as u32))
        .collect::<Vec<_>>()
}

pub fn file_position(start_index: u32, line_offsets: Vec<u32>) -> (u32, u32) {
    let index = binary_search(start_index, line_offsets.clone());
    if index == u32::MAX {
        return (0, 0);
    };
    return (index, start_index - line_offsets[index as usize]);
}

fn binary_search(value: u32, sorted_array: Vec<u32>) -> u32 {
    if sorted_array.len() == 0 {
        return 0;
    }
    if value < sorted_array[0] {
        return u32::MAX;
    }
    if value > *sorted_array.last().unwrap() {
        return sorted_array.len() as u32 - 1;
    }
    if sorted_array.len() < 2 {
        return (value > sorted_array[0]) as u32;
    }
    let mut search_index = sorted_array.len() as u32 / 2;
    let mut change_factor = search_index / 2;
    loop {
        if value < sorted_array[search_index as usize] {
            search_index -= change_factor;
        } else if value > sorted_array[search_index as usize + 1] {
            search_index += change_factor;
        } else {
            return search_index;
        }
        change_factor >>= 2;
        if change_factor == 0 {
            change_factor = 1
        }
    }
}
