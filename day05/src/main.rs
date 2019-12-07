use day05::intcode::IntCodeCpu;
use std::io;

fn main() -> io::Result<()> {
    let mut code = String::new();
    io::stdin().read_line(&mut code)?;

    let mut cpu = IntCodeCpu::from_code(&code);
    cpu.input = Some(1);

    cpu.run();

    println!("p1: {}", cpu.output.unwrap());

    cpu = IntCodeCpu::from_code(&code);
    cpu.input = Some(5);

    cpu.run();

    println!("p2: {}", cpu.output.unwrap());

    Ok(())
}
