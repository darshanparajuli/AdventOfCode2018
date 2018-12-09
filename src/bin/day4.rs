extern crate aoc;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Time {
    hour: u8,
    minute: u8,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Timestamp {
    date: Date,
    time: Time,
}

impl FromStr for Timestamp {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = Date {
            year: s[1..5].parse::<u16>()?,
            month: s[6..8].parse::<u8>()?,
            day: s[9..11].parse::<u8>()?,
        };
        let time = Time {
            hour: s[12..14].parse::<u8>()?,
            minute: s[15..17].parse::<u8>()?,
        };
        Ok(Timestamp { date, time })
    }
}

#[derive(Debug)]
enum GuardState {
    BeginShift(Timestamp),
    FallAsleep(Timestamp),
    WakeUp(Timestamp),
}

fn parse(path: &str) -> Result<HashMap<usize, Vec<GuardState>>, io::Error> {
    let file = File::open(path)?;
    let bufreader = BufReader::new(file);

    let mut lines = bufreader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let timestamp = line[0..18].parse::<Timestamp>().unwrap();
            (timestamp, line[19..].into())
        }).collect::<Vec<(Timestamp, String)>>();

    lines.sort_by(|(ref t0, _), (ref t1, _)| t0.cmp(&t1));

    let mut guards: HashMap<usize, Vec<GuardState>> = HashMap::new();

    let mut index = 0;

    // Assume first one is always `BeginShift`
    while index < lines.len() {
        let (ref timestamp, ref msg) = &lines[index];
        let id = {
            let index = msg[7..].find(' ').unwrap();
            msg[7..7 + index].parse::<usize>().unwrap()
        };

        index += 1;

        let mut states = vec![];
        states.push(GuardState::BeginShift(timestamp.clone()));

        while index < lines.len() {
            let (ref timestamp, ref msg) = &lines[index];
            match &msg[0..5] {
                "wakes" => states.push(GuardState::WakeUp(timestamp.clone())),
                "falls" => states.push(GuardState::FallAsleep(timestamp.clone())),
                _ => break,
            }

            index += 1;
        }

        match guards.entry(id) {
            Entry::Occupied(ref mut o) => {
                o.get_mut().extend(states);
            }
            Entry::Vacant(v) => {
                v.insert(states);
            }
        }
    }

    Ok(guards)
}

fn main() -> Result<(), io::Error> {
    let arg = aoc::get_cmdline_arg()?;

    let guards = parse(&arg)?;
    let mut map = HashMap::new();

    for (id, states) in &guards {
        let mut minutes = map.entry(id).or_insert_with(|| [0usize; 60]);

        let mut iter = states.iter();
        while let Some(state) = iter.next() {
            if let GuardState::FallAsleep(ref t) = state {
                let begin = t.time.minute as usize;

                if let Some(state) = iter.next() {
                    if let GuardState::WakeUp(ref t) = state {
                        let end = t.time.minute as usize;
                        for i in begin..end {
                            minutes[i] += 1;
                        }
                    }
                }
            }
        }
    }

    let mut max: Option<(usize, usize)> = None;

    for (k, v) in &map {
        match max {
            Some((_, m)) => {
                let sum = v.iter().sum();
                if sum > m {
                    max = Some((**k, sum));
                }
            }
            None => {
                max = Some((**k, v.iter().sum()));
            }
        }
    }

    let (id, _) = max.unwrap();

    let (minute, _) = map[&id]
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    println!("part 1:");
    println!("id: {}", id);
    println!("minute: {}", minute);
    println!("ans: {}", id * minute);

    let mut frequents = HashMap::new();
    for (k, v) in &map {
        let max = v.iter().max().unwrap();
        frequents.insert(k, max);
    }

    let (_, (frequent, _)) = frequents
        .iter()
        .enumerate()
        .max_by(|(_, (_, v)), (_, (_, v2))| v.cmp(v2))
        .unwrap();

    let (minute, _) = map[*frequent]
        .iter()
        .enumerate()
        .find(|(_, a)| *a == frequents[frequent])
        .unwrap();

    println!();
    println!("part 2:");
    println!("id: {}", frequent);
    println!("ans: {}", **frequent * minute);

    Ok(())
}
