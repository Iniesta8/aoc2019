use aoc2019::intcode::{Event, IntCodeCpu};
use itertools::Itertools;
use std::cmp::Ordering;
use std::fs;
use std::io;

#[derive(Debug, PartialEq)]
enum TileID {
    Empty,
    Wall,
    Block,
    HPaddle,
    Ball,
}

impl From<i64> for TileID {
    fn from(val: i64) -> Self {
        match val {
            0 => TileID::Empty,
            1 => TileID::Wall,
            2 => TileID::Block,
            3 => TileID::HPaddle,
            4 => TileID::Ball,
            _ => panic!("unknown tile id: {}", val),
        }
    }
}

#[derive(Debug)]
struct Tile {
    position: (i64, i64),
    id: TileID,
}

fn play(cpu: &mut IntCodeCpu) -> i64 {
    let mut score = 0;

    cpu.poke_memory(0, 2);

    let mut ball = Tile {
        position: (0, 0),
        id: TileID::Ball,
    };

    let mut paddle = Tile {
        position: (0, 0),
        id: TileID::HPaddle,
    };

    let mut outputs = vec![];
    loop {
        let event = cpu.run_until_event();
        match event {
            Event::Halted => break,
            Event::OutputAvailable(val) => {
                outputs.push(val);
                if outputs.len() == 3 {
                    let x = outputs[0];
                    let y = outputs[1];
                    let val = outputs[2];
                    if x == -1 && y == 0 {
                        score = val;
                    } else {
                        match TileID::from(val) {
                            TileID::Ball => ball.position = (x, y),
                            TileID::HPaddle => paddle.position = (x, y),
                            _ => {}
                        }
                    }
                    outputs.clear();
                }
            }
            Event::InputRequired => match ball.position.0.cmp(&paddle.position.0) {
                Ordering::Less => cpu.input.push_back(-1),
                Ordering::Equal => cpu.input.push_back(0),
                Ordering::Greater => cpu.input.push_back(1),
            },
        }
    }
    score
}

fn main() -> io::Result<()> {
    let code = fs::read_to_string("./input/day13.in")?;
    let mut cpu = IntCodeCpu::from_code(&code);

    cpu.run();
    let num_blocks = cpu
        .output
        .iter()
        .tuples()
        .filter(|(_, _, id)| TileID::from(**id) == TileID::Block)
        .count();

    println!("p1: {}", num_blocks);

    cpu = IntCodeCpu::from_code(&code);
    let score = play(&mut cpu);
    println!("p2: {}", score);
    Ok(())
}
