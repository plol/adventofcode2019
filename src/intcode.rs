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
    Output,
    Halt,
}

#[derive(Clone)]
pub struct IntcodeComputer {
    pub pc: usize,
    pub relative_base: i64,
    pub state: IntcodeState,
    pub mem: Vec<i64>,

    input_pos: usize,
    output: i64,

    pub trace: bool,
}

impl IntcodeComputer {
    pub fn new(initial_mem: Vec<i64>) -> Self {
        Self {
            pc: 0,
            relative_base: 0,
            state: IntcodeState::NotYetStarted,
            mem: initial_mem,
            input_pos: 0,
            output: 0,
            trace: false,
        }
    }
    pub fn new_from_input_lines(input: &Vec<String>) -> Self {
        Self::new(
            input
                .join("")
                .split(|c| c == ',')
                .map(|x| x.parse().unwrap())
                .collect(),
        )
    }

    pub fn start(&mut self) {
        match self.state {
            IntcodeState::NotYetStarted => (),
            _ => panic!(),
        };
        self.exec();
    }

    pub fn provide_input(&mut self, input: i64) {
        match self.state {
            IntcodeState::NeedsInput => (),
            _ => panic!("Needs state NeedsInput, was {:?}", self.state),
        };
        if self.trace {
            println!(
                "was provided input {} and stored at {}",
                input, self.input_pos
            );
        }
        self.write(self.input_pos, input);
        self.exec();
    }

    pub fn peek_output(&self) -> Option<i64> {
        match self.state {
            IntcodeState::Output => Some(self.output),
            _ => None,
        }
    }

    pub fn consume_output(&mut self) -> i64 {
        match self.state {
            IntcodeState::Output => {
                let output = self.output;
                self.exec();
                output
            }
            _ => panic!(),
        }
    }

    fn pointer_value(&self, instr: &str, param_number: usize) -> usize {
        let pointer = self.mem[self.pc + param_number];
        match instr.chars().nth(instr.len() - 2 - param_number).unwrap() {
            '0' => pointer as usize,
            '2' => (pointer + self.relative_base) as usize,
            _ => panic!("no"),
        }
    }

    fn param_value(&self, instr: &str, param_number: usize) -> i64 {
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
                    if self.trace {
                        println!("store {} = {} + {} at {}", c, a, b, output_pos);
                    }
                    self.write(output_pos, c);
                    self.pc += 4;
                }
                "02" => {
                    let a = self.param_value(&instr, 1);
                    let b = self.param_value(&instr, 2);
                    let output_pos = self.pointer_value(&instr, 3);
                    let c = a * b;
                    if self.trace {
                        println!("store {} = {} * {} at {}", c, a, b, output_pos);
                    }
                    self.write(output_pos, c);
                    self.pc += 4;
                }
                "03" => {
                    self.input_pos = self.pointer_value(&instr, 1);
                    self.state = IntcodeState::NeedsInput;
                    self.pc += 2;
                    if self.trace {
                        println!("wait for input to store at {}", self.input_pos);
                    }
                    return;
                }
                "04" => {
                    let a = self.param_value(&instr, 1);
                    self.pc += 2;
                    self.output = a;
                    self.state = IntcodeState::Output;
                    if self.trace {
                        println!("output {}, waiting for it to be read", a);
                    }
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
                        if self.trace {
                            println!("jump to {} due to {} == 0", b, a);
                        }
                        self.pc = b as usize;
                    } else {
                        if self.trace {
                            println!("does not jump to {} due to {} != 0", b, a);
                        }
                        self.pc += 3;
                    }
                }
                "07" => {
                    let a = self.param_value(&instr, 1);
                    let b = self.param_value(&instr, 2);
                    let output_pos = self.pointer_value(&instr, 3);
                    let val = if a < b { 1 } else { 0 };
                    if self.trace {
                        println!(
                            "write {} to {} due to comparison of {} < {}",
                            val, output_pos, a, b
                        );
                    }
                    self.write(output_pos, val);
                    self.pc += 4;
                }
                "08" => {
                    let a = self.param_value(&instr, 1);
                    let b = self.param_value(&instr, 2);
                    let output_pos = self.pointer_value(&instr, 3);
                    let val = if a == b { 1 } else { 0 };
                    if self.trace {
                        println!(
                            "write {} to {} due to comparison of {} == {}",
                            val, output_pos, a, b
                        );
                    }
                    self.write(output_pos, val);
                    self.pc += 4;
                }
                "09" => {
                    let a = self.param_value(&instr, 1);
                    if self.trace {
                        println!("increase relative base by {} to {}", a, self.relative_base);
                    }
                    self.relative_base += a;
                    self.pc += 2;
                }
                "99" => {
                    if self.trace {
                        println!("halt!");
                    }
                    self.state = IntcodeState::Halt;
                    return;
                }
                _ => panic!("no."),
            }
        }
    }
}

pub fn run_intcode_with_inputs_and_cb_outputs<F>(
    initial_mem: Vec<i64>,
    inputs: &Vec<i64>,
    mut cb: F,
) where
    F: FnMut(i64),
{
    let mut computer = IntcodeComputer::new(initial_mem);
    computer.start();

    let mut next_input_index = 0;
    loop {
        match computer.state {
            IntcodeState::NeedsInput => {
                computer.provide_input(inputs[next_input_index]);
                next_input_index += 1;
            }
            IntcodeState::Output => {
                cb(computer.consume_output());
            }
            IntcodeState::Halt => break,
            IntcodeState::NotYetStarted => panic!(),
        }
    }
}

pub struct ComputerOutputIterator<I>
where
    I: Iterator<Item = i64>,
{
    computer: IntcodeComputer,
    inputs: I,
    next_input_index: usize,
}

impl<I> Iterator for ComputerOutputIterator<I>
where
    I: Iterator<Item = i64>,
{
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.computer.state {
                IntcodeState::NeedsInput => {
                    self.computer.provide_input(self.inputs.next().unwrap());
                    self.next_input_index += 1;
                }
                IntcodeState::Output => {
                    return Some(self.computer.consume_output());
                }
                IntcodeState::Halt => return None,
                IntcodeState::NotYetStarted => self.computer.start(),
            }
        }
    }
}

pub fn run_intcode_with_inputs_and_iterate_over_outputs<I>(
    initial_mem: Vec<i64>,
    inputs: I,
) -> ComputerOutputIterator<I>
where
    I: Iterator<Item = i64>,
{
    ComputerOutputIterator {
        computer: IntcodeComputer::new(initial_mem),
        inputs: inputs,
        next_input_index: 0,
    }
}
