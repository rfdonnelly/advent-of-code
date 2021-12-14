use std::fs;
use std::path::{Path, PathBuf};

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;

fn main() {
    d01::run();
    d02::run();
    d03::run();
    d04::run();
    d05::run();
    d06::run();
    d07::run();
    d08::run();
    d09::run();
    d10::run();
    d11::run();
    d12::run();
    d13::run();
    d14::run();
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
