use aoc2019::intcode::IntCodeCpu;
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Clone, Copy, Debug)]
enum Color {
    Black = 0,
    White = 1,
}

impl From<i64> for Color {
    fn from(val: i64) -> Self {
        match val {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("unknown color {}", val),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

impl From<i64> for Turn {
    fn from(val: i64) -> Self {
        match val {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => panic!("unknown turn direction {}", val),
        }
    }
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

    fn turn(&mut self, turn: Turn) {
        match turn {
            Turn::Left => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Up,
                }
            }
            Turn::Right => {
                self.direction = match self.direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                }
            }
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
                robot.paint(Color::from(new_color));
                robot.turn(Turn::from(new_direction));
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
