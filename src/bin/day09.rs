use aoc2019::intcode::IntCodeCpu;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let code = fs::read_to_string("./input/day09.txt")?;

    let mut cpu = IntCodeCpu::from_code(&code);
    cpu.input.push_back(1);
    cpu.run();
    println!("p1: {}", cpu.output.pop_back().unwrap());

    cpu = IntCodeCpu::from_code(&code);
    cpu.input.push_back(2);
    cpu.run();
    println!("p2: {}", cpu.output.pop_back().unwrap());

    Ok(())
}
