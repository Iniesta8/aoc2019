use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

fn verify_data(layers: &[Vec<u8>]) -> u32 {
    let l = layers
        .iter()
        .map(|e| {
            let mut digits_count: (u32, u32, u32) = (0, 0, 0);
            for d in e.iter() {
                match d {
                    0 => digits_count.0 += 1,
                    1 => digits_count.1 += 1,
                    2 => digits_count.2 += 1,
                    _ => continue,
                }
            }
            digits_count
        })
        .min()
        .unwrap();

    l.1 * l.2
}

fn main() {
    let input = fs::read_to_string("./input/day08.txt").unwrap();

    let digits: Vec<u8> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let layers: Vec<Vec<u8>> = digits.chunks(LAYER_SIZE).map(|ch| ch.to_vec()).collect();

    println!("p1: {}", verify_data(&layers));
}
