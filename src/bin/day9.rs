use aoc;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone)]
struct GameInfo {
    players: usize,
    last_marble_points: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let arg = aoc::get_cmdline_arg()?;

    let game_info = BufReader::new(File::open(arg)?)
        .lines()
        .next()
        .unwrap()
        .map(|line| {
            let players = line[..line.find(" ").unwrap()].parse::<usize>().unwrap();
            let points = line[line.find("worth").unwrap() + 6..line.find("points").unwrap() - 1]
                .parse::<usize>()
                .unwrap();

            GameInfo {
                players,
                last_marble_points: points,
            }
        })?;

    println!("part 1: {}", part1(&game_info));
    println!("part 2: {}", part2(&game_info));

    Ok(())
}

fn part1(game_info: &GameInfo) -> usize {
    let mut players = HashMap::new();
    for i in 1..=game_info.players {
        players.insert(i, 0);
    }

    let mut circle = vec![0];
    let mut current_marble_index = 0;
    let mut current_player = 1;

    for marble in 1..=game_info.last_marble_points {
        if circle.len() == 1 {
            current_marble_index = 1;
            circle.push(marble);
        } else {
            if marble > 1 && marble % 23 == 0 {
                *players.get_mut(&current_player).unwrap() += marble;
                let mut remove_index = (current_marble_index as isize - 7) % circle.len() as isize;
                if remove_index < 0 {
                    remove_index = circle.len() as isize + remove_index;
                }
                let remove_index = remove_index as usize;
                *players.get_mut(&current_player).unwrap() += circle.remove(remove_index);
                current_marble_index = remove_index % circle.len();
            } else {
                let one = (current_marble_index + 1) % circle.len();
                let two = (current_marble_index + 2) % circle.len();
                if one == circle.len() - 1 && two == 0 {
                    circle.push(marble);
                    current_marble_index = one + 1;
                } else {
                    circle.insert(two, marble);
                    current_marble_index = two;
                }
            }
        }

        // print!("[{}]: ", current_player);
        // circle.iter().enumerate().for_each(|(i, v)| {
        //     if i == current_marble_index {
        //         print!("({}) ", v);
        //     } else {
        //         print!("{} ", v);
        //     }
        // });
        // println!();

        current_player += 1;
        if current_player > players.len() {
            current_player = 1;
        }
    }

    let (_, highest_score) = players
        .iter()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap();

    *highest_score
}

fn part2(game_info: &GameInfo) -> usize {
    let mut game_info_large = game_info.clone();
    game_info_large.last_marble_points = game_info.last_marble_points * 100;
    part1(&game_info_large)
}
