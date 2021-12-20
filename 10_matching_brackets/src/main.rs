use std::collections::VecDeque;
use std::io;
use std::io::BufRead;


fn main() {
    let Input { lines } = Input::read_stdin();
    // println!("{:?}", lines);

    let score: u64 = score_corrupted(lines.as_slice());
    println!("score {}", score);

    let score: u64 = score_incomplete(lines.as_slice());
    println!("score {}", score);
}


#[derive(Debug)]
struct Input {
    lines: Vec<Vec<char>>,
}

impl Input {
    fn read_stdin() -> Self {
        let lines = io::stdin().lock().lines()
            .map(|l| l.expect("error: unable to read line").trim()
                 .chars()
                 .collect())
            .collect();
        Input {
            lines
        }
    }
}

#[derive(Debug,PartialEq)]
enum ValidationResult {
    Valid,
    Corrupted(char),
    Incomplete(Vec<char>),
    Invalid
}

fn score_corrupted(lines: &[Vec<char>]) -> u64 {
    use ValidationResult::Corrupted;
    let mut total_score: u64 = 0;
    for line in lines {
        let result = validate_line(line.as_slice());
        let score = match result {
            Corrupted(')') => 3,
            Corrupted(']') => 57,
            Corrupted('}') => 1197,
            Corrupted('>') => 25137,
            _ => 0
        };
        println!("{:?} {:?}", line, result);
        total_score += score;
    }
    total_score
}

fn score_incomplete(lines: &[Vec<char>]) -> u64 {
    use ValidationResult::Incomplete;
    let mut scores: Vec<u64> = Vec::new();
    for line in lines {
        let result = validate_line(line.as_slice());
        match result {
            Incomplete(completion) => {
                let score = completion.iter().map(|c| match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0
                }).fold(0, |acc, n| acc * 5 + n);
                scores.push(score);
            },
            _ => {}
        };
    }
    scores.sort();
    scores[scores.len() / 2]
}

fn validate_line(line: &[char]) -> ValidationResult {
    use ValidationResult::*;
    let mut stack: VecDeque<char> = VecDeque::new();
    let mut result = Valid;
    for &c in line {
        match c {
            '(' | '[' | '{' | '<' =>
                stack.push_back(c),
            ')' | ']' | '}' | '>' => {
                match stack.pop_back() {
                    Some(d) if d == matching(c) => {},
                    Some(_) => {
                        result = Corrupted(c);
                        break;
                    },
                    None => {
                        result = Invalid;
                        break;
                    },
                }
            },
            _ => panic!("Invalid char {}", c),
        }
    }
    if result == Valid && !stack.is_empty() {
        let completion = stack.iter().rev().map(|&c| matching(c)).collect();
        result = Incomplete(completion);
    }
    result
}

fn matching(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        o => o,
    }
}
