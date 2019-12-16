pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        5
    }
    fn main1(input: &Vec<String>) -> String {
        let initial_mem: Vec<i64> = input
            .join("")
            .split(|c| c == ',')
            .map(|x| x.parse().unwrap())
            .collect();

        let mut w = 0;
        exec(initial_mem, || 1, |x| w = x);
        format!("{:?}", w)
    }
    fn main2(input: &Vec<String>) -> String {
        let mut initial_mem: Vec<i64> = input
            .join("")
            .split(|c| c == ',')
            .map(|x| x.parse().unwrap())
            .collect();
        initial_mem.push(0);
        initial_mem.push(0);
        initial_mem.push(0);
        initial_mem.push(0);
        initial_mem.push(0);
        initial_mem.push(0);
        initial_mem.push(0);
        let mut w = 0;
        exec(initial_mem, || 5, |x| w = x);
        format!("{:?}", w)
    }
}

pub fn get_value(mem: &Vec<i64>, instr: &str, instr_number: usize, pointer_or_value: i64) -> i64 {
    match instr.chars().nth(instr.len() - 2 - instr_number).unwrap() {
        '0' => mem[pointer_or_value as usize],
        '1' => pointer_or_value,
        _ => panic!("no"),
    }
}

pub fn exec<F, G>(initial_mem: Vec<i64>, input: F, mut output: G) -> Vec<i64>
where
    F: Fn() -> i64,
    G: FnMut(i64),
{
    let mut mem = initial_mem.clone();
    let mut pc = 0;
    let mut running = true;
    while running {
        let instr = format!("{:09}", mem[pc]);
        let output_pos = mem[pc + 3] as usize;
        match &instr[instr.len() - 2..instr.len()] {
            "01" => {
                mem[output_pos] = get_value(&mem, &instr, 1, mem[pc + 1])
                    + get_value(&mem, &instr, 2, mem[pc + 2]);
                pc += 4;
            }
            "02" => {
                mem[output_pos] = get_value(&mem, &instr, 1, mem[pc + 1])
                    * get_value(&mem, &instr, 2, mem[pc + 2]);
                pc += 4;
            }
            "03" => {
                mem[output_pos] = input();
                pc += 2;
            }
            "04" => {
                output(get_value(&mem, &instr, 1, mem[pc + 1]));
                pc += 2;
            }
            "05" => {
                let a = get_value(&mem, &instr, 1, mem[pc + 1]);
                let b = get_value(&mem, &instr, 2, mem[pc + 2]);
                if a != 0 {
                    pc = b as usize;
                } else {
                    pc += 3;
                }
            }
            "06" => {
                let a = get_value(&mem, &instr, 1, mem[pc + 1]);
                let b = get_value(&mem, &instr, 2, mem[pc + 2]);
                if a == 0 {
                    pc = b as usize;
                } else {
                    pc += 3;
                }
            }
            "07" => {
                if get_value(&mem, &instr, 1, mem[pc + 1]) < get_value(&mem, &instr, 2, mem[pc + 2])
                {
                    mem[output_pos] = 1;
                } else {
                    mem[output_pos] = 0;
                }
                pc += 4;
            }
            "08" => {
                if get_value(&mem, &instr, 1, mem[pc + 1])
                    == get_value(&mem, &instr, 2, mem[pc + 2])
                {
                    mem[output_pos] = 1;
                } else {
                    mem[output_pos] = 0;
                }
                pc += 4;
            }
            "99" => running = false,
            _ => panic!("no."),
        }
    }
    mem
}

pub fn test(input: String) {
    let mut initial_mem: Vec<i64> = input
        .split(|c| c == ',')
        .map(|x| x.parse().unwrap())
        .collect();
    initial_mem.push(0);

    let mem = exec(initial_mem, || 8, |x| println!("output {}", x));
    println!("{:?}", mem);
}
