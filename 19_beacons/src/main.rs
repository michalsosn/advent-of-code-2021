use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::io;
use std::io::BufRead;


fn main() {
    let Input { scanners } = Input::read_stdin();

    let positions = find_positions(scanners.as_slice());
    let beacon_count = count_unique_beacons(positions.as_slice());
    println!("{}", beacon_count);
    let max_distance = find_max_distance(positions.as_slice());
    println!("{}", max_distance);
}

#[derive(Debug)]
struct Scanner {
    beacons: Vec<Beacon>,
}

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

impl Beacon {
    fn difference_to(&self, other: &Self) -> BeaconDiff {
        BeaconDiff {
            x_d: other.x - self.x,
            y_d: other.y - self.y,
            z_d: other.z - self.z,
        }
    }

    fn move_by(&self, diff: &BeaconDiff) -> Self {
        Beacon {
            x: self.x + diff.x_d,
            y: self.y + diff.y_d,
            z: self.z + diff.z_d,
        }
    }
}

#[derive(Debug,Clone,Copy)]
struct BeaconDiff {
    x_d: i32,
    y_d: i32,
    z_d: i32,
}

impl BeaconDiff {
    fn add_to(&self, other: &Self) -> Self {
        BeaconDiff {
            x_d: self.x_d + other.x_d,
            y_d: self.y_d + other.y_d,
            z_d: self.z_d + other.z_d,
        }
    }

    fn reverse(&self) -> Self {
        BeaconDiff {
            x_d: -self.x_d,
            y_d: -self.y_d,
            z_d: -self.z_d,
        }
    }
}

#[derive(Debug)]
struct Input {
    scanners: Vec<Scanner>,
}

impl Input {
    fn read_stdin() -> Self {
        let mut scanners = Vec::new();
        let mut scanner = Scanner { beacons: Vec::new() };
        for line in io::stdin().lock().lines() {
            let input = line.expect("error: unable to read user input");
            if input.starts_with("--- scanner ") {
                continue;
            } else if input.is_empty() {
                scanners.push(scanner);
                scanner = Scanner { beacons: Vec::new() };
            } else {
                let parsed: Vec<i32> = input.split(',').map(|n| n.parse().unwrap()).collect();
                let beacon = Beacon { x: parsed[0], y: parsed[1], z: parsed[2] };
                scanner.beacons.push(beacon);
            }
        }
        scanners.push(scanner);

        Input {
            scanners,
        }
    }
}


#[derive(Debug,Copy,Clone)]
struct Rotation {
    x_s: i32, y_s: i32, z_s: i32,
    x_i: usize, y_i: usize, z_i: usize,
}

impl Rotation {
    fn rotate_beacon(&self, beacon: &Beacon) -> Beacon {
        let Beacon {x, y, z} = beacon;
        let arr = [x, y, z];
        Beacon {
            x: self.x_s * arr[self.x_i],
            y: self.y_s * arr[self.y_i],
            z: self.z_s * arr[self.z_i],
        }
    }

    fn rotate_scanner(&self, scanner: &Scanner) -> Scanner {
        let Scanner { beacons } = scanner;
        let beacons = beacons.into_iter().map(|b| self.rotate_beacon(b)).collect();
        Scanner { beacons }
    }

    fn rotate_diff(&self, diff: &BeaconDiff) -> BeaconDiff {
        let BeaconDiff {x_d, y_d, z_d} = diff;
        let arr = [x_d, y_d, z_d];
        BeaconDiff {
            x_d: self.x_s * arr[self.x_i],
            y_d: self.y_s * arr[self.y_i],
            z_d: self.z_s * arr[self.z_i],
        }
    }

    fn then_rotate(&self, other: &Rotation) -> Rotation {
        let sgn = [self.x_s, self.y_s, self.z_s];
        let ixs = [self.x_i, self.y_i, self.z_i];
        Rotation {
            x_s: sgn[other.x_i] * other.x_s,
            x_i: ixs[other.x_i],
            y_s: sgn[other.y_i] * other.y_s,
            y_i: ixs[other.y_i],
            z_s: sgn[other.z_i] * other.z_s,
            z_i: ixs[other.z_i],
        }
    }

    fn invert(&self) -> Rotation {
        let mut sgn = [0; 3];
        let mut ixs = [0; 3];
        ixs[self.x_i] = 0;
        sgn[self.x_i] = self.x_s;
        ixs[self.y_i] = 1;
        sgn[self.y_i] = self.y_s;
        ixs[self.z_i] = 2;
        sgn[self.z_i] = self.z_s;
        Rotation {
            x_s: sgn[0], x_i: ixs[0], y_s: sgn[1], y_i: ixs[1], z_s: sgn[2], z_i: ixs[2],
        }
    }
}

