use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = 7689;

    let mut fuel_cells = [[0i32; 300]; 300];
    for y in 0..fuel_cells.len() {
        let row = fuel_cells[y];
        let y_coord = y + 1;
        for x in 0..row.len() {
            let x_coord = x + 1;
            let rack_id = x_coord + 10;
            let mut power_level = (rack_id * y_coord) as i32;
            power_level += input as i32;
            power_level *= rack_id as i32;
            power_level = (power_level / 100) % 10;
            power_level -= 5;
            fuel_cells[y][x] = power_level;
        }
    }

    let MaxPower { x, y, .. } = get_max_power(&fuel_cells, 3);
    println!("part 1: {},{}", x, y);

    let mut square = 1;
    let mut max_power = get_max_power(&fuel_cells, square);
    for i in 2..300 {
        let mp = get_max_power(&fuel_cells, i);
        if mp.power > max_power.power {
            max_power = mp;
            square = i;
        }
    }

    println!("part 2: {},{},{}", max_power.x, max_power.y, square);

    Ok(())
}

struct MaxPower {
    x: usize,
    y: usize,
    power: i32,
}

fn get_max_power(fuel_cells: &[[i32; 300]], square_size: usize) -> MaxPower {
    assert!(square_size >= 1);
    assert!(square_size < 300);

    let mut power = 0;
    let mut top_left = (0, 0);

    for y in 0..fuel_cells.len() - square_size - 1 {
        let row = fuel_cells[y];
        for x in 0..row.len() - square_size - 1 {
            let mut sum = 0;
            for y in y..y + square_size {
                for x in x..x + square_size {
                    sum += fuel_cells[y][x];
                }
            }

            if sum > power {
                power = sum;
                top_left = (x + 1, y + 1);
            }
        }
    }

    let (x, y) = top_left;
    MaxPower { x, y, power }
}
