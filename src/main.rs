#![allow(dead_code)]

use std;

fn read_input(filename: &str) -> Vec<String> {
    let file = std::fs::File::open(filename).unwrap();
    let buf = std::io::BufReader::new(file);
    std::io::BufRead::lines(buf)
        .map(|line| line.unwrap())
        .collect()
}

fn advent_01_1(input: Vec<String>) {
    println!(
        "{}",
        input
            .iter()
            .map(|s| s.parse().unwrap())
            .map(|x: i32| x / 3 - 2)
            .sum::<i32>()
    );
}

fn advent_01_2(input: Vec<String>) {
    let mut result: i32 = 0;
    for x in input.iter().map(|s| s.parse::<i32>().unwrap()) {
        let mut j = x / 3 - 2;
        while j > 0 {
            result += j;
            j = j / 3 - 2;
        }
    }
    println!("{}", result);
}

fn advent_02_1(input: Vec<String>) {
    let mut mem: Vec<usize> = input
        .join("")
        .split(|c| c == ',')
        .map(|x| x.parse().unwrap())
        .collect();

    mem[1] = 12;
    mem[2] = 2;
    println!("0 {:?}", mem);

    let mut pc = 0;
    let mut running = true;
    while running {
        let instr = mem[pc];
        let output_pos = mem[pc + 3];
        match instr {
            1 => mem[output_pos] = mem[mem[pc + 1]] + mem[mem[pc + 2]],
            2 => mem[output_pos] = mem[mem[pc + 1]] * mem[mem[pc + 2]],
            99 => running = false,
            _ => panic!("no."),
        }
        pc += 4;
        println!("{} {:?}", pc, mem);
    }

    println!("{:?}", mem);
}

fn advent_02_2(input: Vec<String>) {
    let initial_mem: Vec<usize> = input
        .join("")
        .split(|c| c == ',')
        .map(|x| x.parse().unwrap())
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = initial_mem.clone();

            mem[1] = noun;
            mem[2] = verb;

            let mut pc = 0;
            let mut running = true;
            while running {
                let instr = mem[pc];
                let output_pos = mem[pc + 3];
                match instr {
                    1 => mem[output_pos] = mem[mem[pc + 1]] + mem[mem[pc + 2]],
                    2 => mem[output_pos] = mem[mem[pc + 1]] * mem[mem[pc + 2]],
                    99 => running = false,
                    _ => panic!("no."),
                }
                pc += 4;
            }
            if mem[0] == 19690720 {
                println!("{}", noun * 100 + verb);
                return;
            }
        }
    }
}

fn main() {
    advent_02_2(read_input("inputs/input2"));
}
