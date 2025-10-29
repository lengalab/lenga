pub mod merger;

use std::{
    env,
    fs::{self, File},
    io::{self, ErrorKind, Read},
    path::Path,
};

use language::language::{Language, c::C};

use crate::merger::Merger;

fn main() -> io::Result<()> {
    // args: [name, %O, %A, %B]
    let mut it = env::args();
    let _ = it.next();

    let _path_o = it.next().ok_or(io::Error::new(
        ErrorKind::InvalidInput,
        "missing %O (ancestor) path",
    ))?;
    let path_a = it.next().ok_or(io::Error::new(
        ErrorKind::InvalidInput,
        "missing %A (ancestor) path",
    ))?;
    let path_b = it.next().ok_or(io::Error::new(
        ErrorKind::InvalidInput,
        "missing %B (ancestor) path",
    ))?;

    let c = C::new();

    let file_o = File::open(&path_a)?;
    let content_o: Vec<u8> = file_o.bytes().map(|b| b.unwrap()).collect();
    let src_file_o = c
        .parse_nodes(content_o)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;

    let file_a = File::open(&path_a)?;
    let content_a: Vec<u8> = file_a.bytes().map(|b| b.unwrap()).collect();
    let src_file_a = c
        .parse_nodes(content_a)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;

    let file_b = File::open(&path_b)?;
    let content_b: Vec<u8> = file_b.bytes().map(|b| b.unwrap()).collect();
    let src_file_b = c
        .parse_nodes(content_b)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;

    let merged_file = src_file_b;

    let merged_data = c
        .write_to_nodes(merged_file)
        .map_err(|err| io::Error::new(ErrorKind::Other, err))?;

    fs::write(Path::new(&path_a), merged_data)?;

    Ok(())
}
