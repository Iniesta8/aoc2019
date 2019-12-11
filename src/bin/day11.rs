use aoc2019::intcode::IntCodeCpu;
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Debug)]
enum Color {
    Black = 0,
    White = 1,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    direction: Direction,
    visited_positions: HashMap<(i32, i32), Color>,
}

impl Robot {
    fn new() -> Self {
        let mut robot = Robot {
            position: (0, 0),
            direction: Direction::Up,
            visited_positions: HashMap::new(),
        };
        robot.visit((0, 0));
        robot
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.position.1 += 1,
            Direction::Right => self.position.0 += 1,
            Direction::Down => self.position.1 -= 1,
            Direction::Left => self.position.0 -= 1,
        }
        self.visit(self.position);
    }

    fn visit(&mut self, pos: (i32, i32)) {
        self.visited_positions.entry(pos).or_insert(Color::Black);
    }

    fn paint(&mut self, color: Color) {
        self.visited_positions.insert(self.position, color);
    }
}

fn main() -> io::Result<()> {
    let code = fs::read_to_string("./input/day11.txt")?;

    let mut robot = Robot::new();
    let mut cpu = IntCodeCpu::from_code(&code);
    loop {
        match robot.visited_positions.get(&robot.position).unwrap() {
            Color::Black => cpu.input.push_back(0),
            Color::White => cpu.input.push_back(1),
        }
        if let Some(new_color) = cpu.run_until_output() {
            if let Some(new_direction) = cpu.run_until_output() {
                match new_color {
                    0 => robot.paint(Color::Black),
                    1 => robot.paint(Color::White),
                    _ => panic!("unknown new color"),
                }
                match new_direction {
                    0 => robot.turn_left(),
                    1 => robot.turn_right(),
                    _ => panic!("unknown new direction"),
                }
                robot.move_forward();
            } else {
                break;
            }
        } else {
            break;
        }
    }

    println!("p1: {}", robot.visited_positions.iter().count());

    Ok(())
}
