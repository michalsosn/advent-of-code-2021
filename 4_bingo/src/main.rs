use std::io;
use std::io::BufRead;


fn main() {
    let Input { draws, boards} = Input::read_stdin();
    // println!("{:?} {:?}", draws, boards);

    let result = find_winner(draws.as_slice(), boards.as_slice());
    println!("{:?}", result);
    println!("{}", result.sum_unmarked * result.last_pick as u64);

    let result = find_loser(draws.as_slice(), boards.as_slice());
    println!("{:?}", result);
    println!("{}", result.sum_unmarked * result.last_pick as u64);
}

#[derive(Debug)]
struct Input {
    draws: Vec<u32>,
    boards: Vec<[[u32;5];5]>
}

impl Input {
    fn read_stdin() -> Self {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines()
            .map(|l| l.expect("error: unable to read line").trim().to_string())
            .filter(|l| !l.is_empty());

        let draws: Vec<u32> = lines.next()
            .expect("error: unable to get draws line")
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        let mut boards = Vec::new();
        let mut board = [[0;5];5];
        for (i, line) in lines.enumerate() {
            let row = i % 5;
            let parsed: Vec<u32> = line.split_whitespace().map(|n| n.parse().unwrap()).collect();
            board[row] = parsed.try_into().unwrap();
            if row == 4 {
                boards.push(board);
                board = [[0;5];5];
            }
        }

        Input {
            draws,
            boards
        }
    }
}

#[derive(Debug)]
struct Result {
    sum_unmarked: u64,
    last_pick: u32,
}

fn find_winner(draws: &[u32], boards: &[[[u32;5];5]]) -> Result {
    let len = boards.len();
    let mut row_hits: Vec<[u8; 5]> = vec![[0; 5]; len];
    let mut col_hits: Vec<[u8; 5]> = vec![[0; 5]; len];
    let mut sum_hits: Vec<u64> = vec![0; len];

    let mut winner: usize = len;
    let mut last_pick: u32 = 0;

    'game: for draw in draws {
        for (i, board) in boards.iter().enumerate() {
            for (x, row) in board.iter().enumerate() {
                for (y, cell) in row.iter().enumerate() {
                    if cell == draw {
                        row_hits[i][x] += 1;
                        col_hits[i][y] += 1;
                        sum_hits[i] += *draw as u64;
                    }
                    if row_hits[i][x] == 5 || col_hits[i][y] == 5 {
                        winner = i;
                        last_pick = *draw;
                        break 'game;
                    }
                }
            }
        }
    }

    let sum_winner: u64 = boards[winner].iter().map(|row| row.iter().map(|&n| n as u64).sum::<u64>()).sum();
    let sum_unmarked = sum_winner - sum_hits[winner];

    Result {
        sum_unmarked,
        last_pick,
    }
}


fn find_loser(draws: &[u32], boards: &[[[u32;5];5]]) -> Result {
    let len = boards.len();
    let mut row_hits: Vec<[u8; 5]> = vec![[0; 5]; len];
    let mut col_hits: Vec<[u8; 5]> = vec![[0; 5]; len];
    let mut sum_hits: Vec<u64> = vec![0; len];

    let mut player_won: Vec<bool> = vec![false; len];
    let mut victory_count: usize = 0;

    let mut loser: usize = len;
    let mut last_pick: u32 = 0;

    'game: for draw in draws {
        for (i, board) in boards.iter().enumerate() {
            if player_won[i] {
                continue;
            }
            for (x, row) in board.iter().enumerate() {
                for (y, cell) in row.iter().enumerate() {
                    if cell == draw {
                        row_hits[i][x] += 1;
                        col_hits[i][y] += 1;
                        sum_hits[i] += *draw as u64;
                    }
                    if (row_hits[i][x] == 5 || col_hits[i][y] == 5) && !player_won[i] {
                        player_won[i] = true;
                        victory_count += 1;
                    }
                    if victory_count == len {
                        loser = i;
                        last_pick = *draw;
                        break 'game;
                    }
                }
            }
        }
    }

    let sum_loser: u64 = boards[loser].iter().map(|row| row.iter().map(|&n| n as u64).sum::<u64>()).sum();
    let sum_unmarked = sum_loser - sum_hits[loser];

    Result {
        sum_unmarked,
        last_pick,
    }
}
