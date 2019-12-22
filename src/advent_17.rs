use super::intcode;

pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        17
    }
    fn main1(input: &Vec<String>) -> String {
        let computer = intcode::IntcodeComputer::new_from_input_lines(input);

        let grid = parse_grid(computer);

        let mut intersections = vec![];

        for &(gx, gy) in &grid {
            if grid.contains(&(gx + 1, gy))
                && grid.contains(&(gx - 1, gy))
                && grid.contains(&(gx, gy + 1))
                && grid.contains(&(gx, gy - 1))
            {
                intersections.push((gx, gy));
            }
        }

        format!(
            "{:?}",
            intersections.iter().map(|(x, y)| x * y).sum::<i32>()
        )
    }
    fn main2(input: &Vec<String>) -> String {
        let mut computer = intcode::IntcodeComputer::new_from_input_lines(input);
        // 12345678901234567890
        //                   L,12,L,12,R,4    R,6,4,R,6,R,4,R,4
        //                   L,12,L,12,R,4
        // R,6,L,12,L,12                      R,6,4,R,6,R,4,R,4
        //                   L,12,L,12,R,4    R,6,4,R,6,R,4,R,4
        // R,6,L,12,L,12
        // R,6,L,12,L,12                      R,6,4,R,6,R,4,R,4
        //
        let main = "B,C,B,A,C,B,C,A,A,C";
        let fn_a = "R,6,L,12,L,12";
        let fn_b = "L,12,L,12,R,4";
        let fn_c = "R,10,R,6,R,4,R,4";
        let verbose = "n";
        let input = format!("{}\n{}\n{}\n{}\n{}\n", main, fn_a, fn_b, fn_c, verbose);
        let mut iter = input.chars().map(|c| c as i64);
        computer.mem[0] = 2;
        let mut last_output = None;
        loop {
            match computer.state {
                intcode::IntcodeState::NotYetStarted => computer.start(),
                intcode::IntcodeState::NeedsInput => {
                    let c = iter.next().unwrap();
                    //print!("{}", c as u8 as char);
                    computer.provide_input(c);
                }
                intcode::IntcodeState::Output => {
                    //print!("{}", c as u8 as char);
                    last_output = Some(computer.consume_output());
                }
                intcode::IntcodeState::Halt => break,
            }
        }
        //println!("Halted and ok: {}", last_output.unwrap());
        format!("{}", last_output.unwrap())
    }
}

fn parse_grid(mut computer: intcode::IntcodeComputer) -> Vec<(i32, i32)> {
    let mut grid = vec![];
    let mut x = 0;
    let mut y = 0;
    let mut w = 0;
    loop {
        match computer.state {
            intcode::IntcodeState::NotYetStarted => computer.start(),
            intcode::IntcodeState::NeedsInput => panic!(),
            intcode::IntcodeState::Output => {
                match computer.consume_output() as u8 as char {
                    '.' => {
                        x += 1;
                    }
                    '^' | '>' | '<' | 'v' => {
                        grid.push((x, y));
                        x += 1;
                    }
                    '#' => {
                        grid.push((x, y));
                        x += 1;
                    }
                    '\n' => {
                        w = w.max(x);
                        x = 0;
                        y += 1;
                    }
                    'X' => {
                        x += 1;
                    }
                    _ => panic!(),
                }
                //print!("{}", c as u8 as char);
            }
            intcode::IntcodeState::Halt => break,
        }
    }
    grid
}
