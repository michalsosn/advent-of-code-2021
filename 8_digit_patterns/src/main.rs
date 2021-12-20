use std::collections::HashMap;
use std::collections::BTreeSet;
use std::io;
use std::io::BufRead;


fn main() {
    let Input { cases } = Input::read_stdin();
    // println!("{:?}", cases);

    let special_count: u32 = cases.iter().map(|case| count_special(&case)).sum();
    println!("special digit count {}", special_count);

    let output_sum: u64 = cases.iter().map(|case| decode_output(&case) as u64).sum();
    println!("output sum {}", output_sum);
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Digit {
    code: BTreeSet<u8>,
}

#[derive(Debug)]
struct Case {
    patterns: Vec<Digit>,
    output: Vec<Digit>,
}

#[derive(Debug)]
struct Input {
    cases: Vec<Case>,
}

impl Input {
    fn read_stdin() -> Self {
        let cases = io::stdin().lock().lines()
            .map(|l| l.expect("error: unable to read line").trim()
                 .split_whitespace()
                 .filter(|&s| s != "|")
                 .map(|word| {
                     let code = word.chars().map(|c| c as u8 - 'a' as u8).collect::<BTreeSet<u8>>();
                     Digit { code }
                 })
                 .collect())
            .map(|ns : Vec<Digit>|
                 Case {
                     patterns: ns[0..10].try_into().expect("patterns with incorrect length"),
                     output: ns[10..14].try_into().expect("output with incorrect length"),
                 }
            )
            .collect();
        Input {
            cases
        }
    }
}

fn count_special(case: &Case) -> u32 {
    case.output.iter().filter(|digit| {
        let len = digit.code.len();
        len == 2 || len == 3 || len == 4 || len == 7
    }).count() as u32
}

fn decode_output(case: &Case) -> u32 {
    let mut digit_to_pattern: [Option<&Digit>; 10] = Default::default();
    let mut len_5_patterns: Vec<&Digit> = Vec::new();  // 2,3,5
    let mut len_6_patterns: Vec<&Digit> = Vec::new();  // 0,6,9
    for pattern in &case.patterns {
        match pattern.code.len() {
            2 => digit_to_pattern[1] = Some(pattern),
            3 => digit_to_pattern[7] = Some(pattern),
            4 => digit_to_pattern[4] = Some(pattern),
            5 => len_5_patterns.push(pattern),
            6 => len_6_patterns.push(pattern),
            7 => digit_to_pattern[8] = Some(pattern),
            o => panic!("Cannot match {:?}", o),
        }
    }
    for pattern in len_5_patterns {
        let code = &pattern.code;
        if code.is_superset(&digit_to_pattern[1].unwrap().code) {
            digit_to_pattern[3] = Some(pattern);
        } else if digit_to_pattern[4].unwrap().code.difference(&code).count() == 1 {
            digit_to_pattern[5] = Some(pattern);
        } else {
            digit_to_pattern[2] = Some(pattern);
        }
    }
    for pattern in len_6_patterns {
        let code = &pattern.code;
        if code.is_superset(&digit_to_pattern[3].unwrap().code) {
            digit_to_pattern[9] = Some(pattern);
        } else if code.is_superset(&digit_to_pattern[1].unwrap().code) {
            digit_to_pattern[0] = Some(pattern);
        } else {
            digit_to_pattern[6] = Some(pattern);
        }
    }
    let mut pattern_to_digit: HashMap<&Digit, u32> = HashMap::new();
    for (i, pattern) in digit_to_pattern.iter().enumerate() {
        pattern_to_digit.insert(pattern.unwrap(), i as u32);
    }

    let mut number: u32 = 0;
    for pattern in &case.output {
        let digit = pattern_to_digit.get(pattern).unwrap();
        number = number * 10 + digit;
    }
    // println!("{}", number);
    number
}
