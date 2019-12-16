pub struct Advent;

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        1
    }
    fn main1(input: &Vec<String>) -> String {
        format!(
            "{}",
            input
                .iter()
                .map(|s| s.parse().unwrap())
                .map(|x: i32| x / 3 - 2)
                .sum::<i32>()
        )
    }

    fn main2(input: &Vec<String>) -> String {
        let mut result: i32 = 0;
        for x in input.iter().map(|s| s.parse::<i32>().unwrap()) {
            let mut j = x / 3 - 2;
            while j > 0 {
                result += j;
                j = j / 3 - 2;
            }
        }
        format!("{}", result)
    }
}
