use std::io;
use std::io::BufRead;


fn main() {
    let Input { operations } = Input::read_stdin();
    // println!("{:?}", operations);

    let end_state = run_on_input(operations.as_slice(), &[9, 2, 9, 6, 7, 6, 9, 9, 9, 4, 9, 8, 9, 1]);
    println!("{:?}", end_state);

    let end_state = run_on_input(operations.as_slice(), &[9, 1, 4, 1, 1, 1, 4, 3, 6, 1, 2, 1, 8, 1]);
    println!("{:?}", end_state);
}

#[derive(Debug)]
struct LeftOperand(usize);

impl LeftOperand {
    fn from_input(input: &str) -> Self {
        LeftOperand(AluState::register_name_to_ix(input).unwrap())
    }

    fn value(&self, state: &AluState) -> i64 {
        let LeftOperand(ix) = self;
        state.registers[*ix]
    }

    fn set_value(&self, state: &mut AluState, value: i64) {
        let LeftOperand(ix) = self;
        state.registers[*ix] = value;
    }
}

#[derive(Debug)]
enum RightOperand {
    Lit(i64),
    Ref(usize),
}

impl RightOperand {
    fn from_input(input: &str) -> Self {
        use RightOperand::*;
        match AluState::register_name_to_ix(input) {
            Some(ix) => Ref(ix),
            None => Lit(input.parse().unwrap()),
        }
    }

    fn value(&self, state: &AluState) -> i64 {
        use RightOperand::*;
        match self {
            Lit(value) => *value,
            Ref(ix) => state.registers[*ix],
        }
    }
}

#[derive(Debug)]
enum Operation {
    Inp(LeftOperand),
    Add(LeftOperand, RightOperand),
    Mul(LeftOperand, RightOperand),
    Div(LeftOperand, RightOperand),
    Mod(LeftOperand, RightOperand),
    Eql(LeftOperand, RightOperand),
}

impl Operation {
    fn from_input(input: &str) -> Self {
        use Operation::*;

        let words: Vec<&str> = input.trim().split_whitespace().collect();
        match words.as_slice() {
            ["inp", a] => Inp(LeftOperand::from_input(a)),
            ["add", a, b] => Add(LeftOperand::from_input(a), RightOperand::from_input(b)),
            ["mul", a, b] => Mul(LeftOperand::from_input(a), RightOperand::from_input(b)),
            ["div", a, b] => Div(LeftOperand::from_input(a), RightOperand::from_input(b)),
            ["mod", a, b] => Mod(LeftOperand::from_input(a), RightOperand::from_input(b)),
            ["eql", a, b] => Eql(LeftOperand::from_input(a), RightOperand::from_input(b)),
            _ => panic!("Cannot parse command {}", input)
        }
    }

    fn execute(&self, state: &mut AluState) {
        use Operation::*;
        
        match self {
            Inp(l) => {
                let input = state.input.pop().unwrap();
                l.set_value(state, input);
            },
            Add(l, r) => l.set_value(state, l.value(state) + r.value(state)),
            Mul(l, r) => l.set_value(state, l.value(state) * r.value(state)),
            Div(l, r) => {
                let l_value = l.value(state);
                let r_value = r.value(state);
                if r_value == 0 {
                    panic!("div by 0 in {:?} [{} / {}]", self, l_value, r_value)
                } else {
                    l.set_value(state, l.value(state) / r.value(state))
                }
            },
            Mod(l, r) => {
                let l_value = l.value(state);
                let r_value = r.value(state);
                if l_value < 0 || r_value <= 0 {
                    panic!("invalid mod {:?} [{} % {}]", self, l_value, r_value)
                } else {
                    l.set_value(state, l.value(state) % r.value(state))
                }
            },
            Eql(l, r) => l.set_value(state, if l.value(state) == r.value(state) { 1 } else { 0 }),
        }
    }
}

#[derive(Debug)]
struct Input {
    operations: Vec<Operation>,
}

impl Input {
    fn read_stdin() -> Self {
        let operations: Vec<Operation> = io::stdin().lock().lines()
            .map(|line| {
                let input = line.expect("error: unable to read user input");
                Operation::from_input(&input)
            })
            .collect();
        Input {
            operations
        }
    }
}

#[derive(Debug)]
struct AluState {
    registers: [i64; 4],
    input: Vec<i64>,
}

impl AluState {
    fn register_name_to_ix(name: &str) -> Option<usize> {
        match name {
            "w" => Some(0),
            "x" => Some(1),
            "y" => Some(2),
            "z" => Some(3),
            _ => None,
        }
    }
}

fn run_on_input(operations: &[Operation], input: &[i64]) -> AluState {
    let mut input: Vec<i64> = input.clone().to_vec();
    input.reverse();
    let mut state = AluState { registers: [0; 4], input };
    for operation in operations {
        operation.execute(&mut state);
        // println!("{:?} {:?}", state.registers, operation);
    }
    state
}
