use aoc2019::intcode::IntCodeCpu;
use itertools::Itertools;
use std::fs;

fn run_amplifiers(cpu: &IntCodeCpu) -> i64 {
    (0..5)
        .permutations(5)
        .map(|phase_settings| {
            let mut output = 0;
            for phase_setting in phase_settings {
                let mut amplifier = cpu.clone();
                amplifier.input.push_back(phase_setting);
                amplifier.input.push_back(output);
                amplifier.run();
                output = amplifier.output.pop_front().unwrap();
            }
            output
        })
        .max()
        .unwrap()
}

fn run_amplifiers_with_feedback(cpu: &IntCodeCpu) -> i64 {
    (5..10)
        .permutations(5)
        .map(|phase_settings| {
            let mut amplifiers: Vec<IntCodeCpu> = phase_settings
                .iter()
                .map(|phase_setting| {
                    let mut amplifier = cpu.clone();
                    amplifier.input.push_back(*phase_setting);
                    amplifier.set_running();
                    amplifier
                })
                .collect();

            let mut output = 0;
            while amplifiers.first().unwrap().running() {
                for amplifier in &mut amplifiers {
                    amplifier.input.push_back(output);
                    if let Some(out) = amplifier.run_until_output() {
                        output = out;
                    } else {
                        break;
                    }
                }
            }
            output
        })
        .max()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("./input/day07.in").unwrap();
    let cpu = IntCodeCpu::from_code(&input);

    println!("p1: {}", run_amplifiers(&cpu));
    println!("p2: {}", run_amplifiers_with_feedback(&cpu));
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
            43_210
        );
    }

    #[test]
    fn test_run_amplifiers2() {
        assert_eq!(
            run_amplifiers(&IntCodeCpu::from_code(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,
                101,5,23,23,1,24,23,23,4,23,99,0,0"
            )),
            54_321
        );
    }

    #[test]
    fn test_run_amplifiers3() {
        assert_eq!(
            run_amplifiers(&IntCodeCpu::from_code(
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            )),
            65_210
        );
    }

    #[test]
    fn test_run_amplifiers_with_feedback() {
        assert_eq!(
            run_amplifiers_with_feedback(&IntCodeCpu::from_code(
                "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
            )),
            139_629_729
        );
    }

    #[test]
    fn test_run_amplifiers_with_feedback2() {
        assert_eq!(
            run_amplifiers_with_feedback(&IntCodeCpu::from_code(
                "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            )),
            18_216
        );
    }
}
