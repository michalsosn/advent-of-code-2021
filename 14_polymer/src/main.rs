use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io;
use std::io::BufRead;


fn main() {
    let Input { polymer, rules } = Input::read_stdin();
    // println!("{:?}", polymer);
    // println!("{:?}", rules);

    let score = grow_polymer(polymer.as_slice(), rules.as_slice(), 10);
    println!("{}", score);

    let score = grow_polymer(polymer.as_slice(), rules.as_slice(), 40);
    println!("{}", score);
}

#[derive(Debug)]
struct Rule {
    a: char,
    b: char,
    c: char
}

#[derive(Debug)]
struct Input {
    polymer: Vec<char>,
    rules: Vec<Rule>,
}

impl Input {
    fn read_stdin() -> Self {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines()
            .map(|l| l.expect("error: unable to read line").trim().to_string());

        let mut polymers: Vec<Vec<char>> = lines.by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| l.chars().collect())
            .collect();
        let polymer: Vec<char> = polymers.pop().unwrap();

        let rules: Vec<Rule> = lines.by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                let chars: Vec<char> = l.chars().collect();
                Rule { a: chars[0], b: chars[1], c: chars[6] }
            })
            .collect();

        Input {
            polymer,
            rules
        }
    }
}


fn grow_polymer(polymer: &[char], rules: &[Rule], steps: u32) -> u64 {
    let unique_chars: BTreeSet<char> = BTreeSet::from_iter(
        polymer.iter().cloned().chain(rules.iter().flat_map(|r| vec![r.a, r.b, r.c].into_iter()))
    );
    let unique_chars: Vec<char> = unique_chars.into_iter().collect();
    let char_to_index: BTreeMap<char, usize> = unique_chars.iter().enumerate().map(|(i, c)| (*c, i)).collect();
    let len: usize = unique_chars.len();

    let mut pair_counts: Vec<Vec<u64>> = vec![vec![0; len]; len];
    for (a, b) in polymer.iter().zip(polymer.iter().skip(1)) {
        pair_counts[char_to_index[a]][char_to_index[b]] += 1;
    }

    let mut rule_matrix: Vec<Vec<usize>> = vec![vec![0; len]; len];
    for Rule {a, b, c} in rules {
        rule_matrix[char_to_index[a]][char_to_index[b]] = char_to_index[c];
    }

    for _ in 0..steps {
        let mut new_pair_counts: Vec<Vec<u64>> = vec![vec![0; len]; len];
        for (y, row) in pair_counts.iter().enumerate() {
            for (x, count) in row.into_iter().enumerate() {
                let z = rule_matrix[y][x];
                new_pair_counts[y][z] += count;
                new_pair_counts[z][x] += count;
            }
        }
        pair_counts = new_pair_counts;
    }

    let mut char_counts: Vec<u64> = vec![0; len];
    char_counts[char_to_index[polymer.first().unwrap()]] += 1;
    char_counts[char_to_index[polymer.last().unwrap()]] += 1;
    for (y, row) in pair_counts.iter().enumerate() {
        for (x, count) in row.into_iter().enumerate() {
            char_counts[y] += count;
            char_counts[x] += count;
        }
    }
    for c in char_counts.iter_mut() {
        *c /= 2;
    }

    char_counts.iter().max().unwrap() - char_counts.iter().min().unwrap()
}

