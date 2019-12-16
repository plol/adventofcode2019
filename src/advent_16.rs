pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        16
    }

    fn main1(input: &Vec<String>) -> String {
        format!(
            "{}",
            fft2100(&parse(input), 0)
                .iter()
                .take(8)
                .map(|b| ('0' as i8 + b) as u8 as char)
                .collect::<String>()
        )
    }

    fn main2(input: &Vec<String>) -> String {
        let data = core::iter::repeat(parse(input))
            .take(10000)
            .flat_map(|x| x)
            .collect::<Vec<i8>>();
        let s = input[0][0..7].to_owned().parse::<usize>().unwrap();
        let result = fft2100(&data, s);
        format!(
            "{}",
            &result[s..s + 8]
                .iter()
                .map(|x| x.to_string())
                .collect::<String>()
        )
    }
}

static PATTERN_BASE: [i16; 4] = [0, 1, 0, -1];

fn fft2_step(result: &mut [i8], input: &[i8], skip: usize) {
    let len = input.len();
    for i in skip..len / 2 {
        let mut sum: i16 = 0;
        for j in i..input.len() {
            let x = input[j] as i16;
            let p = PATTERN_BASE[((j + 1) / (i + 1)) % 4];
            sum += x * p;
        }
        result[i] = (sum.abs() % 10) as i8;
    }
    result[len - 1] = input[len - 1];
    for i in (len / 2..len - 1).rev() {
        result[i] = (input[i] + result[i + 1]) % 10;
    }
}

fn fft2100(input: &Vec<i8>, skip: usize) -> Vec<i8> {
    let mut a = input.clone();
    let mut b = input.clone();
    for _ in 0..50 {
        fft2_step(&mut b, &a, skip);
        fft2_step(&mut a, &b, skip);
    }
    a
}

fn parse(input: &Vec<String>) -> Vec<i8> {
    input
        .join("")
        .chars()
        .map(|c| (c as i8) - '0' as i8)
        .collect::<Vec<i8>>()
}

pub fn test1() {
    println!(
        "{}",
        fft2100(
            &parse(&vec!["80871224585914546619083218645595".to_owned()]),
            0
        )
        .iter()
        .take(8)
        .map(|b| ('0' as i8 + b) as u8 as char)
        .collect::<String>(),
    );
    println!(
        "{}",
        fft2100(
            &parse(&vec!["19617804207202209144916044189917".to_owned()]),
            0
        )
        .iter()
        .take(8)
        .map(|b| ('0' as i8 + b) as u8 as char)
        .collect::<String>(),
    );
    println!(
        "{}",
        fft2100(
            &parse(&vec!["69317163492948606335995924319873".to_owned()]),
            0
        )
        .iter()
        .take(8)
        .map(|b| ('0' as i8 + b) as u8 as char)
        .collect::<String>(),
    );
}

pub fn test2() {
    //    let mut input = parse(&vec!["41231232".to_owned()]);
    //    let mut fft_step_result = input.clone();
    //    fft_step(&mut fft_step_result, &input);
    //
    //    let mut fft2_step_result = input.clone();
    //    fft2_step(&mut fft2_step_result, &input);
    //
    //    println!("{}: {:?} != {:?}", 0, fft_step_result, fft2_step_result,);
}
