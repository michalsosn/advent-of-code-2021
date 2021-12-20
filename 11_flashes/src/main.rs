use std::collections::VecDeque;
use std::io;
use std::io::BufRead;


fn main() {
    let Input { board } = Input::read_stdin();
    // println!("{:?}", board);

    let flash_count: u64 = count_flashes(board.as_slice(), 100);
    println!("flash_count {}", flash_count);

    let synchronized_step: u32 = find_synchronized_step(board.as_slice());
    println!("synchronized_step {}", synchronized_step);
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


#[derive(Debug)]
struct Point {
    y: usize,
    x: usize,
}

fn count_flashes(board: &[Vec<u8>], steps: u32) -> u64 {
    let size = 10;
    let mut board: [[u8; 10]; 10] = board.iter()
        .map(|row| row[0..10].try_into().unwrap())
        .collect::<Vec<[u8; 10]>>().try_into().unwrap();

    let mut flash_count: u64 = 0;
    let mut queue: VecDeque<Point> = VecDeque::new();

    for step in 0..steps {
        if step % 10 == 0 {
            println!("#{} ({})\n{}", step, flash_count, show_board(&board));
        }
        for y in 0..size {
            for x in 0..size {
                let v = board[y][x];
                if v < 10 {
                    board[y][x] += 1;
                }
                if v == 9 {
                    queue.push_back(Point { y, x });
                }
            }
        }

        let neighbors: [(i8, i8); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        while !queue.is_empty() {
            let Point { y, x } = queue.pop_front().unwrap();

            for (ny, nx) in neighbors {
                let ty = y as i8 + ny;
                let tx = x as i8 + nx;
                if (ty >= 0) && ((ty as usize) < size) && (tx >= 0) && ((tx as usize) < size) {
                    let ty = ty as usize;
                    let tx = tx as usize;
                    let v = board[ty][tx];
                    if v < 10 {
                        board[ty][tx] += 1;
                    }
                    if v == 9 {
                        queue.push_back(Point { y: ty, x: tx });
                    }
                }
            }
        }

        for y in 0..size {
            for x in 0..size {
                let v = board[y][x];
                if v == 10 {
                    board[y][x] = 0;
                    flash_count += 1;
                }
            }
        }
    }
    println!("#{} ({})\n{}", steps, flash_count, show_board(&board));

    flash_count
}

fn show_board(board: &[[u8; 10]; 10]) -> String {
    board.map(|r| r.map(|c| c.to_string()).join("")).join("\n")
}

fn find_synchronized_step(board: &[Vec<u8>]) -> u32 {
    let size = 10;
    let mut board: [[u8; 10]; 10] = board.iter()
        .map(|row| row[0..10].try_into().unwrap())
        .collect::<Vec<[u8; 10]>>().try_into().unwrap();

    let mut queue: VecDeque<Point> = VecDeque::new();

    let mut step = 0;
    let synchronized_step = loop {
        if step % 10 == 0 {
            println!("#{}\n{}", step, show_board(&board));
        }
        for y in 0..size {
            for x in 0..size {
                let v = board[y][x];
                if v < 10 {
                    board[y][x] += 1;
                }
                if v == 9 {
                    queue.push_back(Point { y, x });
                }
            }
        }

        let neighbors: [(i8, i8); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        while !queue.is_empty() {
            let Point { y, x } = queue.pop_front().unwrap();

            for (ny, nx) in neighbors {
                let ty = y as i8 + ny;
                let tx = x as i8 + nx;
                if (ty >= 0) && ((ty as usize) < size) && (tx >= 0) && ((tx as usize) < size) {
                    let ty = ty as usize;
                    let tx = tx as usize;
                    let v = board[ty][tx];
                    if v < 10 {
                        board[ty][tx] += 1;
                    }
                    if v == 9 {
                        queue.push_back(Point { y: ty, x: tx });
                    }
                }
            }
        }

        let mut flash_count: u32 = 0;
        for y in 0..size {
            for x in 0..size {
                let v = board[y][x];
                if v == 10 {
                    board[y][x] = 0;
                    flash_count += 1;
                }
            }
        }
        if flash_count == 100 {
            break step + 1;
        }
        step += 1;
    };
    println!("#{}\n{}", step, show_board(&board));

    synchronized_step
}
