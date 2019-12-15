use aoc2019::intcode::IntCodeCpu;
use std::collections::HashSet;
use std::fs;
use std::io;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    North = 1,
    South,
    West,
    East,
}

impl From<i64> for Direction {
    fn from(val: i64) -> Self {
        match val {
            1 => Direction::North,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::East,
            _ => panic!("unknown direction {}", val),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Status {
    HitWall = 0,
    Moved,
    Reached,
}

impl From<i64> for Status {
    fn from(val: i64) -> Self {
        match val {
            0 => Status::HitWall,
            1 => Status::Moved,
            2 => Status::Reached,
            _ => panic!("unknown status code {}", val),
        }
    }
}

struct Position {
    steps: i64,
    cpu: IntCodeCpu,
    x: i64,
    y: i64,
}

fn find_oxygen_system(cpu: IntCodeCpu) -> Position {
    let mut positions = vec![Position {
        steps: 0,
        cpu: cpu,
        x: 0,
        y: 0,
    }];

    let mut visited = HashSet::new();

    loop {
        let pos = positions.remove(0);

        for direction in 1..5 {
            let mut new_x = pos.x;
            let mut new_y = pos.y;
            match Direction::from(direction) {
                Direction::North => new_y -= 1,
                Direction::South => new_y += 1,
                Direction::West => new_x -= 1,
                Direction::East => new_x += 1,
            }

            if !visited.insert((new_x, new_y)) {
                continue;
            }

            let mut new_cpu = pos.cpu.clone();
            new_cpu.input.push_back(direction);
            let output = new_cpu.run_until_output().unwrap();
            match Status::from(output) {
                Status::HitWall => {
                    // hit the wall
                }
                Status::Moved => positions.push(Position {
                    cpu: new_cpu,
                    steps: pos.steps + 1,
                    x: new_x,
                    y: new_y,
                }),
                Status::Reached => {
                    return Position {
                        cpu: new_cpu,
                        steps: pos.steps + 1,
                        x: new_x,
                        y: new_y,
                    }
                }
            }
        }
    }
}

fn fill_with_oxygen(cpu: IntCodeCpu) -> i64 {
    let mut positions = vec![Position {
        steps: 0,
        cpu: cpu,
        x: 0,
        y: 0,
    }];

    let mut visited = HashSet::new();
    let mut steps = 0;

    while !positions.is_empty() {
        let pos = positions.remove(0);
        steps = pos.steps;

        for direction in 1..5 {
            let mut new_x = pos.x;
            let mut new_y = pos.y;
            match Direction::from(direction) {
                Direction::North => new_y -= 1,
                Direction::South => new_y += 1,
                Direction::West => new_x -= 1,
                Direction::East => new_x += 1,
            }

            if !visited.insert((new_x, new_y)) {
                continue;
            }

            let mut new_cpu = pos.cpu.clone();
            new_cpu.input.push_back(direction);
            let output = new_cpu.run_until_output().unwrap();
            match Status::from(output) {
                Status::HitWall => {
                    // hit the wall
                }
                Status::Moved | Status::Reached => positions.push(Position {
                    cpu: new_cpu,
                    steps: pos.steps + 1,
                    x: new_x,
                    y: new_y,
                }),
            }
        }
    }
    steps
}

fn main() -> io::Result<()> {
    let code = fs::read_to_string("./input/day15.in")?;
    let cpu = IntCodeCpu::from_code(&code);

    let position = find_oxygen_system(cpu);

    println!("p1: {}", position.steps);
    println!("p2: {}", fill_with_oxygen(position.cpu));
    Ok(())
}
