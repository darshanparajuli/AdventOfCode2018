extern crate aoc;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> Result<(), io::Error> {
    let arg = aoc::get_cmdline_arg()?;

    let reader = BufReader::new(File::open(arg)?);

    let mut twos = 0;
    let mut threes = 0;

    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();

    let mut map = HashMap::new();
    for line in &lines {
        for c in line.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        if let Some(_) = map.iter().find(|(_, v)| **v == 2) {
            twos += 1;
        }
        if let Some(_) = map.iter().find(|(_, v)| **v == 3) {
            threes += 1;
        }

        map.clear();
    }

    println!("part 1: {}", twos * threes);

    'outer: for line1 in &lines {
        for line2 in &lines {
            if line1 == line2 || line1.len() != line2.len() {
                continue;
            }

            let chars1 = line1.chars().collect::<Vec<_>>();
            let chars2 = line2.chars().collect::<Vec<_>>();

            let mut count = 0;
            let mut not_common = None;
            for i in 0..line1.len() {
                if chars1[i] != chars2[i] {
                    count += 1;
                    not_common = Some(chars1[i]);
                    if count > 1 {
                        break;
                    }
                }
            }

            if count == 1 {
                println!("{}", line1.replace(not_common.unwrap(), ""));
                break 'outer;
            }
        }
    }

    Ok(())
}
