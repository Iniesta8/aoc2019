use aoc2019::intcode::IntCodeCpu;
use std::io;

fn main() -> io::Result<()> {
    let mut code = String::new();
    io::stdin().read_line(&mut code)?;

    let mut cpu = IntCodeCpu::from_code(&code);
    cpu.input.push_back(1);

    cpu.run();

    println!("p1: {}", cpu.output.pop_back().unwrap());

    cpu = IntCodeCpu::from_code(&code);
    cpu.input.push_back(5);

    cpu.run();

    println!("p2: {}", cpu.output.pop_back().unwrap());

    Ok(())
}
