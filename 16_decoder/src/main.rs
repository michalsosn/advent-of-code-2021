use bit_vec::BitVec;
use std::io;
use std::io::BufRead;
use std::iter;


fn main() {
    let Input { lines } = Input::read_stdin();
    // println!("{:?}", lines);

    for line in lines {
        println!("## processing line {:?}", line);
        let expression: Expression = parse_packets(&line);
        println!("expression = {:?}", expression);
        let version_sum: u32 = expression.version_sum();
        println!("version_sum = {}", version_sum);
        let value: u64 = expression.value();
        println!("value = {}", value);
    }
}

#[derive(Debug)]
struct Input {
    lines: Vec<BitVec>
}

impl Input {
    fn read_stdin() -> Self {
        let lines = io::stdin().lock().lines()
            .map(|input| {
                let line = input.expect("error: unable to read line");
                let bytes = hex::decode(line.trim()).expect("error: couldn't decode hex");
                BitVec::from_bytes(bytes.as_slice())
            })
            .collect();
        Input {
            lines
        }
    }
}

#[derive(Debug)]
enum Expression {
    Literal {
        version: u8,
        type_id: u8,
        value: u64
    },
    Operator {
        version: u8,
        type_id: u8,
        length_type_id: bool,
        expressions: Vec<Expression>
    }
}

impl Expression {
    fn parse(line: &BitVec, start: &mut usize) -> Self {
        let version = get_number(line, start, 3) as u8;
        let type_id = get_number(line, start, 3) as u8;
        match type_id {
            4 => {
                let mut value: u64 = 0;
                loop {
                    let more_parts = get_flag(line, start);
                    let part: u32 = get_number(line, start, 4);
                    value = value * 16 + part as u64;
                    if !more_parts {
                        break;
                    }
                }
                Expression::Literal { version, type_id, value }
            }
            _ => {
                let length_type_id = get_flag(line, start);
                let expressions = if length_type_id {
                    let subpacket_count: u32 = get_number(line, start, 11);
                    iter::repeat_with(|| Expression::parse(line, start))
                        .take(subpacket_count as usize)
                        .collect()
                } else {
                    let subpacket_length: u32 = get_number(line, start, 15);
                    let expected_end = *start + subpacket_length as usize;
                    let mut expressions = Vec::new();
                    loop {
                        let subexpression = Expression::parse(line, start);
                        expressions.push(subexpression);
                        if *start >= expected_end {
                            break;
                        }
                    }
                    expressions
                };
                Expression::Operator { version, type_id, length_type_id, expressions }
            }
        }
    }

    fn version_sum(&self) -> u32 {
        match self {
            Self::Literal { version, .. } =>
                *version as u32,
            Self::Operator { version, expressions, .. } =>
                (*version as u32) + expressions.iter().map(|e| e.version_sum()).sum::<u32>(),
        }
    }

    fn value(&self) -> u64 {
        match self {
            Self::Literal { value, .. } => *value,
            Self::Operator { type_id: 0, expressions, .. } =>
                expressions.iter().map(|e| e.value()).sum(),
            Self::Operator { type_id: 1, expressions, .. } =>
                expressions.iter().map(|e| e.value()).reduce(|a, b| a * b).unwrap(),
            Self::Operator { type_id: 2, expressions, .. } =>
                expressions.iter().map(|e| e.value()).reduce(|a, b| std::cmp::min(a, b)).unwrap(),
            Self::Operator { type_id: 3, expressions, .. } =>
                expressions.iter().map(|e| e.value()).reduce(|a, b| std::cmp::max(a, b)).unwrap(),
            Self::Operator { type_id: 5, expressions, .. } =>
                if expressions[0].value() > expressions[1].value() { 1 } else { 0 },
            Self::Operator { type_id: 6, expressions, .. } =>
                if expressions[0].value() < expressions[1].value() { 1 } else { 0 },
            Self::Operator { type_id: 7, expressions, .. } =>
                if expressions[0].value() == expressions[1].value() { 1 } else { 0 },
            Self::Operator { type_id, expressions, .. } =>
                panic!("Invalid operator {:?}", self),
        }
    }
}

fn parse_packets(line: &BitVec) -> Expression {
    Expression::parse(&line, &mut 0)
}

fn get_flag(line: &BitVec, start: &mut usize) -> bool {
    let flag: bool = line[*start];
    *start += 1;
    flag
}

fn get_number(line: &BitVec, start: &mut usize, len: usize) -> u32 {
    let end = *start + len;
    let mut n: u32 = 0;
    for i in *start..end {
        n = n * 2 + line[i] as u32;
    }
    *start += len;
    n
}
