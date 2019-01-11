use std::io::{self, Read};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines = {
        let mut lines: Vec<&str> = input
            .lines()
            .collect();

        lines.sort_unstable();

        lines
    };

    lines.iter().for_each(|line| println!("{}", line));

    println!("part1: {}", part1(&lines));
    println!("part2: {}", part2(&lines));

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Guard {
    id: GuardID,
    sleep_times: Vec<Range>,
}

#[derive(Debug, PartialEq)]
struct Range {
    start: Minute,
    end: Minute,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Minute(u32);
#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct GuardID(u32);

#[derive(Debug, PartialEq)]
enum Line {
    BeginsShift(GuardID),
    WakesUp(Minute),
    FallsAsleep(Minute),
}

#[derive(Debug, PartialEq)]
struct MaxMinute {
    minute: u32,
    duration: u32,
}

impl Guard {
    fn new(id: GuardID) -> Guard {
        Guard {
            id: id,
            sleep_times: Vec::new(),
        }
    }

    fn total_time_asleep(&self) -> u32 {
        self.sleep_times
            .iter()
            .map(|range| range.duration())
            .sum()
    }

    fn max_minute(&self) -> MaxMinute {
        let mut minutes = [0u32; 60];

        for range in self.sleep_times.iter() {
            for minute in range.start.0..range.end.0 {
                minutes[minute as usize] += 1;
            }
        }

        let pair = minutes.iter()
            .enumerate()
            .max_by_key(|e| e.1)
            .unwrap();

        MaxMinute {
            minute: pair.0 as u32,
            duration: *pair.1,
        }
    }
}

impl Range {
    fn new() -> Range {
        Range {
            start: Minute(0),
            end: Minute(0),
        }
    }

    fn duration(&self) -> u32 {
        self.end.0 - self.start.0
    }
}

fn parse_line(line: &str) -> Line {
    let minute = line
        .split(|c| c == ':' || c == ']')
        .nth(1).unwrap()
        .parse::<u32>().unwrap();

    let mut words = line.split_whitespace();
    let first_word = words.nth(2).unwrap();
    let first_char = first_word.chars().next().unwrap();

    match first_char {
        'G' => {
            let second_word = words.next().unwrap();
            let id = second_word
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            Line::BeginsShift(GuardID(id))
        }
        'f' => Line::FallsAsleep(Minute(minute)),
        'w' => Line::WakesUp(Minute(minute)),
        _ => panic!("Unexpected first char '{}'", first_char),
    }
}

fn parse_lines(lines: &[&str]) -> Vec<Guard> {
    let mut guards: HashMap<GuardID, Guard> = HashMap::new();

    let lines = lines.iter().map(|line| parse_line(line));

    let mut guardid = GuardID(0);
    let mut range = Range::new();

    for line in lines {
        match line {
            Line::BeginsShift(id) => {
                guardid = id;
                guards.entry(guardid)
                    .or_insert(Guard::new(id));
            }
            Line::FallsAsleep(minute) => {
                range = Range::new();
                range.start = minute;
            }
            Line::WakesUp(minute) => {
                range.end = minute;
                guards.entry(guardid)
                    .and_modify(|guard| {
                        guard.sleep_times.push(range)
                    });
                range = Range::new();
            }
        }
    }

    guards.drain()
        .map(|(_, v)| v)
        .collect()
}


fn most_minutes_asleep(guards: &[Guard]) -> &Guard {
    guards.iter()
        .max_by_key(|guard| guard.total_time_asleep())
        .unwrap()
}

fn most_frequently_asleep(guards: &[Guard]) -> &Guard {
    guards.iter()
        .max_by_key(|guard| guard.max_minute().duration)
        .unwrap()
}

fn part1(lines: &[&str]) -> u32 {
    let guards = parse_lines(lines);
    let guard = most_minutes_asleep(&guards);

    guard.id.0 * guard.max_minute().minute
}

fn part2(lines: &[&str]) -> u32 {
    let guards = parse_lines(lines);
    let guard = most_frequently_asleep(&guards);

    guard.id.0 * guard.max_minute().minute
}

#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn part1_parse_line() {
        assert_eq!(
            parse_line("[1518-04-18 23:59] Guard #3359 begins shift"),
            Line::BeginsShift(GuardID(3359)));

        assert_eq!(
            parse_line("[1518-04-18 23:59] wakes up"),
            Line::WakesUp(Minute(59)));

        assert_eq!(
            parse_line("[1518-04-07 00:26] falls asleep"),
            Line::FallsAsleep(Minute(26)));
    }

    fn example_input() -> Vec<&'static str> {
        vec![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up",
        ]
    }

    #[test]
    fn part1_example() {
        let lines = example_input();

        let guards = parse_lines(&lines);
        let guard = most_minutes_asleep(&guards);

        println!("guards:{:?}", guards);
        println!("guard:{:?}", guard);

        assert_eq!(guard.id, GuardID(10));
        assert_eq!(guard.total_time_asleep(), 50);
        assert_eq!(guard.max_minute().minute, 24);

        assert_eq!(part1(&lines), 240);
    }

    #[test]
    fn part2_example() {
        let lines = example_input();

        let guards = parse_lines(&lines);
        let guard = most_frequently_asleep(&guards);

        assert_eq!(guard.id, GuardID(99));
        assert_eq!(guard.max_minute(), MaxMinute { minute: 45, duration: 3 });

        assert_eq!(part2(&lines), 4455);
    }
}