static ROTATIONS: [Rotation; 24] = [
    Rotation {x_s:  1, x_i: 0, y_s:  1, y_i: 1, z_s:  1, z_i: 2},
    Rotation {x_s:  1, x_i: 0, y_s:  1, y_i: 2, z_s: -1, z_i: 1},
    Rotation {x_s:  1, x_i: 0, y_s: -1, y_i: 2, z_s:  1, z_i: 1},
    Rotation {x_s:  1, x_i: 0, y_s: -1, y_i: 1, z_s: -1, z_i: 2},
    Rotation {x_s:  1, x_i: 1, y_s: -1, y_i: 2, z_s: -1, z_i: 0},
    Rotation {x_s:  1, x_i: 1, y_s: -1, y_i: 0, z_s:  1, z_i: 2},
    Rotation {x_s:  1, x_i: 1, y_s:  1, y_i: 0, z_s: -1, z_i: 2},
    Rotation {x_s:  1, x_i: 1, y_s:  1, y_i: 2, z_s:  1, z_i: 0},
    Rotation {x_s:  1, x_i: 2, y_s:  1, y_i: 0, z_s:  1, z_i: 1},
    Rotation {x_s:  1, x_i: 2, y_s:  1, y_i: 1, z_s: -1, z_i: 0},
    Rotation {x_s:  1, x_i: 2, y_s: -1, y_i: 1, z_s:  1, z_i: 0},
    Rotation {x_s:  1, x_i: 2, y_s: -1, y_i: 0, z_s: -1, z_i: 1},
    Rotation {x_s: -1, x_i: 2, y_s:  1, y_i: 0, z_s: -1, z_i: 1},
    Rotation {x_s: -1, x_i: 2, y_s:  1, y_i: 1, z_s:  1, z_i: 0},
    Rotation {x_s: -1, x_i: 2, y_s: -1, y_i: 1, z_s: -1, z_i: 0},
    Rotation {x_s: -1, x_i: 2, y_s: -1, y_i: 0, z_s:  1, z_i: 1},
    Rotation {x_s: -1, x_i: 1, y_s:  1, y_i: 0, z_s:  1, z_i: 2},
    Rotation {x_s: -1, x_i: 1, y_s:  1, y_i: 2, z_s: -1, z_i: 0},
    Rotation {x_s: -1, x_i: 1, y_s: -1, y_i: 2, z_s:  1, z_i: 0},
    Rotation {x_s: -1, x_i: 1, y_s: -1, y_i: 0, z_s: -1, z_i: 2},
    Rotation {x_s: -1, x_i: 0, y_s:  1, y_i: 1, z_s: -1, z_i: 2},
    Rotation {x_s: -1, x_i: 0, y_s:  1, y_i: 2, z_s:  1, z_i: 1},
    Rotation {x_s: -1, x_i: 0, y_s: -1, y_i: 2, z_s: -1, z_i: 1},
    Rotation {x_s: -1, x_i: 0, y_s: -1, y_i: 1, z_s:  1, z_i: 2},
];

#[derive(Debug,Copy,Clone)]
struct ScannerPosition<'a> {
    scanner: &'a Scanner,
    rotation: Rotation,
    diff: BeaconDiff,
}

fn find_positions(scanners: &[Scanner]) -> Vec<ScannerPosition> {
    let len = scanners.len();

    let mut positions: Vec<Option<ScannerPosition>> = (0..len).map(|_| None).collect();
    let mut queue: VecDeque<ScannerPosition> = VecDeque::new();

    let position = ScannerPosition { scanner: &scanners[0], rotation: ROTATIONS[0], diff: BeaconDiff { x_d: 0, y_d: 0, z_d: 0 }};
    positions[0] = Some(position);
    queue.push_back(position);

    while !queue.is_empty() {
        let root = queue.pop_front().unwrap();
        let ScannerPosition { scanner: root_scanner, rotation: root_rotation, diff: root_diff } = root;

        for rotation in ROTATIONS.iter() {
            let rotated_root = rotation.rotate_scanner(root_scanner);
            for (i, candidate) in scanners.iter().enumerate() {
                if positions[i].is_some() {
                    continue;
                }
                match check_overlap(&rotated_root, candidate) {
                    Some(diff) => {
                        let root_perspective_rotation = root_rotation.then_rotate(&rotation);
                        let root_perspective_diff = root_diff.add_to(&root_perspective_rotation.invert().rotate_diff(&diff));
                        let absolute_position = ScannerPosition {
                            scanner: &scanners[i],
                            rotation: root_perspective_rotation,
                            diff: root_perspective_diff,
                        };
                        positions[i] = Some(absolute_position);
                        queue.push_back(absolute_position);
                    }
                    None => {}
                }
            }
        }
    }

    positions.into_iter().map(|o| o.unwrap()).collect()
}

fn check_overlap(root: &Scanner, candidate: &Scanner) -> Option<BeaconDiff> {
    let Scanner { beacons: root_beacons } = root;
    let Scanner { beacons: candidate_beacons } = candidate;
    let candidate_set: BTreeSet<&Beacon> = candidate_beacons.iter().collect();

    for root_head in root_beacons {
        for candidate in candidate_beacons {
            let diff = root_head.difference_to(candidate);
            let mut hit_count = 0;
            for root in root_beacons {
                let moved = root.move_by(&diff);
                if candidate_set.contains(&moved) {
                    hit_count += 1;
                } else if moved.x.abs() <= 1000 && moved.y.abs() <= 1000 && moved.z.abs() <= 1000 {
                    hit_count = 0;
                    break;
                }
            }
            if hit_count >= 12 {
                return Some(diff.reverse());
            }
        }
    }

    None
}

fn count_unique_beacons(positions: &[ScannerPosition]) -> u32 {
    let mut beacon_set: BTreeSet<Beacon> = BTreeSet::new();
    for position in positions {
        let ScannerPosition { scanner, rotation, diff } = position;
        let Scanner { beacons } = scanner;
        for beacon in beacons {
            let moved = rotation.invert().rotate_beacon(beacon).move_by(&diff);
            beacon_set.insert(moved);
        }
    }

    beacon_set.len() as u32
}

fn find_max_distance(positions: &[ScannerPosition]) -> u32 {
    let mut max_distance: u32 = 0;

    for left in positions.iter() {
        let ldiff = left.diff;
        for right in positions.iter() {
            let rdiff = right.diff;
            let distance = (ldiff.x_d - rdiff.x_d).abs() + (ldiff.y_d - rdiff.y_d).abs() + (ldiff.z_d - rdiff.z_d).abs();
            let distance = distance as u32;

            if distance > max_distance {
                max_distance = distance;
            }
        }
    }

    max_distance
}
