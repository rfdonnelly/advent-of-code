use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Line {
    Nop,
    Push { name: String },
    Pop,
    Dir { name: String },
    File(File)
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let tokens = s.split(" ").collect::<Vec<_>>();
        match tokens[..] {
            ["$", "ls"] => Line::Nop,
            ["$", "cd", "/"] => Line::Nop,
            ["$", "cd", ".."] => Line::Pop,
            ["$", "cd", name] => Line::Push { name: name.to_string() },
            ["dir", name] => Line::Dir { name: name.to_string() },
            [size, name] => Line::File(File { name: name.to_string(), size: size.parse().unwrap() }),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct File {
    name: String,
    size: usize
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dir {
    path: String,
    dirs: Vec<String>,
    files: Vec<File>,
}

impl Default for Dir {
    fn default() -> Self {
        Self {
            path: Default::default(),
            dirs: Default::default(),
            files: Default::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FileSystem {
    dirs: HashMap<String, Dir>,
}

impl From<&[Line]> for FileSystem {
    fn from(lines: &[Line]) -> Self {
        let mut dirs: HashMap<String, Dir> = HashMap::new();
        let mut path = Vec::new();
        let mut cur_dir = dirs.entry("".to_string()).or_default();

        for line in lines {
            match line {
                Line::Push { name } => {
                    path.push(name.clone());
                    cur_dir = dirs.entry(path.join("/")).or_default();
                    cur_dir.path = path.join("/");
                }
                Line::Pop => {
                    path.pop().unwrap();
                    cur_dir = dirs.entry(path.join("/")).or_default();
                }
                Line::Dir { name } => {
                    let mut path = path.clone();
                    path.push(name.to_string());
                    cur_dir.dirs.push(path.join("/"));
                }
                Line::File(file) => {
                    cur_dir.files.push(file.clone());
                }
                _ => {}
            }
        }

        FileSystem { dirs }
    }
}

impl FileSystem {
    fn size_recursive(&self, path: &str, size_map: &mut HashMap<String, usize>) -> usize {
        let dir = self.dirs.get(path).unwrap();
        let files_size: usize = dir.files.iter().map(|file| file.size).sum();
        let dirs_size: usize = dir
            .dirs
            .iter()
            .map(|dir_path| {
                self.size_recursive(dir_path, size_map)
            })
            .sum();

        let size = files_size + dirs_size;
        size_map.insert(path.to_string(), size);

        size
    }
}

type Input = Vec<Line>;

#[aoc_generator(day7)]
fn parse(input: &str) -> Input {
    input.lines().map(Line::from).collect()
}

#[aoc(day7, part1)]
fn p1(input: &Input) -> usize {
    let fs = FileSystem::from(input.as_slice());
    let mut sizes = HashMap::new();
    fs.size_recursive("", &mut sizes);
    sizes.values().filter(|&&size| size <= 100000).sum()
}

#[aoc(day7, part2)]
fn p2(input: &Input) -> usize {
    const CAPACITY: usize = 70_000_000;
    const NEEDED_FREE_SPACE: usize = 30_000_000;
    const MAX_USED: usize = CAPACITY - NEEDED_FREE_SPACE;
    let fs = FileSystem::from(input.as_slice());
    let mut sizes = HashMap::new();
    let total = fs.size_recursive("", &mut sizes);
    let min_space_to_free = total - MAX_USED;
    sizes.values().filter(|&&size| size >= min_space_to_free).copied().min().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "};

    #[test]
    fn test_parse() {
        let expected = vec![
            Line::Nop,
            Line::Nop,
            Line::Dir { name: "a".to_string() },
            Line::File(File { name: "b.txt".to_string(), size: 14848514 }),
            Line::File(File { name: "c.dat".to_string(), size: 8504156 }),
            Line::Dir { name: "d".to_string() },
            Line::Push { name: "a".to_string() },
            Line::Nop,
            Line::Dir { name: "e".to_string() },
            Line::File(File { name: "f".to_string(), size: 29116 }),
            Line::File(File { name: "g".to_string(), size: 2557 }),
            Line::File(File { name: "h.lst".to_string(), size: 62596 }),
            Line::Push { name: "e".to_string() },
            Line::Nop,
            Line::File(File { name: "i".to_string(), size: 584 }),
            Line::Pop,
            Line::Pop,
            Line::Push { name: "d".to_string() },
            Line::Nop,
            Line::File(File { name: "j".to_string(), size: 4060174 }),
            Line::File(File { name: "d.log".to_string(), size: 8033020 }),
            Line::File(File { name: "d.ext".to_string(), size: 5626152 }),
            Line::File(File { name: "k".to_string(), size: 7214296 }),
        ];
        assert_eq!(parse(INPUT), expected);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(&parse(INPUT)), 95437);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&parse(INPUT)), 24933642);
    }
}
