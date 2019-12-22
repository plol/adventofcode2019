pub struct Advent;

use super::intcode;

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        21
    }
    fn main1(input: &Vec<String>) -> String {
        let mut computer = intcode::IntcodeComputer::new_from_input_lines(input);

        computer.start();

        let input = "NOT J J
AND A J
AND B J
AND C J
AND D J
NOT J J
AND D J
WALK
"
        .bytes()
        .collect::<Vec<_>>();

        format!(
            "{}",
            run_with_ascii_input_and_return_last_output(&mut computer, &input, false)
        )
    }

    fn main2(input: &Vec<String>) -> String {
        let mut computer = intcode::IntcodeComputer::new_from_input_lines(input);

        computer.start();

        let input = "NOT J J
AND A J
AND B J
AND C J
NOT J J
AND D J
NOT T T
AND E T
OR H T
AND T J
RUN
"
        .bytes()
        .collect::<Vec<_>>();

        format!(
            "{}",
            run_with_ascii_input_and_return_last_output(&mut computer, &input, false)
        )
    }
}

fn run_with_ascii_input_and_return_last_output(
    computer: &mut intcode::IntcodeComputer,
    input: &[u8],
    output: bool,
) -> i64 {
    let mut i = 0;
    let mut last_instruction = 0;
    loop {
        match computer.state {
            intcode::IntcodeState::NeedsInput => {
                if output {
                    print!("{}", input[i] as char);
                }
                computer.provide_input(input[i] as i64);
                i += 1;
            }
            intcode::IntcodeState::Output => {
                let c = computer.consume_output();
                if output {
                    print!("{}", c as u8 as char);
                }
                last_instruction = c;
            }
            intcode::IntcodeState::Halt => {
                break;
            }
            _ => panic!(),
        }
    }
    last_instruction
}
