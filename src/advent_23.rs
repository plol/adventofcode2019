pub struct Advent;

use super::intcode;

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        23
    }
    fn main1(input: &Vec<String>) -> String {
        let mut computers = vec![intcode::IntcodeComputer::new_from_input_lines(input); 50];
        let mut queues = vec![std::collections::VecDeque::new(); 50];
        for i in 0..computers.len() {
            computers[i].start();
            computers[i].provide_input(i as i64);
        }
        loop {
            for i in 0..computers.len() {
                match computers[i].state {
                    intcode::IntcodeState::NeedsInput => {
                        if let Some((x, y)) = queues[i].pop_front() {
                            computers[i].provide_input(x);
                            computers[i].provide_input(y);
                        } else {
                            computers[i].provide_input(-1);
                        }
                    }
                    intcode::IntcodeState::Output => {
                        let addr = computers[i].consume_output();
                        let x = computers[i].consume_output();
                        let y = computers[i].consume_output();

                        if addr == 255 {
                            return format!("{}", y);
                        } else {
                            queues[addr as usize].push_back((x, y));
                        }
                    }
                    _ => panic!(),
                }
            }
        }
    }

    fn main2(input: &Vec<String>) -> String {
        let mut computers = vec![intcode::IntcodeComputer::new_from_input_lines(input); 50];
        let mut queues = vec![std::collections::VecDeque::new(); 50];
        for i in 0..computers.len() {
            computers[i].start();
            computers[i].provide_input(i as i64);
        }
        let mut nat = (0, 0);
        let mut last_y_given_to_address_0_by_nat = 0;
        let mut all_computers_idle_count = 0;
        loop {
            let mut all_computers_idle = true;
            for i in 0..computers.len() {
                match computers[i].state {
                    intcode::IntcodeState::NeedsInput => {
                        if let Some((x, y)) = queues[i].pop_front() {
                            computers[i].provide_input(x);
                            computers[i].provide_input(y);
                        } else {
                            computers[i].provide_input(-1);
                        }
                    }
                    intcode::IntcodeState::Output => {
                        all_computers_idle = false;
                        let addr = computers[i].consume_output();
                        let x = computers[i].consume_output();
                        let y = computers[i].consume_output();

                        if addr == 255 {
                            nat = (x, y);
                        } else {
                            queues[addr as usize].push_back((x, y));
                        }
                    }
                    _ => panic!(),
                }
            }
            if all_computers_idle && queues.iter().all(|q| q.is_empty()) {
                all_computers_idle_count += 1;
            } else {
                all_computers_idle_count = 0;
            }

            if all_computers_idle_count == 10 {
                if nat.1 == last_y_given_to_address_0_by_nat {
                    return format!("{}", last_y_given_to_address_0_by_nat);
                }
                last_y_given_to_address_0_by_nat = nat.1;

                queues[0].push_back(nat);
            }
        }
    }
}
