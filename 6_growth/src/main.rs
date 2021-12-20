use std::io;
use std::io::BufRead;


fn main() {
    let Input { ages } = Input::read_stdin();
    println!("{:?}", ages);

    let count = count_population_after(ages.as_slice(), 18);
    println!("{}", count);
    let count = count_population_after(ages.as_slice(), 80);
    println!("{}", count);
    let count = count_population_after(ages.as_slice(), 256);
    println!("{}", count);
}

#[derive(Debug)]
struct Input {
    ages: Vec<u8>,
}

impl Input {
    fn read_stdin() -> Self {
        let ages = io::stdin().lock().lines()
            .next().unwrap()
            .expect("error: unable to read line")
            .trim()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        Input {
            ages
        }
    }
}

fn count_population_after(ages: &[u8], day: u32) -> u64 {
    let mut count_per_age: [u64; 9] = [0; 9];
    for age in ages {
        count_per_age[*age as usize] += 1;
    }

    for _ in 0..day {
        let reproduce = count_per_age[0];
        for i in 0..8 {
            count_per_age[i] = count_per_age[i + 1];
        }
        count_per_age[6] += reproduce;
        count_per_age[8] = reproduce;
    }

    let population_count = count_per_age.iter().sum();
    return population_count;
}
