pub mod merger;

use std::{
    env,
    fs::{self, File},
    io::{self, ErrorKind, Read},
    path::Path,
};

use lenga::language::{Language, c::C};

use crate::merger::Merger;

fn main() -> io::Result<()> {
    // args: [name, %O, %A, %B]
    let mut it = env::args();
    let _ = it.next();

    let path_origin = it.next().ok_or(io::Error::new(
        ErrorKind::InvalidInput,
        "missing %O (ancestor) path",
    ))?;
    let path_ours = it.next().ok_or(io::Error::new(
        ErrorKind::InvalidInput,
        "missing %A (ancestor) path",
    ))?;
    let path_theirs = it.next().ok_or(io::Error::new(
        ErrorKind::InvalidInput,
        "missing %B (ancestor) path",
    ))?;

    let c = C::new();

    let file_origin = File::open(&path_origin)?;
    let content_origin: Vec<u8> = file_origin.bytes().map(|b| b.unwrap()).collect();
    let src_file_origin = c
        .parse_nodes(content_origin)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;

    let file_ours = File::open(&path_ours)?;
    let content_ours: Vec<u8> = file_ours.bytes().map(|b| b.unwrap()).collect();
    let src_file_ours = c
        .parse_nodes(content_ours)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;

    let file_theirs = File::open(&path_theirs)?;
    let content_theirs: Vec<u8> = file_theirs.bytes().map(|b| b.unwrap()).collect();
    let src_file_theirs = c
        .parse_nodes(content_theirs)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;

    let merger = Merger::new();

    let merged_file = merger
        .merge(src_file_origin, src_file_ours, src_file_theirs)
        .map_err(|err| io::Error::new(ErrorKind::Other, err))?;

    let merged_data = c
        .write_to_nodes(merged_file)
        .map_err(|err| io::Error::new(ErrorKind::Other, err))?;

    fs::write(Path::new(&path_ours), merged_data)?;

    Ok(())
}
