#[derive(Clone)]
pub struct IntCodeCpu {
    ip: usize,
    running: bool,
    pub input: Option<i32>,
    pub output: Option<i32>,
    memory: Vec<i32>,
}

enum Instruction {
    ADD { src1: i32, src2: i32, dst: i32 },
    MUL { src1: i32, src2: i32, dst: i32 },
    IN { dst: i32 },
    OUT { src: i32 },
    JNZ { cond: i32, target: i32 },
    JZ { cond: i32, target: i32 },
    LT { src1: i32, src2: i32, dst: i32 },
    EQ { src1: i32, src2: i32, dst: i32 },
    HLT,
}

enum ParameterMode {
    Position,
    Immediate,
}

impl IntCodeCpu {
    pub fn from_code(code: &str) -> IntCodeCpu {
        IntCodeCpu {
            ip: 0,
            running: true,
            input: None,
            output: None,
            memory: code
                .split(',')
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect(),
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.step();
        }
    }

    fn fetch_operand(&self, mode: ParameterMode, immediate: i32) -> i32 {
        match mode {
            ParameterMode::Position => self.memory[immediate as usize],
            ParameterMode::Immediate => immediate,
        }
    }

    fn fetch_and_decode(&self) -> Instruction {
        let inst = self.memory[self.ip];
        let opcode = inst % 100;
        let mode1 = match inst / 100 % 10 {
            1 => ParameterMode::Immediate,
            _ => ParameterMode::Position,
        };
        let mode2 = match inst / 1_000 % 10 {
            1 => ParameterMode::Immediate,
            _ => ParameterMode::Position,
        };
        match opcode {
            1 => Instruction::ADD {
                src1: self.fetch_operand(mode1, self.memory[self.ip + 1]),
                src2: self.fetch_operand(mode2, self.memory[self.ip + 2]),
                dst: self.memory[self.ip + 3],
            },
            2 => Instruction::MUL {
                src1: self.fetch_operand(mode1, self.memory[self.ip + 1]),
                src2: self.fetch_operand(mode2, self.memory[self.ip + 2]),
                dst: self.memory[self.ip + 3],
            },
            3 => Instruction::IN {
                dst: self.memory[self.ip + 1],
            },
            4 => Instruction::OUT {
                src: self.fetch_operand(mode1, self.memory[self.ip + 1]),
            },
            5 => Instruction::JNZ {
                cond: self.fetch_operand(mode1, self.memory[self.ip + 1]),
                target: self.fetch_operand(mode2, self.memory[self.ip + 2]),
            },
            6 => Instruction::JZ {
                cond: self.fetch_operand(mode1, self.memory[self.ip + 1]),
                target: self.fetch_operand(mode2, self.memory[self.ip + 2]),
            },
            7 => Instruction::LT {
                src1: self.fetch_operand(mode1, self.memory[self.ip + 1]),
                src2: self.fetch_operand(mode2, self.memory[self.ip + 2]),
                dst: self.memory[self.ip + 3],
            },
            8 => Instruction::EQ {
                src1: self.fetch_operand(mode1, self.memory[self.ip + 1]),
                src2: self.fetch_operand(mode2, self.memory[self.ip + 2]),
                dst: self.memory[self.ip + 3],
            },
            99 => Instruction::HLT,
            _ => panic!("unknown opcode ({})", opcode),
        }
    }

    fn execute(&mut self, inst: &Instruction) {
        match inst {
            Instruction::ADD { src1, src2, dst } => {
                self.memory[*dst as usize] = src1 + src2;
                self.ip += 4;
            }
            Instruction::MUL { src1, src2, dst } => {
                self.memory[*dst as usize] = src1 * src2;
                self.ip += 4;
            }
            Instruction::IN { dst } => {
                self.memory[*dst as usize] = self.input.unwrap();
                self.ip += 2;
            }
            Instruction::OUT { src } => {
                self.output = Some(*src);
                self.ip += 2;
            }
            Instruction::JNZ { cond, target } => {
                if *cond != 0 {
                    self.ip = *target as usize;
                } else {
                    self.ip += 3;
                }
            }
            Instruction::JZ { cond, target } => {
                if *cond == 0 {
                    self.ip = *target as usize;
                } else {
                    self.ip += 3;
                }
            }
            Instruction::LT { src1, src2, dst } => {
                self.memory[*dst as usize] = if *src1 < *src2 { 1 } else { 0 };
                self.ip += 4;
            }
            Instruction::EQ { src1, src2, dst } => {
                self.memory[*dst as usize] = if *src1 == *src2 { 1 } else { 0 };
                self.ip += 4;
            }
            Instruction::HLT => {
                self.running = false;
            }
        }
    }

    fn step(&mut self) {
        self.execute(&self.fetch_and_decode());
    }
}

#[test]
fn test_step_add_mul() {
    let mut cpu = IntCodeCpu::from_code("1,4,5,6,10,20,0");
    cpu.step();
    assert_eq!(cpu.ip, 4);
    assert_eq!(cpu.memory, vec![1, 4, 5, 6, 10, 20, 30]);
    assert!(cpu.running);
    cpu.ip = 0;
    cpu.memory[0] = 2;
    cpu.step();
    assert_eq!(cpu.ip, 4);
    assert_eq!(cpu.memory, vec![2, 4, 5, 6, 10, 20, 200]);
    assert!(cpu.running);
}

#[test]
fn test_run() {
    let mut cpu = IntCodeCpu::from_code("1,9,10,3,2,3,11,0,99,30,40,50");
    cpu.run();
    assert!(!cpu.running);
    assert_eq!(
        cpu.memory,
        vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    )
}

#[test]
fn test_io() {
    let mut cpu = IntCodeCpu::from_code("3,0,4,0,99");
    cpu.input = Some(1234);
    cpu.run();
    assert_eq!(cpu.output, Some(1234));
}

#[test]
fn test_parameter_modes() {
    let mut cpu = IntCodeCpu::from_code("1002,4,3,4,33");
    cpu.run();
    assert_eq!(cpu.memory[4], 99);
    cpu = IntCodeCpu::from_code("1101,100,-1,4,0");
    cpu.run();
    assert_eq!(cpu.memory[4], 99);
}

#[test]
fn test_conditions() {
    fn helper(code: &str, true_example: i32, false_example: i32) {
        let mut cpu = IntCodeCpu::from_code(code);
        cpu.input = Some(true_example);
        cpu.run();
        assert_eq!(cpu.output, Some(1));

        let mut cpu = IntCodeCpu::from_code(code);
        cpu.input = Some(false_example);
        cpu.run();
        assert_eq!(cpu.output, Some(0));
    }

    helper("3,9,8,9,10,9,4,9,99,-1,8", 8, 7);
    helper("3,3,1108,-1,8,3,4,3,99", 8, 7);
    helper("3,9,7,9,10,9,4,9,99,-1,8", 7, 8);
    helper("3,3,1107,-1,8,3,4,3,99", 7, 8);
}
