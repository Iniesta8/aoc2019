use std::collections::VecDeque;

#[derive(Clone)]
pub struct IntCodeCpu {
    ip: usize,
    rbp: usize,
    pub running: bool,
    pub input: VecDeque<i64>,
    pub output: VecDeque<i64>,
    memory: Vec<i64>,
}

enum Instruction {
    ADD { src1: i64, src2: i64, dst: i64 },
    MUL { src1: i64, src2: i64, dst: i64 },
    IN { dst: i64 },
    OUT { src: i64 },
    JNZ { cond: i64, target: i64 },
    JZ { cond: i64, target: i64 },
    LT { src1: i64, src2: i64, dst: i64 },
    EQ { src1: i64, src2: i64, dst: i64 },
    RBO { src: i64 },
    HLT,
}

enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl IntCodeCpu {
    pub fn from_code(code: &str) -> IntCodeCpu {
        IntCodeCpu {
            ip: 0,
            rbp: 0,
            running: true,
            input: VecDeque::new(),
            output: VecDeque::new(),
            memory: code
                .split(',')
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect(),
        }
    }

    // Terminates on opcode halt
    pub fn run(&mut self) {
        while self.running {
            self.step();
        }
    }

    // Terminates on available output
    pub fn run_until_output(&mut self) -> Option<i64> {
        while self.running {
            self.step();
            if let Some(out) = self.output.pop_front() {
                return Some(out);
            }
        }
        None
    }

    fn fetch(&mut self, addr: usize) -> i64 {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }
        self.memory[addr]
    }

    fn store(&mut self, addr: usize, val: i64) {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }
        self.memory[addr] = val;
    }

    fn fetch_operand(&mut self, mode: ParameterMode, immediate: i64) -> i64 {
        match mode {
            ParameterMode::Position => self.fetch(immediate as usize),
            ParameterMode::Immediate => immediate,
            ParameterMode::Relative => self.fetch((self.rbp as i64 + immediate) as usize),
        }
    }

    fn fetch_dest_addr(&self, mode: ParameterMode, immediate: i64) -> i64 {
        match mode {
            ParameterMode::Position => immediate,
            ParameterMode::Immediate => panic!("Invalid parameter mode for dest operand"),
            ParameterMode::Relative => self.rbp as i64 + immediate,
        }
    }

    fn fetch_and_decode(&mut self) -> Instruction {
        let inst = self.memory[self.ip];
        let opcode = inst % 100;
        let mode1 = match inst / 100 % 10 {
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => ParameterMode::Position,
        };
        let mode2 = match inst / 1_000 % 10 {
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => ParameterMode::Position,
        };
        let mode3 = match inst / 10_000 % 10 {
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => ParameterMode::Position,
        };
        match opcode {
            1 => Instruction::ADD {
                src1: self.fetch_operand(mode1, self.memory[self.ip + 1]),
                src2: self.fetch_operand(mode2, self.memory[self.ip + 2]),
                dst: self.fetch_dest_addr(mode3, self.memory[self.ip + 3]),
            },
            2 => Instruction::MUL {
                src1: self.fetch_operand(mode1, self.memory[self.ip + 1]),
                src2: self.fetch_operand(mode2, self.memory[self.ip + 2]),
                dst: self.fetch_dest_addr(mode3, self.memory[self.ip + 3]),
            },
            3 => Instruction::IN {
                dst: self.fetch_dest_addr(mode1, self.memory[self.ip + 1]),
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
                dst: self.fetch_dest_addr(mode3, self.memory[self.ip + 3]),
            },
            8 => Instruction::EQ {
                src1: self.fetch_operand(mode1, self.memory[self.ip + 1]),
                src2: self.fetch_operand(mode2, self.memory[self.ip + 2]),
                dst: self.fetch_dest_addr(mode3, self.memory[self.ip + 3]),
            },
            9 => Instruction::RBO {
                src: self.fetch_operand(mode1, self.memory[self.ip + 1]),
            },
            99 => Instruction::HLT,
            _ => panic!("unknown opcode ({})", opcode),
        }
    }

    fn execute(&mut self, inst: &Instruction) {
        match inst {
            Instruction::ADD { src1, src2, dst } => {
                self.store(*dst as usize, src1 + src2);
                self.ip += 4;
            }
            Instruction::MUL { src1, src2, dst } => {
                self.store(*dst as usize, src1 * src2);
                self.ip += 4;
            }
            Instruction::IN { dst } => {
                let src = self.input.pop_front().unwrap();
                self.store(*dst as usize, src);
                self.ip += 2;
            }
            Instruction::OUT { src } => {
                self.output.push_back(*src);
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
                self.store(*dst as usize, if *src1 < *src2 { 1 } else { 0 });
                self.ip += 4;
            }
            Instruction::EQ { src1, src2, dst } => {
                self.store(*dst as usize, if *src1 == *src2 { 1 } else { 0 });
                self.ip += 4;
            }
            Instruction::RBO { src } => {
                self.rbp = (self.rbp as i64 + *src) as usize;
                self.ip += 2;
            }
            Instruction::HLT => {
                self.running = false;
            }
        }
    }

    fn step(&mut self) {
        let inst = self.fetch_and_decode();
        self.execute(&inst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        cpu.input.push_back(1234);
        cpu.run();
        assert_eq!(cpu.output.pop_front(), Some(1234));
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
        fn helper(code: &str, true_example: i64, false_example: i64) {
            let mut cpu = IntCodeCpu::from_code(code);
            cpu.input.push_back(true_example);
            cpu.run();
            assert_eq!(cpu.output.pop_front(), Some(1));

            let mut cpu = IntCodeCpu::from_code(code);
            cpu.input.push_back(false_example);
            cpu.run();
            assert_eq!(cpu.output.pop_front(), Some(0));
        }

        helper("3,9,8,9,10,9,4,9,99,-1,8", 8, 7);
        helper("3,3,1108,-1,8,3,4,3,99", 8, 7);
        helper("3,9,7,9,10,9,4,9,99,-1,8", 7, 8);
        helper("3,3,1107,-1,8,3,4,3,99", 7, 8);
    }

    #[test]
    fn test_resizing() {
        // quine from day 9
        let mut cpu =
            IntCodeCpu::from_code("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        cpu.run();
        assert_eq!(cpu.output.len(), 16);
        assert_eq!(cpu.output.pop_front(), Some(109));
        assert_eq!(cpu.output.pop_front(), Some(1));
        assert_eq!(cpu.output.pop_back(), Some(99));
    }

    #[test]
    fn test_large_numbers() {
        let mut cpu = IntCodeCpu::from_code("1102,34915192,34915192,7,4,7,99,0");
        cpu.run();
        assert_eq!(cpu.output.pop_front(), Some(34_915_192 * 34_915_192));
    }

    #[test]
    fn test_large_numbers2() {
        let mut cpu = IntCodeCpu::from_code("104,1125899906842624,99");
        cpu.run();
        assert_eq!(cpu.output.pop_front(), Some(1_125_899_906_842_624));
    }
}
