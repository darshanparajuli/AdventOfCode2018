use aoc;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let arg = aoc::get_cmdline_arg()?;

    let mut plants: HashMap<String, bool> = HashMap::new();
    let mut lines = BufReader::new(File::open(arg)?).lines();
    let mut initial_state = lines.next().unwrap()?[15..].to_owned();

    for _ in 0..4 {
        initial_state.insert(0, '.');
        initial_state.push('.');
    }

    lines.skip(1).map(|line| line.unwrap()).for_each(|line| {
        let pattern = line[0..5].to_owned();
        let alive = &line[9..] == "#";
        plants.insert(pattern, alive);
    });

    let sum = sim(&initial_state, &plants, 20);
    println!("part 1: {}", sum);

    let sum = sim(&initial_state, &plants, 50_000_000_000);
    println!("part 2: {}", sum);

    Ok(())
}

fn sim(input: &str, plants: &HashMap<String, bool>, generations: u64) -> i64 {
    let mut curr_state = input.to_owned();
    let mut next_state = String::new();

    let mut zero_index = 4;

    let mut last_diff = 0;
    let mut last_sum = 0;
    let mut gen = 1;
    let mut sum_total = 0;
    let mut same_diff_count = 0;
    while gen <= generations {
        next_state.push_str(&curr_state[0..2]);
        for k in 2..(curr_state.len() - 2) {
            match plants.get(&curr_state[k - 2..k + 3]) {
                Some(alive) => {
                    if *alive {
                        next_state.push('#');
                    } else {
                        next_state.push('.');
                    }
                }
                None => {
                    next_state.push('.');
                }
            }
        }
        next_state.push_str(&curr_state[curr_state.len() - 2..]);

        let old_len = next_state.len();
        if let Some(first_plant) = next_state.find("#") {
            if first_plant < 4 {
                for _ in 0..(4 - first_plant) {
                    next_state.insert(0, '.');
                }
            }
        }
        let new_len = next_state.len();

        zero_index += new_len - old_len;

        if let Some(last_plant) = next_state.rfind("#") {
            if last_plant > next_state.len() - 5 {
                for _ in 0..(last_plant - (next_state.len() - 5)) {
                    next_state.push('.');
                }
            }
        }

        curr_state.clear();
        curr_state.push_str(&next_state);
        next_state.clear();

        let mut sum = 0i64;
        for (i, c) in curr_state.chars().enumerate() {
            if c == '#' {
                sum += i as i64 - zero_index as i64;
            }
        }

        let diff = sum - last_sum;
        if diff == last_diff {
            same_diff_count += 1;
            if same_diff_count == 10 {
                gen -= 1;
                break;
            }
        }

        sum_total += diff;

        last_sum = sum;
        last_diff = diff;

        gen += 1;
    }

    if gen < generations {
        sum_total += last_diff * (generations - gen) as i64;
    }

    sum_total
}
