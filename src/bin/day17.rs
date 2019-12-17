use aoc2019::intcode::IntCodeCpu;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::io;

#[allow(dead_code)]
fn print_scaffold_map(output: &VecDeque<i64>) {
    output
        .iter()
        .map(|e| *e as u8 as char)
        .for_each(|e| print!("{}", e));
}

fn is_intersection(pos: (i64, i64), set: &HashSet<(i64, i64)>) -> bool {
    let (xs, ys) = pos;
    set.contains(&(xs - 1, ys))
        && set.contains(&(xs + 1, ys))
        && set.contains(&(xs, ys + 1))
        && set.contains(&(xs, ys - 1))
}

fn get_intersections_count(scaffold_map: &[Vec<i64>]) -> usize {
    let mut set: HashSet<(i64, i64)> = HashSet::new();
    scaffold_map.iter().enumerate().for_each(|(j, r)| {
        for (i, c) in r.iter().enumerate() {
            if *c == 35 {
                set.insert((i as i64, j as i64));
            }
        }
    });

    set.iter()
        .filter(|e| is_intersection(**e, &set))
        .fold(0, |acc, (x, y)| acc + (x * y) as usize)
}

fn parse_scaffold_map(output: &VecDeque<i64>) -> Vec<Vec<i64>> {
    let mut scaffold_map: Vec<Vec<i64>> = vec![];

    for line in output
        .iter()
        .copied()
        .collect::<Vec<i64>>()
        .split(|e| *e == 10)
    {
        if !line.is_empty() {
            scaffold_map.push(line.to_vec());
        }
    }

    scaffold_map
}

fn part1(output: &VecDeque<i64>) -> usize {
    get_intersections_count(&parse_scaffold_map(&output))
}

fn main() -> io::Result<()> {
    let code = fs::read_to_string("./input/day17.in")?;
    let mut cpu = IntCodeCpu::from_code(&code.trim());

    cpu.run();
    // print_scaffold_map(&cpu.output);
    println!("p1: {}", part1(&cpu.output));
    Ok(())
}
