use super::intcode;
pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        9
    }
    fn main1(input: &Vec<String>) -> String {
        let mem: Vec<i64> = input
            .join("")
            .split(|c| c == ',')
            .map(|x| x.parse().unwrap())
            .collect();
        let mut w = 0;
        intcode::run_intcode_with_inputs_and_cb_outputs(mem, &vec![1], |x| w = x);
        format!("{}", w)
    }
    fn main2(input: &Vec<String>) -> String {
        let mem: Vec<i64> = input
            .join("")
            .split(|c| c == ',')
            .map(|x| x.parse().unwrap())
            .collect();
        let mut w = 0;
        intcode::run_intcode_with_inputs_and_cb_outputs(mem, &vec![2], |x| w = x);
        format!("{}", w)
    }
}

pub fn test1() {
    intcode::run_intcode_with_inputs_and_cb_outputs(
        vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ],
        &Vec::<i64>::new(),
        |x| println!("{}", x),
    );

    intcode::run_intcode_with_inputs_and_cb_outputs(
        vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0],
        &Vec::<i64>::new(),
        |x| println!("{}", x),
    );

    intcode::run_intcode_with_inputs_and_cb_outputs(
        vec![104, 1125899906842624, 99],
        &Vec::<i64>::new(),
        |x| println!("{}", x),
    );
}
