use regex::Regex;
use std::cmp;
use std::io;
use std::io::BufRead;


fn main() {
    let Input { operations } = Input::read_stdin();
    // println!("{:?}", operations);

    let limited_cube = Cube { x_a: -50, x_b: 50, y_a: -50, y_b: 50, z_a: -50, z_b: 50 };
    let on_count = count_on(operations.as_slice(), &limited_cube);
    println!("on_count in {:?} = {}", limited_cube, on_count);

    let background_cube = find_background_cube(operations.as_slice());
    let on_count = count_on(operations.as_slice(), &background_cube);
    println!("on_count in {:?} = {}", background_cube, on_count);
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
    fn is_empty(&self) -> bool {
        self.x_a > self.x_b || self.y_a > self.y_b || self.z_a > self.z_b
    }

    fn size(&self) -> u64 {
        let x_span = (self.x_b - self.x_a + 1) as u64;
        let y_span = (self.y_b - self.y_a + 1) as u64;
        let z_span = (self.z_b - self.z_a + 1) as u64;
        x_span * y_span * z_span
    }

    fn cut_to(&self, other: &Self) -> Option<Cube> {
        let cube = Cube {
            x_a: cmp::max(self.x_a, other.x_a),
            x_b: cmp::min(self.x_b, other.x_b),
            y_a: cmp::max(self.y_a, other.y_a),
            y_b: cmp::min(self.y_b, other.y_b),
            z_a: cmp::max(self.z_a, other.z_a),
            z_b: cmp::min(self.z_b, other.z_b),
        };
        if cube.is_empty() {
            None
        } else {
            Some(cube)
        }
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

fn find_background_cube(operations: &[Operation]) -> Cube {
    Cube {
        x_a: operations.iter().filter(|op| op.value).map(|op| op.cube.x_a).min().unwrap(),
        x_b: operations.iter().filter(|op| op.value).map(|op| op.cube.x_b).max().unwrap(),
        y_a: operations.iter().filter(|op| op.value).map(|op| op.cube.y_a).min().unwrap(),
        y_b: operations.iter().filter(|op| op.value).map(|op| op.cube.y_b).max().unwrap(),
        z_a: operations.iter().filter(|op| op.value).map(|op| op.cube.z_a).min().unwrap(),
        z_b: operations.iter().filter(|op| op.value).map(|op| op.cube.z_b).max().unwrap(),
    }
}

fn count_on(operations: &[Operation], background_cube: &Cube) -> u64 {
    let disjoint_operations = calculate_disjoint_operations(operations, background_cube);

    let mut count: u64 = 0;
    for operation in disjoint_operations {
        if operation.value {
            count += operation.cube.size();
        }
    }
    count
}

fn calculate_disjoint_operations(operations: &[Operation], background_cube: &Cube) -> Vec<Operation> {
    // cubes in this vec are always disjoint and cut to background_cube
    let mut disjoint_ops: Vec<Operation> = Vec::new();
    disjoint_ops.push(Operation { value: false, cube: background_cube.clone() });

    for op in operations {
        let mut keep: Vec<bool> = vec![true; disjoint_ops.len()];
        let mut new_ops: Vec<Operation> = Vec::new();

        for (i, old_op) in disjoint_ops.iter().enumerate() {
            if op.value == old_op.value {
                continue;
            }
            match op.cube.cut_to(&old_op.cube) {
                None => {},
                Some(cut_cube) => {
                    keep[i] = false;
                    let cut_op = Operation { value: op.value, cube: cut_cube };
                    let mut result = split_operations(&cut_op, &old_op);
                    new_ops.append(&mut result);
                }
            }
        }

        let mut keep_iter = keep.iter();
        disjoint_ops.retain(|_| *keep_iter.next().unwrap());
        disjoint_ops.append(&mut new_ops);
    }

    disjoint_ops
}

// assuming different values, new takes precedence, cubes intersect, new cube is cut to the old cube
fn split_operations(new_op: &Operation, old_op: &Operation) -> Vec<Operation> {
    let new_c = &new_op.cube;
    let mut old_op = old_op.clone();
    let mut result: Vec<Operation> = Vec::new();

    if old_op.cube.x_a < new_c.x_a {
        let mut split_op = old_op.clone();
        split_op.cube.x_b = new_c.x_a - 1;
        result.push(split_op);
        old_op.cube.x_a = new_c.x_a;
    }
    if new_c.x_b < old_op.cube.x_b {
        let mut split_op = old_op.clone();
        split_op.cube.x_a = new_c.x_b + 1;
        result.push(split_op);
        old_op.cube.x_b = new_c.x_b;
    }

    if old_op.cube.y_a < new_c.y_a {
        let mut split_op = old_op.clone();
        split_op.cube.y_b = new_c.y_a - 1;
        result.push(split_op);
        old_op.cube.y_a = new_c.y_a;
    }
    if new_c.y_b < old_op.cube.y_b {
        let mut split_op = old_op.clone();
        split_op.cube.y_a = new_c.y_b + 1;
        result.push(split_op);
        old_op.cube.y_b = new_c.y_b;
    }

    if old_op.cube.z_a < new_c.z_a {
        let mut split_op = old_op.clone();
        split_op.cube.z_b = new_c.z_a - 1;
        result.push(split_op);
    }
    if new_c.z_b < old_op.cube.z_b {
        let mut split_op = old_op.clone();
        split_op.cube.z_a = new_c.z_b + 1;
        result.push(split_op);
    }

    result.push(new_op.clone());

    result
}
