pub struct Advent;

use super::intcode;

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        19
    }
    fn main1(input: &Vec<String>) -> String {
        let mut computer = intcode::IntcodeComputer::new_from_input_lines(input);
        computer.run();

        let mut sum = 0;
        for y in 0..50 {
            for x in 0..50 {
                let mut computer2 = computer.clone();
                computer2.provide_input(x);
                computer2.provide_input(y);
                sum += computer2.current_output().unwrap();
            }
        }

        format!("{}", sum)
    }

    fn main2(input: &Vec<String>) -> String {
        let mut computer = intcode::IntcodeComputer::new_from_input_lines(input);
        computer.run();

        let mut vals = std::collections::HashMap::new();

        let mut y = 1347;
        for &delta in [100, 10, 1].iter() {
            loop {
                let here = get_row(&mut vals, &computer, y);
                let then = get_row(&mut vals, &computer, y - 99);
                let diff = then.1 - here.0;
                if diff >= 99 {
                    break;
                }
                y += delta;
            }
            y -= delta;
        }
        y += 1;
        let here = get_row(&mut vals, &computer, y);
        let result = (here.0, y - 99);

        format!("{}", result.0 * 10000 + result.1)
    }
}

fn get_row(
    vals: &mut std::collections::HashMap<i64, (i64, i64)>,
    computer: &intcode::IntcodeComputer,
    y: i64,
) -> (i64, i64) {
    *vals.entry(y).or_insert(scan_line(computer, y))
}

fn scan_point(mut computer: intcode::IntcodeComputer, x: i64, y: i64) -> i64 {
    computer.provide_input(x);
    computer.provide_input(y);
    computer.current_output().unwrap()
}
fn scan_line(computer: &intcode::IntcodeComputer, y: i64) -> (i64, i64) {
    let mut x = 0;
    let min;
    let max;
    loop {
        if scan_point(computer.clone(), x, y) == 1 {
            min = x;
            break;
        }
        x += 1;
    }
    loop {
        if scan_point(computer.clone(), x, y) == 0 {
            max = x - 1;
            break;
        }
        x += 1;
    }
    (min, max)
}

fn print_between(
    vals: &mut std::collections::HashMap<i64, (i64, i64)>,
    computer: &intcode::IntcodeComputer,
    y0: i64,
    y1: i64,
) {
    let (skip, _) = get_row(vals, computer, y0);

    for y in y0..=y1 {
        let (min, max) = get_row(vals, computer, y);
        for _ in skip..min {
            print!(" ");
        }
        for _ in min..max {
            print!("#");
        }
        println!("");
    }
}
