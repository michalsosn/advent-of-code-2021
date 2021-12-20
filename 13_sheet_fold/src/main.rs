use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::cmp;


fn main() {
    let Input { points, folds } = Input::read_stdin();
    // println!("{:?}", points);
    // println!("{:?}", folds);

    let mut points = points;
    let point_count = count_points(points.as_slice());
    println!("points count at start: {}", point_count);
    for fold in folds {
        points = fold_points(points.as_slice(), &fold);
        let point_count = count_points(points.as_slice());
        println!("points count after fold {:?}: {}", fold, point_count);
    }
    println!("{}", render_points(&points));
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum Fold {
    AlongX(u32),
    AlongY(u32),
}

#[derive(Debug)]
struct Input {
    points: Vec<Point>,
    folds: Vec<Fold>,
}

impl Input {
    fn read_stdin() -> Self {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines()
            .map(|l| l.expect("error: unable to read line").trim().to_string());

        let points: Vec<Point> = lines.by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                let parsed: Vec<u32> = l.split(',').map(|n| n.parse().unwrap()).collect();
                Point { x: parsed[0], y: parsed[1] }
            })
            .collect();

        let folds: Vec<Fold> = lines
            .map(|l| {
                let parsed: Vec<&str> = l.split_whitespace().flat_map(|t| t.split('=')).collect();
                match parsed.as_slice() {
                    ["fold", "along", "y", n] => Fold::AlongY(n.parse().unwrap()),
                    ["fold", "along", "x", n] => Fold::AlongX(n.parse().unwrap()),
                    _ => panic!("Cannot parse fold {}", l),
                }
            })
            .collect();

        Input {
            points,
            folds
        }
    }
}


fn fold_points(points: &[Point], fold: &Fold) -> Vec<Point> {
    let deduplicated: HashSet<Point> = match fold {
        Fold::AlongY(fy) =>
            points.iter().map(|&Point { x, y }| {
                let y = if y < *fy { y } else { 2 * *fy - y };
                Point { x, y }
            }).collect(),
        Fold::AlongX(fx) =>
            points.iter().map(|&Point { x, y }| {
                let x = if x < *fx { x } else { 2 * *fx - x };
                Point { x, y }
            }).collect(),
    };
    deduplicated.into_iter().collect()
}

fn count_points(points: &[Point]) -> usize {
    points.iter().count()
}

fn render_points(points: &[Point]) -> String {
    let Point { x: max_x, y: max_y } = points.iter().fold(Point { x: 0, y: 0 }, |acc, point| Point {
        x: cmp::max(acc.x, point.x),
        y: cmp::max(acc.y, point.y),
    });
    let width = (max_x + 1) as usize;
    let height = (max_y + 1) as usize;
    println!("{} {}", width, height);

    let mut board: Vec<Vec<char>> = vec![vec![' '; width]; height];
    for Point { x, y } in points {
        board[*y as usize][*x as usize] = '#';
    }
    board.iter().map(|r| r.into_iter().collect()).collect::<Vec<String>>().join("\n")
}
