use aoc2019::intcode::IntCodeCpu;
use itertools::Itertools;
use std::fs;

fn run_amplifiers(cpu: &IntCodeCpu) -> i32 {
    (0..5)
        .permutations(5)
        .map(|phase_settings| {
            let mut output = 0;
            for phase_setting in phase_settings {
                let mut amplifier = cpu.clone();
                amplifier.input.push_back(phase_setting);
                amplifier.input.push_back(output);
                amplifier.run();
                output = amplifier.output.unwrap();
            }
            output
        })
        .max()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("./input/day07.txt").unwrap();
    let cpu = IntCodeCpu::from_code(&input);

    println!("p1: {}", run_amplifiers(&cpu));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_amplifiers() {
        assert_eq!(
            run_amplifiers(&IntCodeCpu::from_code(
                "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            )),
            43210
        );
    }

    #[test]
    fn test_run_amplifiers2() {
        assert_eq!(
            run_amplifiers(&IntCodeCpu::from_code(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,
                101,5,23,23,1,24,23,23,4,23,99,0,0"
            )),
            54321
        );
    }

    #[test]
    fn test_run_amplifiers3() {
        assert_eq!(
            run_amplifiers(&IntCodeCpu::from_code(
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            )),
            65210
        );
    }
}
