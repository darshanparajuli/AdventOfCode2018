use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = 633601;
    part1(input);
    part2(input);
    Ok(())
}

fn part1(input: usize) {
    let mut recipes = vec![3, 7];
    let mut digits = vec![];
    digits.reserve(2);

    let mut elf1 = 0;
    let mut elf2 = 1;

    loop {
        let r1 = recipes[elf1];
        let r2 = recipes[elf2];

        digits.clear();
        let mut sum = r1 + r2;
        if sum == 0 {
            recipes.push(0);
        } else {
            while sum != 0 {
                let digit = sum % 10;
                digits.push(digit);
                sum /= 10;
            }
            for d in digits.iter().rev() {
                recipes.push(*d);
            }
        }

        if recipes.len() >= input + 10 {
            break;
        }

        elf1 = (elf1 + r1 + 1) % recipes.len();
        elf2 = (elf2 + r2 + 1) % recipes.len();

        // for (i, v) in recipes.iter().enumerate() {
        //     if i == elf1 {
        //         print!("({})", v);
        //     } else if i == elf2 {
        //         print!("[{}]", v);
        //     } else {
        //         print!(" {} ", v);
        //     }
        // }
        // println!();
    }

    print!("part 1: ");
    for r in recipes.iter().skip(input).take(10) {
        print!("{}", r);
    }
    println!();
}

fn part2(input: usize) {
    let mut recipes = vec![3, 7];
    let mut digits = vec![];
    digits.reserve(2);

    let mut input_seq = vec![];
    let mut tmp = input;
    while tmp != 0 {
        let digit = tmp % 10;
        digits.push(digit);
        tmp /= 10;
    }
    for d in digits.iter().rev() {
        input_seq.push(*d);
    }
    digits.clear();

    let mut elf1 = 0;
    let mut elf2 = 1;

    'l: loop {
        let r1 = recipes[elf1];
        let r2 = recipes[elf2];

        digits.clear();
        let mut sum = r1 + r2;
        if sum == 0 {
            recipes.push(0);
        } else {
            while sum != 0 {
                let digit = sum % 10;
                digits.push(digit);
                sum /= 10;
            }
            for d in digits.iter().rev() {
                recipes.push(*d);
            }
        }

        for (i, w) in recipes.windows(input_seq.len()).rev().take(2).enumerate() {
            if w == &input_seq[..] {
                println!("part 2: {}", recipes.len() - input_seq.len() - i);
                break 'l;
            }
        }

        elf1 = (elf1 + r1 + 1) % recipes.len();
        elf2 = (elf2 + r2 + 1) % recipes.len();

        // for (i, v) in recipes.iter().enumerate() {
        //     if i == elf1 {
        //         print!("({})", v);
        //     } else if i == elf2 {
        //         print!("[{}]", v);
        //     } else {
        //         print!(" {} ", v);
        //     }
        // }
        // println!();
    }
}
