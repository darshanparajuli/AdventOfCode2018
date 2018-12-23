use aoc;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let arg = aoc::get_cmdline_arg()?;

    let input = BufReader::new(File::open(arg)?)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let (map, carts) = build_map(&input);
    part1(&map, carts.clone());
    part2(&map, carts);

    Ok(())
}

fn part1(map: &Vec<Vec<Cell>>, mut carts: Vec<Cart>) {
    let mut cart_positions: HashMap<usize, Cart> = HashMap::new();

    'l: loop {
        carts.sort_by(|a, b| a.pos.x.cmp(&b.pos.x));
        carts.sort_by(|a, b| a.pos.y.cmp(&b.pos.y));

        for c in carts.iter_mut() {
            match map[c.pos.y][c.pos.x] {
                Cell::Empty => unreachable!("BUG"),
                Cell::Intersection => {
                    c.turn_intersection();
                }
                Cell::Curve(curve) => match c.dir {
                    Direction::Left => match curve {
                        '\\' => {
                            c.dir = Direction::Up;
                        }
                        '/' => {
                            c.dir = Direction::Down;
                        }
                        _ => unreachable!("BUG"),
                    },
                    Direction::Right => match curve {
                        '\\' => {
                            c.dir = Direction::Down;
                        }
                        '/' => {
                            c.dir = Direction::Up;
                        }
                        _ => unreachable!("BUG"),
                    },
                    Direction::Up => match curve {
                        '\\' => {
                            c.dir = Direction::Left;
                        }
                        '/' => {
                            c.dir = Direction::Right;
                        }
                        _ => unreachable!("BUG"),
                    },
                    Direction::Down => match curve {
                        '\\' => {
                            c.dir = Direction::Right;
                        }
                        '/' => {
                            c.dir = Direction::Left;
                        }
                        _ => unreachable!("BUG"),
                    },
                },
                _ => {}
            }

            c.move_forward();

            if let Some((_, cart)) = cart_positions
                .iter()
                .find(|(k, v)| **k != c.id && v.pos == c.pos)
            {
                println!("part 1: {},{}", cart.pos.x, cart.pos.y);
                break 'l;
            }

            cart_positions.insert(c.id, c.clone());
        }
    }
}

fn part2(map: &Vec<Vec<Cell>>, mut carts: Vec<Cart>) {
    let mut cart_positions: HashMap<usize, Cart> = HashMap::new();
    let mut crashed = HashSet::new();

    'l: loop {
        carts.sort_by(|a, b| a.pos.x.cmp(&b.pos.x));
        carts.sort_by(|a, b| a.pos.y.cmp(&b.pos.y));

        crashed.clear();
        for c in carts.iter_mut() {
            match map[c.pos.y][c.pos.x] {
                Cell::Empty => unreachable!("BUG"),
                Cell::Intersection => {
                    c.turn_intersection();
                }
                Cell::Curve(curve) => match c.dir {
                    Direction::Left => match curve {
                        '\\' => {
                            c.dir = Direction::Up;
                        }
                        '/' => {
                            c.dir = Direction::Down;
                        }
                        _ => unreachable!("BUG"),
                    },
                    Direction::Right => match curve {
                        '\\' => {
                            c.dir = Direction::Down;
                        }
                        '/' => {
                            c.dir = Direction::Up;
                        }
                        _ => unreachable!("BUG"),
                    },
                    Direction::Up => match curve {
                        '\\' => {
                            c.dir = Direction::Left;
                        }
                        '/' => {
                            c.dir = Direction::Right;
                        }
                        _ => unreachable!("BUG"),
                    },
                    Direction::Down => match curve {
                        '\\' => {
                            c.dir = Direction::Right;
                        }
                        '/' => {
                            c.dir = Direction::Left;
                        }
                        _ => unreachable!("BUG"),
                    },
                },
                _ => {}
            }

            c.move_forward();

            if let Some((k, _)) = cart_positions
                .iter()
                .find(|(k, v)| **k != c.id && v.pos == c.pos)
            {
                crashed.insert(*k);
                crashed.insert(c.id);
            }

            cart_positions.insert(c.id, c.clone());
        }

        carts.retain(|c| !crashed.contains(&c.id));
        for id in crashed.iter() {
            cart_positions.remove(id);
        }

        if carts.len() == 1 {
            let c = &carts[0];
            println!("part 2: {},{}", c.pos.x, c.pos.y);
            break;
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Clone)]
enum Cell {
    Empty,
    PathHorizontal,
    PathVertical,
    Curve(char),
    Intersection,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Cart {
    id: usize,
    pos: Point,
    dir: Direction,
    count: u8,
}

impl Cart {
    fn new(id: usize, x: usize, y: usize, dir: Direction) -> Self {
        Self {
            id,
            pos: Point { x, y },
            dir,
            count: 0,
        }
    }

