pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        15
    }
    fn main1(input: &Vec<String>) -> String {
        self::main1(input)
    }
    fn main2(input: &Vec<String>) -> String {
        self::main2(input)
    }
}
use super::intcode;

#[derive(Clone)]
struct State {
    steps: i64,
    pos: (i64, i64),
    computer: intcode::IntcodeComputer,
}

fn step(
    mut state: State,
    direction: i64,
    seen: &mut std::collections::HashSet<(i64, i64)>,
) -> Option<(State, bool)> {
    let (x, y) = state.pos;
    let next_pos = match direction {
        1 => (x, y + 1),
        2 => (x, y - 1),
        3 => (x - 1, y),
        4 => (x + 1, y),
        _ => panic!(),
    };
    if seen.contains(&next_pos) {
        None
    } else {
        state.computer.provide_input(direction);
        match state.computer.consume_output() {
            0 => None,
            1 => {
                seen.insert(next_pos);
                Some((
                    State {
                        steps: state.steps + 1,
                        pos: next_pos,
                        computer: state.computer,
                    },
                    false,
                ))
            }
            2 => {
                seen.insert(next_pos);
                Some((
                    State {
                        steps: state.steps + 1,
                        pos: next_pos,
                        computer: state.computer,
                    },
                    true,
                ))
            }
            _ => panic!(),
        }
    }
}
fn step2(
    state: (i64, i64, i64),
    direction: i64,
    map: &std::collections::HashSet<(i64, i64)>,
    seen: &mut std::collections::HashSet<(i64, i64)>,
) -> Option<(i64, i64, i64)> {
    let (x, y, num_steps) = state;
    let next_pos = match direction {
        1 => (x, y + 1),
        2 => (x, y - 1),
        3 => (x - 1, y),
        4 => (x + 1, y),
        _ => panic!(),
    };
    if seen.contains(&next_pos) {
        None
    } else if !map.contains(&next_pos) {
        None
    } else {
        seen.insert(next_pos);
        Some((next_pos.0, next_pos.1, num_steps + 1))
    }
}

pub fn main1(input: &Vec<String>) -> String {
    let mut computer = intcode::IntcodeComputer::new_from_input_lines(input);

    computer.start();

    let mut queueue = std::collections::VecDeque::new();

    queueue.push_back(State {
        steps: 0,
        pos: (0, 0),
        computer: computer,
    });
    let mut seen = std::collections::HashSet::new();

    let _pos;
    let num_steps;

    loop {
        let state = queueue.pop_front().unwrap();

        if let Some((next_state, finished)) = step(state.clone(), 1, &mut seen) {
            if finished {
                _pos = next_state.pos;
                num_steps = next_state.steps;
                break;
            } else {
                queueue.push_back(next_state);
            }
        }
        if let Some((next_state, finished)) = step(state.clone(), 2, &mut seen) {
            if finished {
                _pos = next_state.pos;
                num_steps = next_state.steps;
                break;
            } else {
                queueue.push_back(next_state);
            }
        }
        if let Some((next_state, finished)) = step(state.clone(), 3, &mut seen) {
            if finished {
                _pos = next_state.pos;
                num_steps = next_state.steps;
                break;
            } else {
                queueue.push_back(next_state);
            }
        }
        if let Some((next_state, finished)) = step(state, 4, &mut seen) {
            if finished {
                _pos = next_state.pos;
                num_steps = next_state.steps;
                break;
            } else {
                queueue.push_back(next_state);
            }
        }
    }
    //println!("Found the exit {:?} after {:?} steps", pos, num_steps);
    format!("{}", num_steps)
}
pub fn main2(input: &Vec<String>) -> String {
    let mut computer = intcode::IntcodeComputer::new_from_input_lines(input);

    computer.start();

    let mut queueue = std::collections::VecDeque::new();

    queueue.push_back(State {
        steps: 0,
        pos: (0, 0),
        computer: computer,
    });
    let mut seen = std::collections::HashSet::new();

    while queueue.len() > 0 {
        let state = queueue.pop_front().unwrap();

        if let Some((next_state, _)) = step(state.clone(), 1, &mut seen) {
            queueue.push_back(next_state);
        }
        if let Some((next_state, _)) = step(state.clone(), 2, &mut seen) {
            queueue.push_back(next_state);
        }
        if let Some((next_state, _)) = step(state.clone(), 3, &mut seen) {
            queueue.push_back(next_state);
        }
        if let Some((next_state, _)) = step(state, 4, &mut seen) {
            queueue.push_back(next_state);
        }
    }
    let mut queueue2 = std::collections::VecDeque::new();

    queueue2.push_back((14, -14, 0));
    let mut seen2 = std::collections::HashSet::new();

    let mut max = 0;

    while queueue2.len() > 0 {
        let state = queueue2.pop_front().unwrap();
        if state.2 > max {
            max = state.2;
        }

        if let Some(next_state) = step2(state, 1, &seen, &mut seen2) {
            queueue2.push_back(next_state);
        }
        if let Some(next_state) = step2(state, 2, &seen, &mut seen2) {
            queueue2.push_back(next_state);
        }
        if let Some(next_state) = step2(state, 3, &seen, &mut seen2) {
            queueue2.push_back(next_state);
        }
        if let Some(next_state) = step2(state, 4, &seen, &mut seen2) {
            queueue2.push_back(next_state);
        }
    }
    format!("{:?}", max)
}
