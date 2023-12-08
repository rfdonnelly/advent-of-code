use std::process::Command;
use std::os::unix::process::CommandExt;

fn main() {
    Command::new("cargo")
        .arg("aoc")
        .exec();
}
