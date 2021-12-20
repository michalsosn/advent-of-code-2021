use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;
use std::io::BufRead;


fn main() {
    let Input { costs } = Input::read_stdin();
    // println!("{:?}", costs);

    let lowest_risk: u64 = find_path(costs.as_slice());
    println!("lowest_risk = {}", lowest_risk);

    let large_costs = enlarge_costs(costs.as_slice(), 5);
    let lowest_risk: u64 = find_path(large_costs.as_slice());
    println!("lowest_risk = {}", lowest_risk);
}


#[derive(Debug)]
struct Input {
    costs: Vec<Vec<u8>>,
}

impl Input {
    fn read_stdin() -> Self {
        let costs = io::stdin().lock().lines()
            .map(|l| l.expect("error: unable to read line").trim()
                 .chars()
                 .map(|c| c.to_digit(10).unwrap() as u8)
                 .collect())
            .collect();
        Input {
            costs
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Point {
    total_cost: u64,
    y: usize,
    x: usize,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.total_cost, self.y, self.x).cmp(&(other.total_cost, other.y, other.x)).reverse()
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_path(costs: &[Vec<u8>]) -> u64 {
    let height = costs.len();
    let width = costs[0].len();
    let max_y = height - 1;
    let max_x = width - 1;

    let mut total_costs: Vec<Vec<u64>> = vec![vec![u64::MAX; width]; height];
    let mut queue: BinaryHeap<Point> = BinaryHeap::new();

    total_costs[0][0] = 0; 
    queue.push(Point { y: 0, x: 0, total_cost: 0 });

    while !queue.is_empty() {
        let Point { total_cost, y, x } = queue.pop().unwrap();

        if total_cost > total_costs[y][x] {
            continue;
        }

        if y > 0 && total_costs[y - 1][x] > total_cost + costs[y - 1][x] as u64 {
            total_costs[y - 1][x] = total_cost + costs[y - 1][x] as u64;
            queue.push(Point { y: y - 1, x: x, total_cost: total_costs[y - 1][x] });
        }
        if x > 0 && total_costs[y][x - 1] > total_cost + costs[y][x - 1] as u64 {
            total_costs[y][x - 1] = total_cost + costs[y][x - 1] as u64;
            queue.push(Point { y: y, x: x - 1, total_cost: total_costs[y][x - 1] });
        }
        if y < max_y && total_costs[y + 1][x] > total_cost + costs[y + 1][x] as u64 {
            total_costs[y + 1][x] = total_cost + costs[y + 1][x] as u64;
            queue.push(Point { y: y + 1, x: x, total_cost: total_costs[y + 1][x] });
        }
        if x < max_x && total_costs[y][x + 1] > total_cost + costs[y][x + 1] as u64 {
            total_costs[y][x + 1] = total_cost + costs[y][x + 1] as u64;
            queue.push(Point { y: y, x: x + 1, total_cost: total_costs[y][x + 1] });
        }
    }

    total_costs[max_y][max_x]
}

fn enlarge_costs(costs: &[Vec<u8>], multiplier: usize) -> Vec<Vec<u8>> {
    let height = costs.len();
    let width = costs[0].len();

    let mut large_costs: Vec<Vec<u8>> = vec![vec![0; width * multiplier]; height * multiplier];
    for ym in 0..multiplier {
        for xm in 0..multiplier {
            for (y, row) in costs.iter().enumerate() {
                for (x, v) in row.into_iter().enumerate() {
                    let ty = height * ym + y;
                    let tx = width * xm + x;
                    let inc = ym + xm;
                    let mut new_v = *v as usize + inc;
                    while new_v >= 10 {
                        new_v -= 9;
                    }
                    large_costs[ty][tx] = new_v as u8;
                }
            }
        }
    }

    large_costs
}
