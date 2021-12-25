use std::io;
use std::io::BufRead;


fn main() {
    let Input { board } = Input::read_stdin();
    println!("{}", render_board(board.as_slice()));

    let (end_board, step_count) = simulate_until_stop(&board);
    println!("Stopped after {} steps", step_count);
    println!("{}", render_board(end_board.as_slice()));
}

#[derive(Debug)]
struct Input {
    board: Vec<Vec<u8>>,
}

impl Input {
    fn read_stdin() -> Self {
        let board: Vec<Vec<u8>> = io::stdin().lock().lines()
            .map(|line| {
                let line = line.expect("error: unable to read line");
                line.chars().map(|c| symbol_to_number(c)).collect()
            })
            .collect();
        Input {
            board,
        }
    }
}

fn simulate_until_stop(board: &Vec<Vec<u8>>) -> (Vec<Vec<u8>>, u32) {
    let height = board.len();
    let width = board[0].len();
    let mut board_a = board.clone();
    let mut board_b = board.clone();
    let mut step = 0;

    loop {
        let mut changed: bool = false;
        step += 1;

        for y in 0..height {
            for x in 0..width {
                let prev_x = if x == 0 { width - 1 } else { x - 1 };
                let mark = if x == 0 { 4 } else { 0 };
                if board_a[y][x] == 0 && board_a[y][prev_x] == 1 {
                    board_b[y][x] = 1;
                    board_b[y][prev_x] = mark;
                    changed = true;
                } else if board_b[y][x] == 4 {
                    board_b[y][x] = 0;
                } else {
                    board_b[y][x] = board_a[y][x];
                }
            }
        }

        for y in 0..height {
            let prev_y = if y == 0 { height - 1 } else { y - 1 };
            let mark = if y == 0 { 4 } else { 0 };
            for x in 0..width {
                if board_b[y][x] == 0 && board_b[prev_y][x] == 2 {
                    board_a[y][x] = 2;
                    board_a[prev_y][x] = mark;
                    changed = true;
                } else if board_a[y][x] == 4 {
                    board_a[y][x] = 0;
                } else {
                    board_a[y][x] = board_b[y][x];
                }
            }
        }

        if !changed {
            break;
        }
    }

    (board_a, step)
}

fn render_board(board: &[Vec<u8>]) -> String {
    board.iter()
        .map(|r| r.into_iter().map(|&n| number_to_symbol(n)).collect())
        .collect::<Vec<String>>()
        .join("\n")
}

fn symbol_to_number(c: char) -> u8 {
    match c {
        '>' => 1,
        'v' => 2,
        _   => 0,
    }
}

fn number_to_symbol(n: u8) -> char {
    match n {
        1 => '>',
        2 => 'v',
        _ => '.',
    }
}
