use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;


fn main() {
    let Input { rooms } = Input::read_stdin();
    println!("{:?}", rooms);

    let min_state: Option<GameState> = find_min_cost_state(&rooms);
    println!("min_state = {:?}", min_state);
    println!("min_cost = {}", min_state.unwrap().total_cost);
}


#[derive(Debug)]
struct Input {
    rooms: [Vec<u8>; 4],
}

impl Input {
    fn read_stdin() -> Self {
        let re = Regex::new(r"..#([A-D])#([A-D])#([A-D])#([A-D])#").unwrap();
        let first_char: i32 = 'A' as i32;
        let rooms: Vec<[u8; 4]> = io::stdin().lock().lines()
            .skip(2)
            .map(|line| line.expect("error: unable to read line").to_string())
            .take_while(|line| re.is_match(&line))
            .map(|line| {
                let captures = re.captures(&line).unwrap();
                [
                    (captures.get(1).unwrap().as_str().chars().next().unwrap() as i32 - first_char + 1) as u8,
                    (captures.get(2).unwrap().as_str().chars().next().unwrap() as i32 - first_char + 1) as u8,
                    (captures.get(3).unwrap().as_str().chars().next().unwrap() as i32 - first_char + 1) as u8,
                    (captures.get(4).unwrap().as_str().chars().next().unwrap() as i32 - first_char + 1) as u8,
                ]
            })
            .collect();
        let rooms: [Vec<u8>; 4] = [
            rooms.iter().map(|row| row[0]).collect(),
            rooms.iter().map(|row| row[1]).collect(),
            rooms.iter().map(|row| row[2]).collect(),
            rooms.iter().map(|row| row[3]).collect(),
        ];
        Input {
            rooms
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct GameState {
    predicted_cost: u64,
    total_cost: u64,
    hallway: [u8; 11],
    rooms: [Vec<u8>; 4],
}

static TYPE_TO_COST: [u64; 5] = [0, 1, 10, 100, 1000];
static ROOM_TO_TYPE: [u8; 4] = [1, 2, 3, 4];
static TYPE_TO_ROOM: [usize; 5] = [0, 0, 1, 2, 3];

impl GameState {
    fn init_state(init_rooms: &[Vec<u8>; 4]) -> GameState {
        let mut state = GameState {
            predicted_cost: 0,
            total_cost: 0,
            hallway: [0; 11],
            rooms: init_rooms.clone(),
        };
        state.predicted_cost = state.predict_remaining_cost();
        state
    }

    fn gen_following_states(&self) -> Vec<GameState> {
        let mut result: Vec<GameState> = Vec::new();

        // move hallway -> the destined room if it contains only the right ppl and no obstacles in the hallway
        for (hallway_ix, &typ) in self.hallway.iter().enumerate() {
            if typ == 0 {
                continue;
            }
            let room_ix = TYPE_TO_ROOM[typ as usize];
            let room_hallway_ix = Self::room_to_hallway_ix(room_ix);
            if self.can_enter_room(room_ix, typ) && self.is_hallway_clear_exclusive(hallway_ix, room_hallway_ix) {
                let mut cost = Self::hallway_cost(hallway_ix, room_hallway_ix, typ);
                let mut new_state = self.clone();
                new_state.hallway[hallway_ix] = 0;
                match new_state.bottom_empty_ix(room_ix) {
                    None => panic!("No empty ix in room despite previous can_enter_room check"),
                    Some(room_inside_ix) => {
                        new_state.rooms[room_ix][room_inside_ix] = typ;
                        cost += Self::room_cost(room_inside_ix, typ);
                    }
                }
                new_state.total_cost += cost;
                new_state.predicted_cost = new_state.total_cost + new_state.predict_remaining_cost();
                result.push(new_state);
            }
        }

        // move room -> hallway if no obstacles
        for room_ix in 0..4 {
            match self.top_occupied_ix(room_ix) {
                None => {},
                Some(room_inside_ix) => {
                    let room_hallway_ix = Self::room_to_hallway_ix(room_ix);
                    let typ = self.rooms[room_ix][room_inside_ix];
                    if ROOM_TO_TYPE[room_ix] == typ {
                        if self.rooms[room_ix].iter().skip(room_inside_ix + 1).all(|&t| t == typ) {
                            continue
                        }
                    }
                    let room_cost = Self::room_cost(room_inside_ix, typ);
                    for hallway_ix in 0..11 {
                        if !Self::is_room_hallway_ix(hallway_ix) && self.is_hallway_clear_exclusive(room_hallway_ix, hallway_ix) {
                            let mut new_state = self.clone();
                            new_state.rooms[room_ix][room_inside_ix] = 0;
                            new_state.hallway[hallway_ix] = typ;
                            let cost = room_cost + Self::hallway_cost(room_hallway_ix, hallway_ix, typ);
                            new_state.total_cost += cost;
                            new_state.predicted_cost = new_state.total_cost + new_state.predict_remaining_cost();
                            result.push(new_state);
                        }
                    }
                }
            }
        }

        result
    }


    fn predict_remaining_cost(&self) -> u64 {
        let mut cost: u64 = 0;

        for (hallway_ix, &typ) in self.hallway.iter().enumerate() {
            if typ > 0 {
                let target_room_ix = TYPE_TO_ROOM[typ as usize];
                cost += Self::hallway_cost(hallway_ix, Self::room_to_hallway_ix(target_room_ix), typ)
                    + Self::room_cost(0, typ);
            }
        }

        for (room_ix, room) in self.rooms.iter().enumerate() {
            for (room_inside_ix, &typ) in room.iter().enumerate() {
                if typ > 0 {
                    let target_room_ix = TYPE_TO_ROOM[typ as usize];
                    if room_ix != target_room_ix {
                        cost += Self::hallway_cost(Self::room_to_hallway_ix(room_ix), Self::room_to_hallway_ix(target_room_ix), typ)
                            + Self::room_cost(room_inside_ix, typ)
                            + Self::room_cost(0, typ);
                    } else if room_inside_ix == 0 && room[1] != typ {
                        cost += 6 * Self::room_cost(0, typ);
                    }
                }
            }
        }

        cost
    }

    fn can_enter_room(&self, room: usize, typ: u8) -> bool {
        if ROOM_TO_TYPE[room] != typ {
            return false;
        }
        if self.rooms[room].iter().any(|&cur_type| cur_type != typ && cur_type != 0) {
            return false;
        }
        self.rooms[room][0] == 0
    }

    fn bottom_empty_ix(&self, room: usize) -> Option<usize> {
        for (i, &v) in self.rooms[room].iter().enumerate().rev() {
            if v == 0 {
                return Some(i);
            }
        }
        return None;
    }

    fn top_occupied_ix(&self, room: usize) -> Option<usize> {
        for (i, &v) in self.rooms[room].iter().enumerate() {
            if v != 0 {
                return Some(i);
            }
        }
        return None;
    }

    fn is_hallway_clear_exclusive(&self, from: usize, to: usize) -> bool {
        if from < to {
            self.is_hallway_clear(from + 1, to)
        } else {
            self.is_hallway_clear(to, from - 1)
        }
    }

    fn is_hallway_clear(&self, from: usize, to: usize) -> bool {
        for i in from..=to {
            if self.hallway[i] > 0 {
                return false;
            }
        }
        return true;
    }

    fn room_to_hallway_ix(room: usize) -> usize {
        room * 2 + 2
    }

    fn is_room_hallway_ix(room: usize) -> bool {
        room == 2 || room == 4 || room == 6 || room == 8
    }

    fn type_cost(typ: u8) -> u64 {
        TYPE_TO_COST[typ as usize]
    }

    fn hallway_cost(from: usize, to: usize, typ: u8) -> u64 {
        (to as i32 - from as i32).abs() as u64 * Self::type_cost(typ)
    }

    fn room_cost(ix: usize, typ: u8) -> u64 {
        (ix as u64 + 1) * Self::type_cost(typ)
    }

    fn is_final(&self) -> bool {
        for (room_ix, room) in self.rooms.iter().enumerate() {
            let exp_type = ROOM_TO_TYPE[room_ix];
            if room.iter().any(|&cur_type| cur_type != exp_type) {
                return false;
            }
        }
        return true;
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.predicted_cost, self.total_cost).cmp(&(other.predicted_cost, self.total_cost)).reverse()
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SeenKey {
    total_cost: u64,
    hallway: [u8; 11],
    rooms: [Vec<u8>; 4]
}

fn find_min_cost_state(init_rooms: &[Vec<u8>; 4]) -> Option<GameState> {
    let mut min_cost: u64 = u64::MAX;
    let mut min_state: Option<GameState> = None;
    let mut queue: BinaryHeap<GameState> = BinaryHeap::new();
    let mut seen: HashSet<SeenKey> = HashSet::new();

    let init_state = GameState::init_state(init_rooms);
    queue.push(init_state);

    while !queue.is_empty() {
        let state = queue.pop().unwrap();
        let key = SeenKey { total_cost: state.total_cost, hallway: state.hallway, rooms: state.rooms.clone() };

        if seen.contains(&key) {
            continue;
        }
        seen.insert(key);

        // println!("{:?}", state);
        if state.total_cost >= min_cost {
            continue;
        }

        let following_states = state.gen_following_states();
        for following_state in following_states {
            if following_state.is_final() {
                if following_state.total_cost < min_cost {
                    min_cost = following_state.total_cost;
                    min_state = Some(following_state);
                    println!("found min cost {} in {:?}", min_cost, min_state);
                }
            } else {
                queue.push(following_state);
            }
        }
    }

    min_state
}

