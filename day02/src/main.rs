use std::io;

#[derive(PartialEq)]
enum Opcode {
    Add = 1,
    Mul = 2,
    Halt = 99,
}

impl Opcode {
    fn from_usize(value: usize) -> Self {
        match value {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            99 => Opcode::Halt,
            _ => panic!("Unknown opcode"),
        }
    }
}

fn run_until_halt(input: &mut [usize]) {
    let mut ip = 0;

    loop {
        let oc: Opcode = Opcode::from_usize(input[ip]);
        if oc == Opcode::Halt {
            break;
        }
        let op1_pos = input[ip + 1];
        let op2_pos = input[ip + 2];
        let out_pos = input[ip + 3];

        match oc {
            Opcode::Add => input[out_pos] = input[op1_pos] + input[op2_pos],
            Opcode::Mul => input[out_pos] = input[op1_pos] * input[op2_pos],
            Opcode::Halt => break,
        };

        ip += 4;
    }
}

fn solve_p1(input: Vec<usize>) -> usize {
    let mut memory = input;
    memory[1] = 12;
    memory[2] = 2;

    run_until_halt(&mut memory);

    memory[0]
}

fn solve_p2(input: Vec<usize>) -> usize {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = input.clone();
            memory[1] = noun;
            memory[2] = verb;
            run_until_halt(&mut memory);
            if memory[0] == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input: Vec<usize> = input
        .trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    println!("p1: {}", solve_p1(input.clone()));
    println!("p2: {}", solve_p2(input.clone()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_until_halt() {
        let mut v = vec![1, 0, 0, 0, 99];
        run_until_halt(&mut v);
        assert_eq!(v, [2, 0, 0, 0, 99]);

        v = vec![2, 3, 0, 3, 99];
        run_until_halt(&mut v);
        assert_eq!(v, [2, 3, 0, 6, 99]);

        v = vec![2, 4, 4, 5, 99, 0];
        run_until_halt(&mut v);
        assert_eq!(v, [2, 4, 4, 5, 99, 9801]);

        v = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run_until_halt(&mut v);
        assert_eq!(v, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
