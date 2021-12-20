use std::cmp;
use std::io;
use std::io::BufRead;


fn main() {
    let Input { positions } = Input::read_stdin();
    // println!("{:?}", positions);

    let cost = find_abs_cost(positions.as_slice());
    println!("{}", cost);

    let cost = find_inc_cost(positions.as_slice());
    println!("{}", cost);
}

#[derive(Debug)]
struct Input {
    positions: Vec<u32>,
}

impl Input {
    fn read_stdin() -> Self {
        let positions = io::stdin().lock().lines()
            .next().unwrap()
            .expect("error: unable to read line")
            .trim()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        Input {
            positions
        }
    }
}

fn find_abs_cost(positions: &[u32]) -> u64 {
    let len = positions.len();
    let mut sorted: Vec<u32> = positions.to_vec();
    sorted.sort();

    if len % 2 == 1 || (sorted[len / 2 - 1] == sorted[len / 2]) {
        calculate_abs_cost(positions, sorted[len / 2])
    } else {
        let mut min_cost = calculate_abs_cost(positions, sorted[len / 2]);
        for p in sorted[len / 2 - 1]..sorted[len / 2] {
            let cost = calculate_abs_cost(positions, p);
            if cost < min_cost {
                min_cost = cost;
            }
        }
        min_cost
    }
}

fn calculate_abs_cost(positions: &[u32], target: u32) -> u64 {
    positions.iter().map(|&n| (n as i64 - target as i64).abs() as u64).sum()
}

fn find_inc_cost(positions: &[u32]) -> u64 {
    let len = positions.len();
    let sum: u64 = positions.iter().map(|&n| n as u64).sum();
    let mean: f64 = sum as f64 / len as f64;

    let candidates = [(mean - 1.0) as u32, mean as u32, (mean + 1.0) as u32];

    candidates.iter().map(|&n| calculate_inc_cost(positions, n)).min().unwrap()
}

fn calculate_inc_cost(positions: &[u32], target: u32) -> u64 {
    positions.iter()
        .map(|&n| (n as i64 - target as i64).abs() as u64)
        .map(|n| n * (n + 1) / 2)
        .sum()
}

