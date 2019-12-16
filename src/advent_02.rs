pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        2
    }
    fn main1(input: &Vec<String>) -> String {
        let mut mem: Vec<usize> = input
            .join("")
            .split(|c| c == ',')
            .map(|x| x.parse().unwrap())
            .collect();

        mem[1] = 12;
        mem[2] = 2;
        //println!("0 {:?}", mem);

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
            //println!("{} {:?}", pc, mem);
        }

        format!("{:?}", mem[0])
    }
    fn main2(input: &Vec<String>) -> String {
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
                    return format!("{}", noun * 100 + verb);
                }
            }
        }
        panic!();
    }
}
