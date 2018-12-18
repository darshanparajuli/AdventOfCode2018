use aoc;
use std::cmp;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::ops::{AddAssign, SubAssign};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point {
    pos: Vector,
    vel: Vector,
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s[s.find("<").unwrap() + 1..s.find(",").unwrap()]
            .trim()
            .parse::<i32>()?;
        let y = s[s.find(",").unwrap() + 1..s.find(">").unwrap()]
            .trim()
            .parse::<i32>()?;

        let velocity_index = s.find("velocity").unwrap() + 9;
        let s = &s[velocity_index..];
        let vx = s[s.find("<").unwrap() + 1..s.find(",").unwrap()]
            .trim()
            .parse::<i32>()?;
        let vy = s[s.find(",").unwrap() + 1..s.find(">").unwrap()]
            .trim()
            .parse::<i32>()?;

        Ok(Point {
            pos: Vector { x: x, y: y },
            vel: Vector { x: vx, y: vy },
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let arg = aoc::get_cmdline_arg()?;

    let mut input = BufReader::new(File::open(arg)?)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<Point>().unwrap())
        .collect::<Vec<_>>();

    let mut max_dy_count = 0;
    let mut max_dx_count = 0;
    let mut seconds = 0;

    loop {
        for p in input.iter_mut() {
            p.pos += p.vel.clone();
        }

        let mut dy_count = 0;
        let mut dx_count = 0;
        for p1 in input.iter() {
            for p2 in input.iter() {
                let x1 = p1.pos.x;
                let y1 = p1.pos.y;
                let x2 = p2.pos.x;
                let y2 = p2.pos.y;
                if x1 == x2 && y1 == y2 {
                    continue;
                }

                let dy = (y2 - y1).abs();
                let dx = (x2 - x1).abs();
                if x1 == x2 && dy <= 11 {
                    dy_count += 1;
                }
                if y1 == y2 && dx <= 7 {
                    dx_count += 1;
                }
            }
        }

        if dy_count < max_dy_count && dx_count < max_dx_count {
            break;
        } else {
            max_dy_count = dy_count;
            max_dx_count = dx_count;
            seconds += 1;
        }
    }

    for p in input.iter_mut() {
        p.pos -= p.vel.clone();
    }

    let mut max_x = 0i32;
    let mut max_y = 0i32;
    for i in input.iter() {
        max_x = cmp::max(max_x, i.pos.x);
        max_y = cmp::max(max_y, i.pos.y);
    }

    let mut min_x = max_x;
    let mut min_y = max_y;
    for i in input.iter() {
        min_x = cmp::min(min_x, i.pos.x);
        min_y = cmp::min(min_y, i.pos.y);
    }

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut board = vec![vec![0u8; width as usize]; height as usize];

    for Point { pos, .. } in input.iter() {
        let x = pos.x - min_x;
        let y = pos.y - min_y;
        if x >= 0 && x < width && y >= 0 && y < height {
            let x = x as usize;
            let y = y as usize;
            board[y][x] = 1u8;
        }
    }

    println!("part 1:\n");
    print_board(&board);
    println!();

    println!("part 2: {} seconds", seconds);

    Ok(())
}

fn print_board(board: &[Vec<u8>]) {
    for y in 0..board.len() {
        let row = &board[y];
        for x in 0..row.len() {
            if row[x] == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}
