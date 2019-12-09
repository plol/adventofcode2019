fn get_value(mem: &Vec<i64>, instr: &str, instr_number: usize, pointer_or_value: i64) -> i64 {
    match instr.chars().nth(instr.len() - 2 - instr_number).unwrap() {
        '0' => mem[pointer_or_value as usize],
        '1' => pointer_or_value,
        _ => panic!("no"),
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IntcodeState {
    NotYetStarted,
    NeedsInput,
    Output(i64),
    Halt,
}

pub struct IntcodeComputer {
    pub pc: usize,
    pub state: IntcodeState,
    pub input_pos: usize,
    pub mem: Vec<i64>,
}

impl IntcodeComputer {
    pub fn new(initial_mem: &Vec<i64>) -> Self {
        Self {
            pc: 0,
            state: IntcodeState::NotYetStarted,
            input_pos: 0,
            mem: initial_mem.clone(),
        }
    }
    pub fn run(&mut self) {
        match self.state {
            IntcodeState::NotYetStarted => (),
            IntcodeState::Output(_) => (),
            _ => panic!(),
        };
        self.exec();
    }
    pub fn provide_input(&mut self, input: i64) {
        match self.state {
            IntcodeState::NeedsInput => (),
            _ => panic!("Needs state NeedsInput, was {:?}", self.state),
        };
        self.mem[self.input_pos] = input;
        self.exec();
    }

    pub fn current_output(&self) -> Option<i64> {
        match self.state {
            IntcodeState::Output(x) => Some(x),
            _ => None,
        }
    }

    fn exec(&mut self) {
        loop {
            let instr = format!("{:09}", self.mem[self.pc]);
            match &instr[instr.len() - 2..instr.len()] {
                "01" => {
                    let output_pos = self.mem[self.pc + 3] as usize;
                    let a = get_value(&self.mem, &instr, 1, self.mem[self.pc + 1]);
                    let b = get_value(&self.mem, &instr, 2, self.mem[self.pc + 2]);
                    let c = a + b;
                    //println!("store {} = {} + {} at {}", c, a, b, output_pos);
                    self.mem[output_pos] = c;
                    self.pc += 4;
                }
                "02" => {
                    let output_pos = self.mem[self.pc + 3] as usize;
                    let a = get_value(&self.mem, &instr, 1, self.mem[self.pc + 1]);
                    let b = get_value(&self.mem, &instr, 2, self.mem[self.pc + 2]);
                    let c = a * b;
                    //println!("store {} = {} * {} at {}", c, a, b, output_pos);
                    self.mem[output_pos] = c;
                    self.pc += 4;
                }
                "03" => {
                    self.input_pos = self.mem[self.pc + 1] as usize;
                    self.state = IntcodeState::NeedsInput;
                    self.pc += 2;
                    return;
                }
                "04" => {
                    let a = get_value(&self.mem, &instr, 1, self.mem[self.pc + 1]);
                    self.pc += 2;
                    self.state = IntcodeState::Output(a);
                    return;
                }
                "05" => {
                    let a = get_value(&self.mem, &instr, 1, self.mem[self.pc + 1]);
                    let b = get_value(&self.mem, &instr, 2, self.mem[self.pc + 2]);
                    if a != 0 {
                        self.pc = b as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                "06" => {
                    let a = get_value(&self.mem, &instr, 1, self.mem[self.pc + 1]);
                    let b = get_value(&self.mem, &instr, 2, self.mem[self.pc + 2]);
                    if a == 0 {
                        self.pc = b as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                "07" => {
                    let output_pos = self.mem[self.pc + 3] as usize;
                    if get_value(&self.mem, &instr, 1, self.mem[self.pc + 1])
                        < get_value(&self.mem, &instr, 2, self.mem[self.pc + 2])
                    {
                        self.mem[output_pos] = 1;
                    } else {
                        self.mem[output_pos] = 0;
                    }
                    self.pc += 4;
                }
                "08" => {
                    let output_pos = self.mem[self.pc + 3] as usize;
                    if get_value(&self.mem, &instr, 1, self.mem[self.pc + 1])
                        == get_value(&self.mem, &instr, 2, self.mem[self.pc + 2])
                    {
                        self.mem[output_pos] = 1;
                    } else {
                        self.mem[output_pos] = 0;
                    }
                    self.pc += 4;
                }
                "99" => {
                    self.state = IntcodeState::Halt;
                    return;
                }
                _ => panic!("no."),
            }
            //println!("{:?}", mem);
        }
    }
}
