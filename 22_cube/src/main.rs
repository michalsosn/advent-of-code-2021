use regex::Regex;
use std::io;
use std::io::BufRead;


fn main() {
    let Input { operations } = Input::read_stdin();
    // println!("{:?}", operations);

    let limited_cube = Cube { x_a: -50, x_b: 50, y_a: -50, y_b: 50, z_a: -50, z_b: 50 };
    let on_count = count_on(operations.as_slice(), &limited_cube);
    println!("on_count in {:?} = {}", limited_cube, on_count);

    let cube = find_cube(operations.as_slice());
    let on_count = count_on(operations.as_slice(), &cube);
    println!("on_count in {:?} = {}", cube, on_count);
}

#[derive(Debug,Clone)]
struct Cube {
    x_a: i32,
    x_b: i32,
    y_a: i32,
    y_b: i32,
    z_a: i32,
    z_b: i32,
}

impl Cube {
    fn contains(&self, x: i32, y: i32, z: i32) -> bool {
        self.x_a <= x && x <= self.x_b &&
            self.y_a <= y && y <= self.y_b &&
            self.z_a <= z && z <= self.z_b
    }

    fn intersects(&self, other: &Self) -> bool {
        self.x_b >= other.x_a && self.x_a <= other.x_b &&
            self.y_b >= other.y_a && self.y_a <= other.y_b &&
            self.z_b >= other.z_a && self.z_a <= other.z_b
    }

    fn subsumes(&self, other: &Self) -> bool {
        self.x_a <= other.x_a && other.x_b <= self.x_b &&
            self.y_a <= other.y_a && other.y_b <= self.y_b &&
            self.z_a <= other.z_a && other.z_b <= self.z_b
    }

    fn subsumed(&self, other: &Self) -> bool {
        other.subsumes(self)
    }

    fn size(&self) -> u64 {
        let x_span = (self.x_b - self.x_a) as u64;
        let y_span = (self.y_b - self.y_a) as u64;
        let z_span = (self.z_b - self.z_a) as u64;
        x_span * y_span * z_span
    }

    // assuming the Cubes intersect, but does not subsume each other
    fn split(&self, other: &Self) -> bool {
        self.x_a <= other.x_a && other.x_b <= self.x_b &&
            self.y_a <= other.y_a && other.y_b <= self.y_b &&
            self.z_a <= other.z_a && other.z_b <= self.z_b
    }
}

#[derive(Debug,Clone)]
struct Operation {
    value: bool,
    cube: Cube,
}

#[derive(Debug)]
struct Input {
    operations: Vec<Operation>
}

impl Input {
    fn read_stdin() -> Self {
        let re = Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
        let stdin = io::stdin();

        let operations = stdin.lock().lines()
            .map(|line| {
                let line = line.expect("error: unable to read line");
                let captures = re.captures(&line).unwrap();
                let cube = Cube {
                    x_a: captures.get(2).unwrap().as_str().parse().unwrap(),
                    x_b: captures.get(3).unwrap().as_str().parse().unwrap(),
                    y_a: captures.get(4).unwrap().as_str().parse().unwrap(),
                    y_b: captures.get(5).unwrap().as_str().parse().unwrap(),
                    z_a: captures.get(6).unwrap().as_str().parse().unwrap(),
                    z_b: captures.get(7).unwrap().as_str().parse().unwrap(),
                };
                Operation {
                    value: captures.get(1).unwrap().as_str() == "on",
                    cube,
                }
            })
            .collect();

        Input {
            operations 
        }
    }
}

fn count_on(operations: &[Operation], cube: &Cube) -> u64 {
    let operations_last_first: Vec<Operation> = preprocess_operations(operations, cube);

    let mut count: u64 = 0;
    for z in cube.z_a..=cube.z_b {
        for y in cube.y_a..=cube.y_b {
            for x in cube.x_a..=cube.x_b {
                let value = find_first_value(x, y, z, operations_last_first.as_slice());
                if value {
                    count += 1;
                }
            }
        }
    }
    count
}

fn preprocess_operations(operations: &[Operation], cube: &Cube) -> Vec<Operation> {
    let mut operations_last_first: Vec<Operation> = operations.into_iter()
        .filter(|op| op.cube.intersects(cube))
        .cloned()
        .collect();
    operations_last_first.reverse();
    operations_last_first
}

fn find_first_value(x: i32, y: i32, z: i32, operations_last_first: &[Operation]) -> bool {
    for op in operations_last_first {
        if op.cube.contains(x, y, z) {
            return op.value;
        }
    }
    return false;
}

fn find_cube(operations: &[Operation]) -> Cube {
    Cube {
        x_a: operations.iter().filter(|op| op.value).map(|op| op.cube.x_a).min().unwrap(),
        x_b: operations.iter().filter(|op| op.value).map(|op| op.cube.x_b).max().unwrap(),
        y_a: operations.iter().filter(|op| op.value).map(|op| op.cube.y_a).min().unwrap(),
        y_b: operations.iter().filter(|op| op.value).map(|op| op.cube.y_b).max().unwrap(),
        z_a: operations.iter().filter(|op| op.value).map(|op| op.cube.z_a).min().unwrap(),
        z_b: operations.iter().filter(|op| op.value).map(|op| op.cube.z_b).max().unwrap(),
    }
}
