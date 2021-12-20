use std::cmp;
use std::io;
use std::io::BufRead;


fn main() {
    let Input { lines } = Input::read_stdin();
    // println!("{:?}", lines);

    let count = count_overlap(lines.as_slice());
    println!("{}", count);
}

#[derive(Debug)]
struct Line {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

#[derive(Debug)]
struct Input {
    lines: Vec<Line>,
}

impl Input {
    fn read_stdin() -> Self {
        let lines = io::stdin().lock().lines()
            .map(|l| l.expect("error: unable to read line").trim().to_string())
            .filter(|l| !l.is_empty())
            .map(|l| l.split(" -> ").flat_map(|p| p.split(',')).map(|n| n.parse().unwrap()).collect())
            .map(|ns: Vec<usize>| match ns.as_slice() {
                [sx, sy, ex, ey] => Line { start_x: *sx, start_y: *sy, end_x: *ex, end_y: *ey },
                other => panic!("Not a line {:?}", other)
            })
            .collect();

        Input {
            lines
        }
    }
}

fn line_range(start: usize, end: usize) -> std::ops::RangeInclusive<usize> {
    let min = cmp::min(start, end);
    let max = cmp::max(start, end);
    return min..=max;
}

fn count_overlap(lines: &[Line]) -> u64 {
    let max_x = lines.iter().map(|l| cmp::max(l.start_x, l.end_x)).max().unwrap();
    let max_y = lines.iter().map(|l| cmp::max(l.start_y, l.end_y)).max().unwrap();
    let width = max_x + 1;
    let height = max_y + 1;

    let mut board = vec![vec![0; width]; height];
    for line in lines {
        match line {
            Line { start_x, start_y, end_x, end_y } if start_x == end_x => {
                for y in line_range(*start_y, *end_y) {
                    board[y][*start_x] += 1;
                }
            }
            Line { start_x, start_y, end_x, end_y } if start_y == end_y => {
                for x in line_range(*start_x, *end_x) {
                    board[*start_y][x] += 1;
                }
            }
            Line { start_x, start_y, end_x, end_y } => {
                let (min_x, len, init_y, ascent_f) = if start_x < end_x {
                    let min_x = start_x;
                    let len = end_x - start_x;
                    let init_y = start_y;
                    let ascent_f = if start_y < end_y { 1 } else { -1 };
                    (min_x, len, init_y, ascent_f)
                } else {
                    let min_x = end_x;
                    let len = start_x - end_x;
                    let init_y = end_y;
                    let ascent_f = if start_y < end_y { -1 } else { 1 };
                    (min_x, len, init_y, ascent_f)
                };
                for i in 0..=len {
                    let x = min_x + i;
                    let y = (*init_y as i32 + i as i32 * ascent_f) as usize;
                    board[y][x] += 1;
                }
            }
        }
    }

    let overlap_count = board.iter().map(|row| row.iter().filter(|&n| *n > 1).count() as u64).sum();
    return overlap_count;
}

