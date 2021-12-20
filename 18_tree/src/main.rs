use std::io;
use std::io::BufRead;
use nom::{
    IResult,
    Finish,
    branch::alt,
    character::complete::{char,digit1},
    combinator::{map, map_res}
};


fn main() {
    let Input { trees } = Input::read_stdin();
    // println!("{:?}", trees);

    let result = trees.iter().fold(None, |acc, tree| {
        match acc {
            None => Some(tree.clone()),
            Some(prev) => Some(Tree::combine(&prev, &tree)),
        }
    }).unwrap();
    println!("{:?}", result);
    println!("magnitude = {}", result.magnitude());

    let mut max_magnitude = 0;
    for (i, left) in trees.iter().enumerate() {
        for (j, right) in trees.iter().enumerate() {
            if i != j {
                let combined = Tree::combine(left, right);
                let magnitude = combined.magnitude();
                if max_magnitude < magnitude {
                    max_magnitude = magnitude;
                }
            }
        }
    }
    println!("max_magnitude = {}", max_magnitude);
}

#[derive(Debug)]
struct Input {
    trees: Vec<Tree>
}

impl Input {
    fn read_stdin() -> Self {
        let trees = io::stdin().lock().lines()
            .map(|input| {
                let line = input.expect("error: unable to read line");
                Input::tree(&line).finish().unwrap().1
            })
            .collect();
        Input {
            trees
        }
    }

    fn value(input: &str) -> IResult<&str, Tree> {
        map(map_res(digit1, |s: &str| s.parse::<u8>()), |n| Tree::Value(n))(input)
    }

    fn pair(input: &str) -> IResult<&str, Tree> {
        let (input, _) = char('[')(input)?;
        let (input, left) = Input::tree(input)?;
        let (input, _) = char(',')(input)?;
        let (input, right) = Input::tree(input)?;
        let (input, _) = char(']')(input)?;
        Ok((input, Tree::tree(left, right)))
    }

    fn tree(input: &str) -> IResult<&str, Tree> {
        alt((Input::value, Input::pair))(input)
    }
}

#[derive(Clone,Debug)]
enum Tree {
    Pair {
        left: Box<Tree>,
        right: Box<Tree>,
    },
    Value(u8),
}

enum ExplodeResult {
    Unchanged,
    Exploded { new_tree: Tree, add_left: u8, add_right: u8 }
}

enum SplitResult {
    Unchanged,
    Split(Tree)
}

impl Tree {
    fn tree(left: Self, right: Self) -> Self {
        Self::Pair {
            left: Box::new(left),
            right: Box::new(right)
        }
    }

    fn combine(left: &Self, right: &Self) -> Self {
        let mut tree = Self::tree((*left).clone(), (*right).clone());
        loop {
            match tree.explode() {
                ExplodeResult::Exploded { new_tree, .. } => {
                    tree = new_tree;
                    continue;
                }
                ExplodeResult::Unchanged => { }
            }
            match tree.split() {
                SplitResult::Split(new_tree) => {
                    tree = new_tree;
                    continue;
                }
                SplitResult::Unchanged => { }
            }
            break;
        };
        tree
    }

    fn explode(&self) -> ExplodeResult {
        self.go_explode(0)
    }

    fn go_explode(&self, depth: u8) -> ExplodeResult {
        use ExplodeResult::*;

        match self {
            Self::Pair { left, right } if depth > 3 && left.is_value() && right.is_value() => {  // box pattern matching hard ?
                return Exploded {
                    new_tree: Self::Value(0),
                    add_left: left.value().unwrap(),
                    add_right: right.value().unwrap()
                };
            }
            Self::Pair { left, right } => {
                match left.go_explode(depth + 1) {
                    Exploded { new_tree: new_left, add_left, add_right } => {
                        let new_right = if add_right > 0 {
                            Box::new(right.add_leftmost(add_right))
                        } else {
                            (*right).clone()
                        };
                        return Exploded {
                            new_tree: Self::Pair { left: Box::new(new_left), right: new_right },
                            add_left,
                            add_right: 0
                        };
                    },
                    Unchanged => {},
                }
                match right.go_explode(depth + 1) {
                    Exploded { new_tree: new_right, add_left, add_right } => {
                        let new_left = if add_left > 0 {
                            Box::new(left.add_rightmost(add_left))
                        } else {
                            (*left).clone()
                        };
                        return Exploded {
                            new_tree: Self::Pair { left: new_left, right: Box::new(new_right) },
                            add_left: 0,
                            add_right
                        };
                    },
                    Unchanged => {},
                }
                return Unchanged;
            },
            Self::Value(_) => {
                return Unchanged;
            }
        }
    }

    fn add_rightmost(&self, add: u8) -> Self {
        match self {
            Self::Value(value) => Self::Value(value + add),
            Self::Pair { left, right } => Self::Pair {
                left: (*left).clone(),
                right: Box::new(right.add_rightmost(add)),
            },
        }
    }

    fn add_leftmost(&self, add: u8) -> Self {
        match self {
            Self::Value(value) => Self::Value(value + add),
            Self::Pair { left, right } => Self::Pair {
                left: Box::new(left.add_leftmost(add)),
                right: (*right).clone(),
            },
        }
    }

    fn is_value(&self) -> bool {
        match self {
            Self::Value(_) => true,
            Self::Pair { .. } => false,
        }
    }

    fn value(&self) -> Option<u8> {
        match self {
            Self::Value(value) => Some(*value),
            Self::Pair { .. } => None,
        }
    }

    fn split(&self) -> SplitResult {
        use SplitResult::*;
        match self {
            Self::Value(value) if *value >= 10 => {
                let left = Self::Value(value / 2);
                let right = Self::Value((value + 1) / 2);
                Split(Tree::tree(left, right))
            },
            Self::Value(_) =>
                Unchanged,
            Self::Pair { left, right } => {
                match left.split() {
                    Split(new_left) => {
                        return Split(Self::Pair {
                            left: Box::new(new_left),
                            right: (*right).clone(),
                        });
                    }
                    Unchanged => {}
                };
                match right.split() {
                    Split(new_right) => {
                        return Split(Self::Pair {
                            left: (*left).clone(),
                            right: Box::new(new_right),
                        });
                    }
                    Unchanged => {}
                };
                return Unchanged;
            },
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            Self::Value(value) =>
                *value as u64,
            Self::Pair { left, right } =>
                3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}
