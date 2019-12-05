enum Opcode {
    ADD = 1,
    MUL = 2,
    HALT = 99,
}

impl Opcode {
    fn from_usize(value: usize) -> Opcode {
        match value {
            1 => Opcode::ADD,
            2 => Opcode::MUL,
            99 => Opcode::HALT,
            _ => panic!("Unknown opcode"),
        }
    }
}

const INPUT: [usize; 145] = [
    1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 19, 9, 23, 1, 23, 6, 27, 1, 9,
    27, 31, 1, 31, 10, 35, 2, 13, 35, 39, 1, 39, 10, 43, 1, 43, 9, 47, 1, 47, 13, 51, 1, 51, 13,
    55, 2, 55, 6, 59, 1, 59, 5, 63, 2, 10, 63, 67, 1, 67, 9, 71, 1, 71, 13, 75, 1, 6, 75, 79, 1,
    10, 79, 83, 2, 9, 83, 87, 1, 87, 5, 91, 2, 91, 9, 95, 1, 6, 95, 99, 1, 99, 5, 103, 2, 103, 10,
    107, 1, 107, 6, 111, 2, 9, 111, 115, 2, 9, 115, 119, 2, 13, 119, 123, 1, 123, 9, 127, 1, 5,
    127, 131, 1, 131, 2, 135, 1, 135, 6, 0, 99, 2, 0, 14, 0,
];

fn run_until_halt(input: &mut [usize]) {
    let mut ip = 0;

    loop {
        let oc: Opcode = Opcode::from_usize(input[ip]);
        let op1_pos = input[ip + 1];
        let op2_pos = input[ip + 2];
        let out_pos = input[ip + 3];

        match oc {
            Opcode::ADD => input[out_pos] = input[op1_pos] + input[op2_pos],
            Opcode::MUL => input[out_pos] = input[op1_pos] * input[op2_pos],
            Opcode::HALT => break,
        };

        ip += 4;
    }
}

fn solve_p1() -> usize {
    let mut input = INPUT;

    input[1] = 12;
    input[2] = 2;

    run_until_halt(&mut input);

    input[0]
}

fn solve_p2() -> usize {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut input = INPUT;
            input[1] = noun;
            input[2] = verb;
            run_until_halt(&mut input);
            if input[0] == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

fn main() {
    println!("p1: {}", solve_p1());
    println!("p2: {}", solve_p2());
}
