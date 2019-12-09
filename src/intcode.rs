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
    pub relative_base: i64,
    pub state: IntcodeState,
    pub input_pos: usize,
    pub mem: Vec<i64>,
}

impl IntcodeComputer {
    pub fn new(initial_mem: &Vec<i64>) -> Self {
        Self {
            pc: 0,
            relative_base: 0,
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
        self.write(self.input_pos, input);
        self.exec();
    }

    pub fn current_output(&self) -> Option<i64> {
        match self.state {
            IntcodeState::Output(x) => Some(x),
            _ => None,
        }
    }

    fn pointer_value(&self, instr: &str, param_number: usize) -> usize {
        //fn get_value(mem: &Vec<i64>, instr: &str, instr_number: usize, pointer_or_value: i64) -> i64 {
        let pointer = self.mem[self.pc + param_number];
        match instr.chars().nth(instr.len() - 2 - param_number).unwrap() {
            '0' => pointer as usize,
            '2' => (pointer + self.relative_base) as usize,
            _ => panic!("no"),
        }
    }
    fn param_value(&self, instr: &str, param_number: usize) -> i64 {
        //fn get_value(mem: &Vec<i64>, instr: &str, instr_number: usize, pointer_or_value: i64) -> i64 {
        let pointer_or_value = self.mem[self.pc + param_number];
        match instr.chars().nth(instr.len() - 2 - param_number).unwrap() {
            '0' => self.read(pointer_or_value as usize),
            '1' => pointer_or_value,
            '2' => self.read((pointer_or_value + self.relative_base) as usize),
            _ => panic!("no"),
        }
    }

    fn read(&self, location: usize) -> i64 {
        if location < self.mem.len() {
            self.mem[location]
        } else {
            0
        }
    }
    fn write(&mut self, location: usize, value: i64) {
        while location >= self.mem.len() {
            self.mem.push(0);
        }
        self.mem[location] = value;
    }

    fn exec(&mut self) {
        loop {
            let instr = format!("{:09}", self.mem[self.pc]);
            match &instr[instr.len() - 2..instr.len()] {
                "01" => {
                    let a = self.param_value(&instr, 1);
                    let b = self.param_value(&instr, 2);
                    let output_pos = self.pointer_value(&instr, 3);
                    let c = a + b;
                    //println!("store {} = {} + {} at {}", c, a, b, output_pos);
                    self.write(output_pos, c);
                    self.pc += 4;
                }
                "02" => {
                    let a = self.param_value(&instr, 1);
                    let b = self.param_value(&instr, 2);
                    let output_pos = self.pointer_value(&instr, 3);
                    let c = a * b;
                    //println!("store {} = {} * {} at {}", c, a, b, output_pos);
                    self.write(output_pos, c);
                    self.pc += 4;
                }
                "03" => {
                    self.input_pos = self.pointer_value(&instr, 1);
                    self.state = IntcodeState::NeedsInput;
                    self.pc += 2;
                    return;
                }
                "04" => {
                    let a = self.param_value(&instr, 1);
                    self.pc += 2;
                    self.state = IntcodeState::Output(a);
                    return;
                }
                "05" => {
                    let a = self.param_value(&instr, 1);
                    let b = self.param_value(&instr, 2);
                    if a != 0 {
                        self.pc = b as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                "06" => {
                    let a = self.param_value(&instr, 1);
                    let b = self.param_value(&instr, 2);
                    if a == 0 {
                        self.pc = b as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                "07" => {
                    let a = self.param_value(&instr, 1);
                    let b = self.param_value(&instr, 2);
                    let output_pos = self.pointer_value(&instr, 3);
                    self.write(output_pos, if a < b { 1 } else { 0 });
                    self.pc += 4;
                }
                "08" => {
                    let a = self.param_value(&instr, 1);
                    let b = self.param_value(&instr, 2);
                    let output_pos = self.pointer_value(&instr, 3);
                    self.write(output_pos, if a == b { 1 } else { 0 });
                    self.pc += 4;
                }
                "09" => {
                    let a = self.param_value(&instr, 1);
                    self.relative_base += a;
                    self.pc += 2;
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

pub fn run_intcode_with_inputs_and_print_outputs(initial_mem: &Vec<i64>, inputs: &Vec<i64>) {
    let mut computer = IntcodeComputer::new(initial_mem);
    computer.run();

    let mut next_input_index = 0;
    loop {
        match computer.state {
            IntcodeState::NeedsInput => {
                computer.provide_input(inputs[next_input_index]);
                next_input_index += 1;
            }
            IntcodeState::Output(x) => {
                println!("{}", x);
                computer.run();
            }
            IntcodeState::Halt => break,
            IntcodeState::NotYetStarted => panic!(),
        }
    }
}
