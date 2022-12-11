use std::{
    fs::{DirEntry, File},
    io::{BufRead, BufReader, Lines},
    path::Path,
};

mod direction;
pub use direction::*;
mod gridpoint;
pub use gridpoint::*;

pub const INPUT_FILE: &str = "input.txt";

pub fn read_lines<P: AsRef<Path>>(path: P) -> anyhow::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

// one possible implementation of walking a directory only visiting files
pub fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
