use aoc2019::intcode::IntCodeCpu;
use std::fs;
use std::io;

fn is_affected_by_beam(cpu: &IntCodeCpu, x: usize, y: usize) -> bool {
    let mut cpu = cpu.clone();
    cpu.input.push_back(x as i64);
    cpu.input.push_back(y as i64);
    cpu.run();
    cpu.output.pop_front() == Some(1)
}

fn num_affected_points(cpu: &IntCodeCpu, xmax: usize, ymax: usize) -> usize {
    let mut count = 0;
    for xs in 0..xmax {
        for ys in 0..ymax {
            if is_affected_by_beam(&cpu, xs, ys) {
                count += 1;
            }
        }
    }
    count
}

fn main() -> io::Result<()> {
    let code = fs::read_to_string("./input/day19.in")?;
    let cpu = IntCodeCpu::from_code(&code.trim());

    println!("p1: {}", num_affected_points(&cpu, 50, 50));
    Ok(())
}
