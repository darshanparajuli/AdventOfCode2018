extern crate aoc;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> Result<(), io::Error> {
    let arg = aoc::get_cmdline_arg()?;

    let chars = BufReader::new(File::open(arg)?)
        .lines()
        .next()
        .unwrap()?
        .chars()
        .collect::<Vec<_>>();

    let chars2 = chars.clone();
    println!("part 1: {}", react(chars).len());

    let mut counts = Vec::new();
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        let mut chars = chars2.clone();
        chars.retain(|a| *a != c && (*a as i8 - c as i8).abs() != 32);
        counts.push(react(chars).len());
    }

    println!("part 2: {}", counts.iter().min().unwrap());

    Ok(())
}

fn react(mut chars: Vec<char>) -> Vec<char> {
    let mut deletes = vec![];
    loop {
        deletes.clear();

        if chars.len() <= 1 {
            break;
        }

        let mut index = 0;
        while index < chars.len() - 1 {
            let c0 = chars[index];
            let c1 = chars[index + 1];

            if c0 != c1 && (c0 as i8 - c1 as i8).abs() == 32 {
                deletes.push(index);
                index += 1;
            }

            index += 1;
        }

        if deletes.is_empty() {
            break;
        } else {
            for i in deletes.iter().rev() {
                chars.remove(*i);
                chars.remove(*i);
            }
        }
    }
    chars
}
