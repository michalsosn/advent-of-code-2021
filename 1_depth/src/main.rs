use std::io;
use std::io::BufRead;


fn main() {
    let Input { depths } = read_input();

    let increases = count_increases(depths.as_slice());
    println!("{}", increases);

    let increases_window = count_increases_window(depths.as_slice(), 3);
    println!("{}", increases_window);
}

#[derive(Debug)]
struct Input {
    depths: Vec<u32>
}

fn read_input() -> Input {
    let mut depths: Vec<u32> = Vec::new();

    for line in io::stdin().lock().lines() {
        let input = line.expect("error: unable to read user input");
        let depth: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Cannot parse {}", input),
        };
        depths.push(depth);
    }

    Input {
        depths
    }
}

fn count_increases(depths: &[u32]) -> usize {
    let mut increases: usize = 0;
    let mut previous: &u32 = &depths[0];
    for depth in depths {
        if depth > previous {
            increases += 1;
        }
        previous = depth;
    }
    return increases;
}

fn count_increases_window(depths: &[u32], width: usize) -> usize {
    let mut increases: usize = 0;
    for (i, depth) in depths.iter().enumerate() {
        if i >= width && depth > &depths[i - width] {
            increases += 1;
        }
    }
    return increases;
}
