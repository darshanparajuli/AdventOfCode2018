use std::cmp;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Debug)]
struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

fn to_claim(line: String) -> Result<Claim, io::Error> {
    let index_at = line
        .find('@')
        .ok_or(io::Error::from(io::ErrorKind::InvalidData))?;

    let id = line[1..index_at - 1]
        .parse()
        .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;

    let index_comma = line
        .find(',')
        .ok_or(io::Error::from(io::ErrorKind::InvalidData))?;

    let left = line[index_at + 2..index_comma]
        .parse()
        .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;

    let index_colon = line
        .find(':')
        .ok_or(io::Error::from(io::ErrorKind::InvalidData))?;

    let top = line[index_comma + 1..index_colon]
        .parse()
        .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;

    let index_x = line
        .find('x')
        .ok_or(io::Error::from(io::ErrorKind::InvalidData))?;

    let width = line[index_colon + 2..index_x]
        .parse()
        .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;

    let height = line[index_x + 1..]
        .parse()
        .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;

    Ok(Claim {
        id,
        left,
        top,
        width,
        height,
    })
}

fn parse_input(file_name: &str) -> Result<Vec<Claim>, io::Error> {
    let mut file = File::open(file_name)?;
    let reader = BufReader::new(&mut file);
    Ok(reader
        .lines()
        .map(|line| line.unwrap())
        .map(to_claim)
        .map(|claim| claim.unwrap())
        .collect::<Vec<_>>())
}

fn main() -> Result<(), io::Error> {
    let arg = env::args()
        .skip(1)
        .next()
        .ok_or(io::Error::from(io::ErrorKind::InvalidInput))?;

    let claims = parse_input(&arg)?;

    let mut min_left = std::u32::MAX;
    let mut min_top = std::u32::MAX;
    let mut max_right = 0;
    let mut max_bottom = 0;

    for c in &claims {
        min_left = cmp::min(c.left, min_left);
        min_top = cmp::min(c.top, min_top);
        max_right = cmp::max(c.left + c.width, max_right);
        max_bottom = cmp::max(c.top + c.height, max_bottom);
    }

    let mut fabric =
        vec![vec![0u32; (max_bottom - min_top) as usize]; (max_right - min_left) as usize];

    for c in &claims {
        for i in c.left..(c.left + c.width) {
            for j in c.top..(c.top + c.height) {
                fabric[(i - min_left) as usize][(j - min_top) as usize] += 1;
            }
        }
    }

    let mut inches = 0;
    for i in &fabric {
        for j in i {
            if *j > 1 {
                inches += 1;
            }
        }
    }

    println!("inches: {}", inches);

    for c in claims {
        let mut count = 0;
        for i in c.left..(c.left + c.width) {
            for j in c.top..(c.top + c.height) {
                count += fabric[(i - min_left) as usize][(j - min_top) as usize];
            }
        }

        if count == (c.width * c.height) {
            println!("id: {}", c.id);
            break;
        }
    }

    Ok(())
}
