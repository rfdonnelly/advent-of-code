use std::fs;
use std::path::{Path, PathBuf};

use rayon::prelude::*;

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
mod d16;

fn main() {
    let time = std::time::Instant::now();

    let days: Vec<Box<dyn Fn() -> String + Send + Sync>> = vec![
        Box::new(d01::run),
        Box::new(d02::run),
        Box::new(d03::run),
        Box::new(d04::run),
        Box::new(d05::run),
        Box::new(d06::run),
        Box::new(d07::run),
        Box::new(d08::run),
        Box::new(d09::run),
        Box::new(d10::run),
        Box::new(d11::run),
        Box::new(d12::run),
        Box::new(d13::run),
        Box::new(d14::run),
        Box::new(d16::run),
    ];

    let output = days.par_iter()
        .map(|day| day())
        .collect::<Vec<String>>()
        .join("");

    println!("{}", output);

    println!("total {:?}", time.elapsed());
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
