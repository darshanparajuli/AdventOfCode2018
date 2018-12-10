extern crate aoc;

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::str::FromStr;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

impl FromStr for Point {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let index = s
            .find(',')
            .ok_or(io::Error::from(io::ErrorKind::InvalidData))?;

        let x = s[0..index]
            .parse::<u16>()
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;;
        let y = s[index + 2..]
            .parse::<u16>()
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;;
        Ok(Point { x, y })
    }
}

fn main() -> Result<(), io::Error> {
    let arg = aoc::get_cmdline_arg()?;

    let points = BufReader::new(File::open(arg)?)
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<Point>().unwrap())
        .collect::<Vec<_>>();

    let (max_x, max_y) = max_size(&points);

    let mut id_map = HashMap::new();

    let mut board = vec![vec![0i32; max_x]; max_y];
    for (i, p) in points.iter().enumerate() {
        let id = (i + 1) as i32;
        board[p.y as usize][p.x as usize] = id;
        id_map.insert(p, id);
    }

    let mut dist_map = HashMap::new();

    for y in 0..max_y {
        for x in 0..max_x {
            let p = Point {
                x: x as u16,
                y: y as u16,
            };

            if id_map.contains_key(&p) {
                continue;
            }

            dist_map.clear();
            for (point, id) in &id_map {
                let d = m_dist(&p, point);
                dist_map.insert(id, d);
            }

            let (id, d) = dist_map
                .iter()
                .min_by(|(_, d0), (_, d1)| d0.cmp(d1))
                .unwrap();

            match dist_map
                .iter()
                .filter(|(i, _)| *i != id)
                .find(|(_, dd)| d == *dd)
            {
                Some(_) => {}
                None => board[y][x] = **id,
            }
        }
    }

    // for i in 0..board.len() {
    //     for j in 0..board[i].len() {
    //         let v = board[i][j];
    //         if v == 0 {
    //             print!("{:2}", " .");
    //         } else {
    //             print!("{:2}", v);
    //         }
    //     }
    //     println!();
    // }

    let mut infinites = HashSet::new();
    for i in board[0].iter() {
        if *i != 0 {
            infinites.insert(*i);
        }
    }

    for i in board[board.len() - 1].iter() {
        if *i != 0 {
            infinites.insert(*i);
        }
    }

    for i in 0..board.len() {
        let v = *board[i].first().unwrap();
        if v != 0 {
            infinites.insert(v);
        }

        let v = *board[i].last().unwrap();
        if v != 0 {
            infinites.insert(v);
        }
    }

    let mut areas = vec![];
    for (_, v) in id_map.iter().filter(|(_, id)| !infinites.contains(id)) {
        let mut count = 0;
        for row in &board {
            count += row.iter().filter(|a| *a == v).count();
        }
        areas.push(count);
    }

    println!("part 1: {}", areas.iter().max().unwrap());

    Ok(())
}

fn max_size<'a>(points: &'a Vec<Point>) -> (usize, usize) {
    let mut max_x = 0usize;
    let mut max_y = 0usize;

    for Point { x, y } in points {
        max_x = cmp::max(*x as usize, max_x);
        max_y = cmp::max(*y as usize, max_y);
    }

    (max_x + 1, max_y + 1)
}

fn m_dist<'a>(a: &'a Point, b: &'a Point) -> u16 {
    (a.x as i16 - b.x as i16).abs() as u16 + (a.y as i16 - b.y as i16).abs() as u16
}
