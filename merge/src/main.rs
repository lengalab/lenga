pub mod merger;

use std::{
    env,
    fs::{self, File},
    io::{self, BufReader, ErrorKind, Read},
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

    let mut file_origin = File::open(&path_origin)?;
    let mut content_origin = Vec::new();
    BufReader::new(&mut file_origin)
        .read_to_end(&mut content_origin)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;
    let src_file_origin = c
        .parse_nodes(content_origin)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;

    let mut file_ours = File::open(&path_ours)?;
    let mut content_ours = Vec::new();
    BufReader::new(&mut file_ours)
        .read_to_end(&mut content_ours)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;
    let src_file_ours = c
        .parse_nodes(content_ours)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;

    let mut file_theirs = File::open(&path_theirs)?;
    let mut content_theirs = Vec::new();
    BufReader::new(&mut file_theirs)
        .read_to_end(&mut content_theirs)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;
    let src_file_theirs = c
        .parse_nodes(content_theirs)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?;

    let merger = Merger::new();

    let merged_file = merger
        .merge(src_file_origin, src_file_ours, src_file_theirs)
        .map_err(io::Error::other)?;

    let merged_data = c.write_to_nodes(merged_file).map_err(io::Error::other)?;

    fs::write(Path::new(&path_ours), merged_data)?;

    Ok(())
}
