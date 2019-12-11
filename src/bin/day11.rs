use aoc2019::intcode::IntCodeCpu;
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Color {
    Black,
    White,
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
    brain: IntCodeCpu,
}

impl Robot {
    fn new(brain: IntCodeCpu) -> Self {
        let mut robot = Robot {
            position: (0, 0),
            direction: Direction::Up,
            visited_positions: HashMap::new(),
            brain,
        };
        robot
            .visited_positions
            .entry(robot.position)
            .or_insert(BASIC_PANEL_COLOR);
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
            // Up has to be decreased and down has to be increased,
            // otherwise the result will be upside-down!
            Direction::Up => self.position.1 -= 1,
            Direction::Right => self.position.0 += 1,
            Direction::Down => self.position.1 += 1,
            Direction::Left => self.position.0 -= 1,
        }
    }

    fn paint(&mut self, color: Color) {
        self.visited_positions.insert(self.position, color);
    }
}

fn show_panel(visited_positions: &HashMap<(i32, i32), Color>) {
    let xmin = visited_positions.keys().min_by_key(|xs| xs.0).unwrap().0;
    let xmax = visited_positions.keys().max_by_key(|xs| xs.0).unwrap().0;
    let ymin = visited_positions.keys().min_by_key(|ys| ys.1).unwrap().1;
    let ymax = visited_positions.keys().max_by_key(|ys| ys.1).unwrap().1;

    let mut panel =
        vec![vec![' '; (xmax - xmin).abs() as usize + 1]; (ymax - ymin).abs() as usize + 1];

    for pos in visited_positions.keys() {
        if *visited_positions.get(&pos).unwrap() == Color::White {
            let (xs, ys) = pos;
            panel[(*ys + ymin.abs()) as usize][(*xs + xmin.abs()) as usize] = '█';
        }
    }

    for l in panel {
        for c in l {
            print!("{}", c);
        }
        println!();
    }
}

const BASIC_PANEL_COLOR: Color = Color::White;

fn main() -> io::Result<()> {
    let code = fs::read_to_string("./input/day11.txt")?;

    let mut robot = Robot::new(IntCodeCpu::from_code(&code));
    loop {
        match robot
            .visited_positions
            .get(&robot.position)
            .unwrap_or(&BASIC_PANEL_COLOR)
        {
            Color::Black => robot.brain.input.push_back(0),
            Color::White => robot.brain.input.push_back(1),
        }
        if let Some(new_color) = robot.brain.run_until_output() {
            robot.paint(Color::from(new_color));

            if let Some(new_direction) = robot.brain.run_until_output() {
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

    println!("p2: ");
    show_panel(&robot.visited_positions);

    Ok(())
}
