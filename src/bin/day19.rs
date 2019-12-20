use aoc2019::intcode::IntCodeCpu;
use std::fs;
use std::io;

fn is_affected_by_beam(cpu: &IntCodeCpu, x: usize, y: usize) -> bool {
    let mut cpu = cpu.clone();
    cpu.input.push_back(x as i64);
    cpu.input.push_back(y as i64);
    cpu.run();
    Some(1) == cpu.output.pop_front()
}

fn num_affected_points(cpu: &IntCodeCpu, xmax: usize, ymax: usize) -> usize {
    let mut count = 0;
    for xs in 0..xmax {
        for ys in 0..ymax {
            if is_affected_by_beam(cpu, xs, ys) {
                count += 1;
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    count
}

// Returns the nearest possible location (upper left corner) of a ship_width x ship_len ship
// within the beam
fn get_ship_location(cpu: &IntCodeCpu, ship_width: usize, ship_len: usize) -> (usize, usize) {
    let mut x = 0;
    let mut ys = 4; // beam is not continuous
    loop {
        let mut within_beam = false;
        let mut xs = x;
        loop {
            if is_affected_by_beam(cpu, xs, ys) {
                if !within_beam {
                    within_beam = true;
                    x = xs;
                }
                if is_affected_by_beam(cpu, xs + ship_width - 1, ys + ship_len - 1)
                    && is_affected_by_beam(cpu, xs + ship_width - 1, ys)
                    && is_affected_by_beam(cpu, xs, ys + ship_len - 1)
                {
                    return (xs, ys);
                }
            } else if within_beam {
                break;
            }
            xs += 1;
        }
        ys += 1;
    }
}

fn main() -> io::Result<()> {
    let code = fs::read_to_string("./input/day19.in")?;
    let cpu = IntCodeCpu::from_code(&code.trim());

    println!("p1: {}", num_affected_points(&cpu, 50, 50));

    let ship_pos = get_ship_location(&cpu, 100, 100);
    println!("p2: {}", ship_pos.0 * 10000 + ship_pos.1);
    Ok(())
}
