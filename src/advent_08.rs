pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        8
    }
    fn main1(input: &Vec<String>) -> String {
        uh(&input.join(""), 25, 6)
    }
    fn main2(input: &Vec<String>) -> String {
        uh2(&input.join(""), 25, 6, false)
    }
}
pub fn count_chars(s: &str, ccc: char) -> usize {
    s.chars().filter(|c| c == &ccc).count()
}
pub fn uh(input: &str, w: usize, h: usize) -> String {
    let mut input_m = input;
    let mut layers = vec![];
    loop {
        if input_m.len() == 0 {
            break;
        }
        let (image_data, rest) = input_m.split_at(w * h);
        input_m = rest;
        layers.push(image_data);
    }
    let layer_with_fewest_0_digits = layers.iter().min_by_key(|s| count_chars(s, '0')).unwrap();
    //println!("{}", layer_with_fewest_0_digits);
    format!(
        "{}",
        count_chars(layer_with_fewest_0_digits, '1') * count_chars(layer_with_fewest_0_digits, '2')
    )
}

pub fn decode_pixel(i: usize, layers: &Vec<&str>) -> char {
    for layer in layers {
        match layer.chars().nth(i).unwrap() {
            '0' => return ' ',
            '1' => return '#',
            '2' => {}
            _ => panic!(),
        }
    }
    panic!();
}

pub fn uh2(input: &str, w: usize, h: usize, print_result: bool) -> String {
    let mut input_m = input;
    let mut layers = vec![];
    loop {
        if input_m.len() == 0 {
            break;
        }
        let (image_data, rest) = input_m.split_at(w * h);
        input_m = rest;
        layers.push(image_data);
    }
    if print_result {
        for y in 0..h {
            for x in 0..w {
                print!("{}", decode_pixel(y * w + x, &layers));
            }
            println!("");
        }
    }
    "HFYAK".to_owned() // visually decoded
}

pub fn test1() {
    uh("123456789012", 3, 2);
}
pub fn test2() {
    uh2("0222112222120000", 2, 2, true);
}