    fn move_forward(&mut self) {
        match self.dir {
            Direction::Up => {
                self.pos.y -= 1;
            }
            Direction::Down => {
                self.pos.y += 1;
            }
            Direction::Left => {
                self.pos.x -= 1;
            }
            Direction::Right => {
                self.pos.x += 1;
            }
        }
    }

    fn turn_intersection(&mut self) {
        match self.count {
            0 => {
                self.dir = self.dir.turn_left();
            }
            2 => {
                self.dir = self.dir.turn_right();
            }
            1 => {
                // NOTE(dparajuli): Go straight
            }
            _ => unreachable!("BUG"),
        }

        self.count = (self.count + 1) % 3;
    }
}

fn build_map(input: &[String]) -> (Vec<Vec<Cell>>, Vec<Cart>) {
    let max_width = input
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len();

    let mut map = vec![vec![Cell::Empty; max_width]; input.len()];
    let mut carts = vec![];

    let mut id = 0;
    for y in 0..input.len() {
        let s = &input[y];
        for x in 0..s.len() {
            map[y][x] = match &s[x..x + 1] {
                "-" => Cell::PathHorizontal,
                "|" => Cell::PathVertical,
                c @ "\\" | c @ "/" => Cell::Curve(c.chars().next().unwrap()),
                "+" => Cell::Intersection,
                "<" => {
                    carts.push(Cart::new(id, x, y, Direction::Left));
                    id += 1;
                    Cell::PathHorizontal
                }
                ">" => {
                    carts.push(Cart::new(id, x, y, Direction::Right));
                    id += 1;
                    Cell::PathHorizontal
                }
                "^" => {
                    carts.push(Cart::new(id, x, y, Direction::Up));
                    id += 1;
                    Cell::PathVertical
                }
                "v" => {
                    carts.push(Cart::new(id, x, y, Direction::Down));
                    id += 1;
                    Cell::PathVertical
                }
                _ => Cell::Empty,
            };
        }
    }

    (map, carts)
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<Cell>>, cart_map: &HashMap<usize, Cart>) {
    for y in 0..map.len() {
        let row = &map[y];
        for x in 0..row.len() {
            let p = Point { x, y };
            if let Some((_, cart)) = cart_map.iter().find(|(_, v)| p == v.pos) {
                match cart.dir {
                    Direction::Up => {
                        print!("^");
                    }
                    Direction::Down => {
                        print!("v");
                    }
                    Direction::Right => {
                        print!(">");
                    }
                    Direction::Left => {
                        print!("<");
                    }
                }
            } else {
                match map[y][x] {
                    Cell::Empty => {
                        print!(" ");
                    }
                    Cell::PathHorizontal => {
                        print!("-");
                    }
                    Cell::PathVertical => {
                        print!("|");
                    }
                    Cell::Intersection => {
                        print!("+");
                    }
                    Cell::Curve(c) => {
                        print!("{}", c);
                    }
                }
            }
        }
        println!();
    }
}
