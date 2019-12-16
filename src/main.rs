#![allow(dead_code)]

use std;

mod common;
mod intcode;
use common::read_input;

mod advent_01;
mod advent_02;
mod advent_03;
mod advent_04;
mod advent_05;
mod advent_06;
mod advent_07;
mod advent_08;
mod advent_09;
mod advent_10;
mod advent_11;
mod advent_12;
mod advent_13;
mod advent_14;
mod advent_15;
mod advent_16;

fn run_advent<A>(input: Vec<String>, expected1: &str, expected2: &str)
where
    A: common::Advent,
{
    let before = std::time::Instant::now();
    let result1 = A::main1(&input);
    let middle = std::time::Instant::now();
    let result2 = A::main2(&input);
    let after = std::time::Instant::now();
    let dt1 = (middle - before).as_micros() as f64 / 1000.0;
    let dt2 = (after - middle).as_micros() as f64 / 1000.0;
    if result1 != expected1 {
        println!("part1 failed! Expected {} was {}", expected1, result1);
    }
    if result2 != expected2 {
        println!("part2 failed! Expected {} was {}", expected2, result2);
    }
    println!(
        "Advent {}: part1 {} ms, part2 {} ms",
        A::advent_number(),
        dt1,
        dt2
    );
}

fn main() {
    run_advent::<advent_01::Advent>(read_input("inputs/input1"), "3226488", "4836845");
    run_advent::<advent_02::Advent>(read_input("inputs/input2"), "4023471", "8051");
    run_advent::<advent_03::Advent>(read_input("inputs/input3"), "557", "56410");
    run_advent::<advent_04::Advent>(vec![], "594", "364");
    run_advent::<advent_05::Advent>(read_input("inputs/input5"), "5182797", "12077198");
    run_advent::<advent_06::Advent>(read_input("inputs/input6"), "322508", "496");
    run_advent::<advent_07::Advent>(read_input("inputs/input7"), "22012", "4039164");
    run_advent::<advent_08::Advent>(read_input("inputs/input8"), "2048", "HFYAK");
    run_advent::<advent_09::Advent>(read_input("inputs/input9"), "3409270027", "82760");
    run_advent::<advent_10::Advent>(read_input("inputs/input10"), "292", "317");
    run_advent::<advent_11::Advent>(read_input("inputs/input11"), "1771", "HGEHJHUZ");
    run_advent::<advent_12::Advent>(read_input("inputs/input12"), "7636", "281691380235984");
    run_advent::<advent_13::Advent>(read_input("inputs/input13"), "270", "12535");
    run_advent::<advent_14::Advent>(read_input("inputs/input14"), "248794", "4906796");
    run_advent::<advent_15::Advent>(read_input("inputs/input15"), "336", "360");
    run_advent::<advent_16::Advent>(read_input("inputs/input16"), "10332447", "14288025");
}
