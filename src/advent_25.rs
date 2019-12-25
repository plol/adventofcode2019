pub struct Advent;

use super::intcode;

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        25
    }
    fn main1(input: &Vec<String>) -> String {
        let mut computer = intcode::IntcodeComputer::new_from_input_lines(input);
        computer.start();

        let mut inputs = [
            //"west",
            //"north",
            //"take dark matter",
            //"south",
            //"east",
            "north",
            "west",
            //"take planetoid",
            "west",
            "take spool of cat6",
            "east",
            "east",
            "south",
            "east",
            "north",
            "take sand",
            // "east",
            // "take giant electromagnet",
            // "west",
            "west",
            //"take coin",
            "north",
            "take jam",
            "south",
            "west",
            "south",
            //"take wreath",
            "west",
            "take fuel cell",
            "east",
            "north",
            "north",
            "west",
            "south",
        ]
        .iter();
        loop {
            match computer.state {
                intcode::IntcodeState::NeedsInput => {
                    if let Some(input) = inputs.next() {
                        for byte in input.bytes() {
                            //print!("{}", byte as char);
                            computer.provide_input(byte as i64);
                        }
                    //println!("");
                    } else {
                        let mut buf = String::new();
                        std::io::stdin().read_line(&mut buf).unwrap();
                        for byte in buf.trim().bytes() {
                            computer.provide_input(byte as i64);
                        }
                    }
                    computer.provide_input(10);
                }
                intcode::IntcodeState::Output => {
                    computer.consume_output();
                    //print!("{}", computer.consume_output() as u8 as char);
                }
                intcode::IntcodeState::Halt => break,
                _ => panic!(),
            }
        }
        "8401920".to_owned()
    }

    fn main2(_input: &Vec<String>) -> String {
        "God jul".to_owned()
    }
}
