use std::fs;
use std::path::{Path, PathBuf};

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;

fn main() {
    d01::run();
    d02::run();
    d03::run();
    d04::run();
    d05::run();
}

fn load_file<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    fs::read_to_string(path).unwrap()
}

fn input(day: usize) -> String {
    let path = PathBuf::from(format!(
        "{}/inputs/{:02}.txt",
        env!("CARGO_MANIFEST_DIR"),
        day
    ));
    load_file(path)
}
