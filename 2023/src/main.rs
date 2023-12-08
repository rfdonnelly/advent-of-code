use cargo_util::ProcessBuilder;

fn main() {
    ProcessBuilder::new("cargo")
        .arg("aoc")
        .exec_replace()
        .unwrap()
}
