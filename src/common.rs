pub trait Advent {
    fn advent_number() -> u8;
    fn main1(input: &Vec<String>) -> String;
    fn main2(input: &Vec<String>) -> String;
}

pub fn read_input(filename: &str) -> Vec<String> {
    let file = std::fs::File::open(filename).unwrap();
    let buf = std::io::BufReader::new(file);
    std::io::BufRead::lines(buf)
        .map(|line| line.unwrap())
        .collect()
}
