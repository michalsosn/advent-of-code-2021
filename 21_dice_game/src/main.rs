use regex::Regex;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;


fn main() {
    let Input { positions } = Input::read_stdin();
    println!("{:?}", positions);

    // let score = simulate_game(positions.as_slice());
    // println!("score = {}", score);

    let score = simulate_game_multi(positions.as_slice());
    println!("score = {:?}, best = {}", score, score.win_counts.iter().max().unwrap());
}

#[derive(Debug)]
struct Input {
    positions: Vec<u32>
}

impl Input {
    fn read_stdin() -> Self {
        let re = Regex::new(r"Player \d+ starting position: (\d+)").unwrap();
        let stdin = io::stdin();
        let positions = stdin.lock().lines()
            .map(|line| {
                let line = line.expect("error: unable to read line");
                let captures = re.captures(&line).unwrap();
                captures.get(1).unwrap().as_str().parse().unwrap()
            })
            .collect();

        Input {
            positions
        }
    }
}


static ROLL_COUNT: u32 = 3;
static MAX_POSITION: u32 = 10;
static MAX_DIE_STATE: u32 = 100;
static MAX_SCORE: u32 = 1000;

struct DieState {
    state: u32,
    count: u32,
}

impl DieState {
    fn new() -> Self {
        DieState {
            state: 0,
            count: 0,
        }
    }

    fn roll(&mut self) -> u32 {
        let result = self.state + 1;
        self.state = (self.state + 1) % MAX_DIE_STATE;
        self.count += 1;
        result
    }
}

fn simulate_game(positions: &[u32]) -> u32 {
    let len = positions.len();

    let mut positions: Vec<u32> = positions.to_vec();
    let mut scores: Vec<u32> = vec![0; len];
    let mut die_state: DieState = DieState::new();
    let mut turn: u32 = 1;

    'game: loop {
        for i in 0..len {
            let mut roll_sum = 0;
            for _ in 0..ROLL_COUNT {
                let roll = die_state.roll();
                roll_sum += roll;
            }
            positions[i] = (positions[i] - 1 + roll_sum) % MAX_POSITION + 1;
            scores[i] += positions[i];

            println!("Player {} rolled a total of {}, moved to {} for a total score of {}", i+1, roll_sum, positions[i], scores[i]);
            if scores[i] >= MAX_SCORE {
                break 'game;
            }
        }
        turn += 1;
    }

    let loser_score = scores.iter().min().unwrap();
    loser_score * die_state.count
}

#[derive(Debug)]
struct GameState {
    positions: [u32; 2],
    scores: [u32; 2],
    next_player: usize
}

#[derive(Debug)]
struct MultiScore {
    win_counts: [u64; 2]
}

static WIN_SCORE_MULTI: u32 = 21;
static POSSIBLE_ROLLS: [(u32, u64); 7] = [
    (3, 1),
    (4, 3),
    (5, 6),
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1),
];

fn simulate_game_multi(positions: &[u32]) -> MultiScore {
    let mut state = GameState {
        positions: [positions[0], positions[1]],
        scores: [0, 0],
        next_player: 0
    };
    let mut score = MultiScore {
        win_counts: [0, 0]
    };
    go_simulate_game_multi(&mut state, &mut score, 1);
    score
}


fn go_simulate_game_multi(state: &mut GameState, multi_score: &mut MultiScore, times_acc: u64) {
    // println!("{:?}", state);
    let player: usize = state.next_player;
    let position: u32 = state.positions[player];
    let score: u32 = state.scores[player];

    for (roll, times) in POSSIBLE_ROLLS {
        let new_position = (position - 1 + roll) % MAX_POSITION + 1;
        state.positions[player] = new_position;
        state.scores[player] = score + new_position;
        state.next_player = (player + 1) % 2;

        if state.scores[player] >= WIN_SCORE_MULTI {
            multi_score.win_counts[player] += times_acc * times;
        } else {
            go_simulate_game_multi(state, multi_score, times_acc * times);
        }

        state.positions[player] = position;
        state.scores[player] = score;
        state.next_player = player;
    }
}
