use aoc;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let arg = aoc::get_cmdline_arg()?;

    let numbers = BufReader::new(File::open(arg)?)
        .lines()
        .next()
        .unwrap()?
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    part1(&numbers);

    Ok(())
}

fn part1(numbers: &[usize]) {
    let mut sum = 0;

    sum_metadatas(numbers, 0, &mut sum);

    println!("part 1: {}", sum);
}

fn sum_metadatas(numbers: &[usize], index: usize, sum: &mut usize) -> usize {
    let child_nodes = numbers[index];
    let metadata_count = numbers[index + 1];

    let next_index = if child_nodes == 0 {
        index + 2 + metadata_count
    } else {
        let mut next_index = sum_metadatas(numbers, index + 2, sum);
        for _ in 1..child_nodes {
            next_index = sum_metadatas(numbers, next_index, sum);
        }

        next_index + metadata_count
    };

    for i in 0..metadata_count {
        *sum += numbers[next_index - i - 1];
    }
    next_index
}
