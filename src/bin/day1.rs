extern crate aoc;

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> Result<(), io::Error> {
    let arg = aoc::get_cmdline_arg()?;

    let reader = BufReader::new(File::open(arg)?);

    let mut set = HashSet::new();

    let changes = reader
        .lines()
        .map(|line| line.unwrap().parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let mut ans = changes.iter().fold(0, |acc, v| {
        let f = acc + v;
        set.insert(f);
        f
    });
    println!("part 1: {}", ans);

    let mut value = 0;
    let mut found = false;
    while !found {
        ans = changes.iter().fold(ans, |acc, v| {
            let f = acc + v;

            if !found {
                if set.contains(&f) {
                    found = true;
                    value = f;
                }
                set.insert(f);
            }
            f
        });
    }

    println!("part 2: {}", value);

    Ok(())
}
