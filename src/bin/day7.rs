extern crate aoc;

use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Step(char);

impl Step {
    fn work_time(&self) -> usize {
        60 + self.0 as usize - 65 + 1
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Step) -> Ordering {
        other.0.cmp(&self.0)
    }
}

fn main() -> Result<(), io::Error> {
    let arg = aoc::get_cmdline_arg()?;

    let steps = BufReader::new(File::open(arg)?)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let must_be_finished = line[5..6].parse::<char>().unwrap();
            let before = line[36..37].parse::<char>().unwrap();
            (Step(must_be_finished), Step(before))
        })
        .collect::<Vec<_>>();

    let mut map: HashMap<&Step, Vec<&Step>> = HashMap::new();
    let mut map_inv: HashMap<&Step, HashSet<&Step>> = HashMap::new();
    for (m, b) in &steps {
        match map.entry(m) {
            Entry::Occupied(ref mut o) => {
                o.get_mut().push(b);
            }
            Entry::Vacant(v) => {
                v.insert(vec![b]);
            }
        }

        match map_inv.entry(b) {
            Entry::Occupied(ref mut o) => {
                o.get_mut().insert(m);
            }
            Entry::Vacant(v) => {
                let mut set = HashSet::new();
                set.insert(m);
                v.insert(set);
            }
        }
    }

    part1(&map, &map_inv);
    part2(&map, &map_inv, 5);

    Ok(())
}

fn part1(map: &HashMap<&Step, Vec<&Step>>, map_inv: &HashMap<&Step, HashSet<&Step>>) {
    let mut available = BinaryHeap::new();
    for (k, _) in map.iter() {
        if !map_inv.contains_key(k) {
            available.push(k);
        }
    }

    let mut output = String::new();
    let mut completed = HashSet::new();
    let mut tmp = HashSet::new();

    while !available.is_empty() {
        let mut a = available.pop().unwrap();

        tmp.clear();
        loop {
            let mut completed_previous = true;
            if let Some(prev_steps) = map_inv.get(a) {
                for i in prev_steps {
                    if !completed.contains(i) {
                        completed_previous = false;
                        break;
                    }
                }
            }

            if completed_previous {
                break;
            } else {
                if available.is_empty() {
                    break;
                } else {
                    tmp.insert(a);
                    a = available.pop().unwrap();
                }
            }
        }

        for i in &tmp {
            available.push(i);
        }

        if !completed.contains(a) {
            output.push(a.0.to_owned());
        }

        completed.insert(a);

        if let Some(steps) = map.get(a) {
            for s in steps {
                available.push(s);
            }
        }
    }

    println!("part 1: {}", output);
}

fn part2(
    map: &HashMap<&Step, Vec<&Step>>,
    map_inv: &HashMap<&Step, HashSet<&Step>>,
    worker_count: usize,
) {
    let mut finished: HashSet<&Step> = HashSet::new();
    let mut workers: HashMap<&Step, usize> = HashMap::new();
    let mut available = BinaryHeap::new();

    for (k, _) in map.iter() {
        if !map_inv.contains_key(k) {
            available.push(k);
            if workers.len() < worker_count {
                workers.insert(k, 0);
            }
        }
    }

    let mut count = 0;
    while !workers.is_empty() {
        workers.retain(|k, v| {
            if *v == k.work_time() {
                finished.insert(k);
                false
            } else {
                *v += 1;
                true
            }
        });

        while workers.len() < worker_count {
            let mut tmp = vec![];
            let mut found = false;
            while let Some(a) = available.pop() {
                let mut finished_previous = true;
                if let Some(prev_steps) = map_inv.get(a) {
                    for i in prev_steps {
                        if !finished.contains(i) {
                            finished_previous = false;
                            break;
                        }
                    }
                }

                if finished_previous {
                    workers.insert(a, 1);
                    found = true;
                    if let Some(steps) = map.get(a) {
                        for s in steps {
                            available.push(s);
                        }
                    }
                    break;
                } else {
                    tmp.push(a);
                }
            }

            for a in tmp {
                available.push(a);
            }

            if !found {
                break;
            }
        }

        if !workers.is_empty() {
            count += 1;
        }
    }

    println!("part 2: {}", count);
}
