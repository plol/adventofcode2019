pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        7
    }
    fn main1(input: &Vec<String>) -> String {
        self::main(input)
    }
    fn main2(input: &Vec<String>) -> String {
        self::main2(input)
    }
}
fn permutations2<T, F>(k: usize, arr: &mut Vec<T>, output: &mut F)
where
    F: FnMut(&Vec<T>),
{
    if k == 1 {
        output(arr);
    } else {
        permutations2(k - 1, arr, output);
        for i in 0..(k - 1) {
            if k % 2 == 0 {
                arr.swap(i, k - 1);
            } else {
                arr.swap(0, k - 1);
            }
            permutations2(k - 1, arr, output);
        }
    }
}

fn permutations<T, F>(x: &Vec<T>, mut output: F)
where
    T: Clone,
    F: FnMut(&Vec<T>),
{
    permutations2(x.len(), &mut x.clone(), &mut output);
}
pub fn run_with_input(initial_mem: &Vec<i64>, config: i64, signal: i64) -> i64 {
    let mut computer = super::intcode::IntcodeComputer::new(initial_mem.clone());
    computer.start();
    computer.provide_input(config);
    computer.provide_input(signal);
    computer.peek_output().unwrap()
}
pub fn run(initial_mem: &Vec<i64>) -> String {
    let mut max = i64::min_value();
    permutations(&vec![0, 1, 2, 3, 4], |v| {
        let a = run_with_input(&initial_mem, v[0], 0);
        let b = run_with_input(&initial_mem, v[1], a);
        let c = run_with_input(&initial_mem, v[2], b);
        let d = run_with_input(&initial_mem, v[3], c);
        let e = run_with_input(&initial_mem, v[4], d);
        if e > max {
            max = e
        }
    });
    format!("{}", max)
}
pub fn run2(initial_mem: &Vec<i64>) -> String {
    let mut max = i64::min_value();
    permutations(&vec![5, 6, 7, 8, 9], |v| {
        let mut computers = [
            super::intcode::IntcodeComputer::new(initial_mem.clone()),
            super::intcode::IntcodeComputer::new(initial_mem.clone()),
            super::intcode::IntcodeComputer::new(initial_mem.clone()),
            super::intcode::IntcodeComputer::new(initial_mem.clone()),
            super::intcode::IntcodeComputer::new(initial_mem.clone()),
        ];
        for i in 0..computers.len() {
            computers[i].start();
            computers[i].provide_input(v[i]);
        }

        computers[0].provide_input(0);
        loop {
            for i in 0..5 {
                match (computers[i].state, computers[(i + 1) % 5].state) {
                    (
                        super::intcode::IntcodeState::Output,
                        super::intcode::IntcodeState::NeedsInput,
                    ) => {
                        let x = computers[i].consume_output();
                        computers[(i + 1) % 5].provide_input(x);
                    }
                    _ => {}
                }
            }

            if let (super::intcode::IntcodeState::Halt, super::intcode::IntcodeState::Output) =
                (computers[0].state, computers[4].state)
            {
                let x = computers[4].consume_output();
                if x > max {
                    max = x;
                }
                break;
            }
        }
    });
    format!("{}", max)
}
pub fn test() {
    run(&vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ]);
    run(&vec![
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ]);
    run(&vec![
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ]);
}

pub fn main(input: &Vec<String>) -> String {
    let mem: Vec<i64> = input
        .join("")
        .split(|c| c == ',')
        .map(|x| x.parse().unwrap())
        .collect();
    run(&mem)
}

pub fn test2() {
    run2(&vec![
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ]);
    run2(&vec![
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ]);
}
pub fn main2(input: &Vec<String>) -> String {
    let mem: Vec<i64> = input
        .join("")
        .split(|c| c == ',')
        .map(|x| x.parse().unwrap())
        .collect();
    run2(&mem)
}
