use std::io;
use std::io::BufRead;


fn main() {
    let Input { lines } = Input::read_stdin();
    // println!("{:?}", lines);

    let consumption = calculate_power_consumption(lines.as_slice());
    println!("{:?}", consumption);
    println!("{}", consumption.gamma * consumption.epsilon);

    let life_rating = calculate_life_rating(lines.as_slice());
    println!("{:?}", life_rating);
    println!("{}", life_rating.oxygen * life_rating.scrubber);
}

#[derive(Debug)]
struct Input {
    lines: Vec<Vec<bool>>
}

impl Input {
    fn read_stdin() -> Self {
        let lines: Vec<Vec<bool>> = io::stdin().lock().lines()
            .map(|line| line.expect("error: unable to read user input").chars()
                 .map(|c| c == '1')
                 .collect())
            .collect();

        Input {
            lines
        }
    }
}

#[derive(Debug)]
struct PowerConsumption {
    gamma: u32,
    epsilon: u32,
}

fn calculate_power_consumption(lines: &[Vec<bool>]) -> PowerConsumption {
    let line_len = lines[0].len();
    let mut counts: Vec<i32> = vec![0; line_len];

    for line in lines {
        for (i, c) in line.iter().enumerate() {
            if *c {
                counts[i] += 1;
            } else {
                counts[i] -= 1;
            }
        }
    }

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for count in counts.iter() {
        if *count > 0 {
            gamma = (gamma << 1) + 1;
            epsilon = epsilon << 1;
        }
        else if *count < 0 {
            gamma = gamma << 1;
            epsilon = (epsilon << 1) + 1;
        }
        else {
            panic!("A count equal to 0 in {:?}", counts);
        }
    }

    PowerConsumption {
        gamma,
        epsilon,
    }
}

#[derive(Debug)]
struct LifeRating {
    oxygen: u32,
    scrubber: u32,
}

fn select_rating<'a>(lines: &'a [Vec<bool>], d: usize, mask: &mut Vec<bool>, select_common: bool) -> &'a Vec<bool> {
    let mut total_count: i32 = 0;
    let mut count: i32 = 0;

    for (i, line) in lines.iter().enumerate() {
        if mask[i] {
            if line[d] {
                count += 1;
            } else {
                count -= 1;
            }
            total_count += 1;
        }
    }

    let to_remove = (count >= 0) ^ select_common;

    for (i, line) in lines.iter().enumerate() {
        if mask[i] {
            if line[d] == to_remove {
                mask[i] = false;
                total_count -= 1;
            }
        }
    }

    if total_count == 1 {
        for (i, line) in lines.iter().enumerate() {
            if mask[i] {
                return line;
            }
        }
    }
    return select_rating(lines, d + 1, mask, select_common);
}

fn calculate_life_rating(lines: &[Vec<bool>]) -> LifeRating {
    let lines_len = lines.len();

    let mut mask: Vec<bool> = vec![true; lines_len];
    let oxygen_line = select_rating(lines, 0, &mut mask, true);
    let oxygen: u32 = bin_to_int(oxygen_line.as_slice());

    let mut mask: Vec<bool> = vec![true; lines_len];
    let scrubber_line = select_rating(lines, 0, &mut mask, false);
    let scrubber: u32 = bin_to_int(scrubber_line.as_slice());

    LifeRating {
        oxygen,
        scrubber,
    }
}

fn bin_to_int(line: &[bool]) -> u32 {
    let mut value: u32 = 0;
    for b in line {
        value = (value << 1) + *b as u32;
    }
    return value;
}
