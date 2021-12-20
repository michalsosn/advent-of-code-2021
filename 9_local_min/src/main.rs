use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::io;
use std::io::BufRead;


fn main() {
    let Input { board } = Input::read_stdin();
    // println!("{:?}", board);

    let risk_level: u64 = find_minima(board.as_slice());
    println!("risk_level {}", risk_level);

    let top_basins_size: u64 = find_basins(board.as_slice());
    println!("top_basins_size {}", top_basins_size);
}


#[derive(Debug)]
struct Input {
    board: Vec<Vec<u8>>,
}

impl Input {
    fn read_stdin() -> Self {
        let board = io::stdin().lock().lines()
            .map(|l| l.expect("error: unable to read line").trim()
                 .chars()
                 .map(|c| c.to_digit(10).unwrap() as u8)
                 .collect())
            .collect();
        Input {
            board
        }
    }
}

fn find_minima(board: &[Vec<u8>]) -> u64 {
    let mut risk_level: u64 = 0;

    let max_y = board.len() - 1;
    let max_x = board[0].len() - 1;
    for (y, row) in board.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            if (y == 0 || board[y - 1][x] > v) &&
                (y == max_y || board[y + 1][x] > v) &&
                (x == 0 || row[x - 1] > v) &&
                (x == max_x || row[x + 1] > v) {
                risk_level += v as u64 + 1;
            }
        }
    }
    risk_level
}

#[derive(Debug)]
struct Point {
    y: usize,
    x: usize,
}

fn find_basins(board: &[Vec<u8>]) -> u64 {
    let height = board.len();
    let width = board[0].len();
    let max_y = height - 1;
    let max_x = width - 1;

    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut id_assignments: Vec<Vec<u32>> = vec![vec![0; width]; height];
    let mut next_id: u32 = 1;

    for (y, row) in board.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            if (y == 0 || board[y - 1][x] > v) &&
                (y == max_y || board[y + 1][x] > v) &&
                (x == 0 || row[x - 1] > v) &&
                (x == max_x || row[x + 1] > v) {
                    let node = Point { y, x };
                    queue.push_back(node);
                    id_assignments[y][x] = next_id;
                    next_id += 1;
            }
        }
    }

    while !queue.is_empty() {
        let Point { y, x } = queue.pop_front().unwrap();
        let v = board[y][x];
        let id = id_assignments[y][x];
        // println!("Point y={} x={} v={} id={}", y, x, v, id);
        if y > 0 && id_assignments[y - 1][x] == 0 && board[y - 1][x] > v && board[y - 1][x] < 9 {
            id_assignments[y - 1][x] = id;
            queue.push_back(Point { y: y - 1, x });
        }
        if y < max_y && id_assignments[y + 1][x] == 0 && board[y + 1][x] > v && board[y + 1][x] < 9 {
            id_assignments[y + 1][x] = id;
            queue.push_back(Point { y: y + 1, x });
        }
        if x > 0 && id_assignments[y][x - 1] == 0 && board[y][x - 1] > v && board[y][x - 1] < 9 {
            id_assignments[y][x - 1] = id;
            queue.push_back(Point { y, x: x - 1 });
        }
        if x < max_x && id_assignments[y][x + 1] == 0 && board[y][x + 1] > v && board[y][x + 1] < 9 {
            id_assignments[y][x + 1] = id;
            queue.push_back(Point { y, x: x + 1 });
        }
    }

    let mut basin_sizes: Vec<u64> = vec![0; next_id as usize];
    for row in id_assignments {
        for id in row {
            basin_sizes[id as usize] += 1;
        }
    }
    println!("basin sizes: {:?}", basin_sizes);

    let mut heap: BinaryHeap<u64> = basin_sizes.iter().skip(1).copied().collect();
    heap.pop().unwrap() * heap.pop().unwrap() * heap.pop().unwrap()
}
