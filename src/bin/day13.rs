use aoc2019::intcode::IntCodeCpu;
use std::fs;
use std::io;

#[derive(Debug, PartialEq)]
enum TileID {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl From<i64> for TileID {
    fn from(val: i64) -> Self {
        match val {
            0 => TileID::Empty,
            1 => TileID::Wall,
            2 => TileID::Block,
            3 => TileID::HorizontalPaddle,
            4 => TileID::Ball,
            _ => panic!("unknown tile id {}", val),
        }
    }
}

#[derive(Debug)]
struct Tile {
    position: (i64, i64),
    id: TileID,
}

impl Tile {
    fn new() -> Self {
        Tile {
            position: (0, 0),
            id: TileID::Empty,
        }
    }
}

fn get_tiles(cpu: &mut IntCodeCpu) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = vec![];
    loop {
        let mut tile = Tile::new();
        if let Some(x) = cpu.run_until_output() {
            tile.position.0 = x;
            if let Some(y) = cpu.run_until_output() {
                tile.position.1 = y;
                if let Some(id) = cpu.run_until_output() {
                    tile.id = TileID::from(id);
                    tiles.push(tile);
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    tiles
}

fn main() -> io::Result<()> {
    let code = fs::read_to_string("./input/day13.in")?;

    let mut cpu = IntCodeCpu::from_code(&code);

    let tiles = get_tiles(&mut cpu);

    println!(
        "p1: {}",
        tiles.iter().filter(|t| t.id == TileID::Block).count()
    );

    Ok(())
}
