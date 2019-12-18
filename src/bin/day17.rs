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

fn part1(cpu: &mut IntCodeCpu) -> usize {
    cpu.run();
    print_scaffold_map(&cpu.output);
    get_intersections_count(&parse_scaffold_map(&cpu.output))
}

fn input_ascii_code(cpu: &mut IntCodeCpu, ascii: &str) {
    ascii.chars().for_each(|c| cpu.input.push_back(c as i64));
}

fn part2(cpu: &mut IntCodeCpu) -> i64 {
    cpu.poke_memory(0, 2);

    let _possible_path = "L,12,L,10,R,8,L,12,
                          R,8,R,10,R,12,
                          L,12,L,10,R,8,L,12,
                          R,8,R,10,R,12,
                          L,10,R,12,R,8,
                          L,10,R,12,R,8,
                          R,8,R,10,R,12,
                          L,12,L,10,R,8,L,12,
                          R,8,R,10,R,12,
                          L,10,R,12,R,8";

    input_ascii_code(cpu, "A,B,A,B,C,C,B,A,B,C\n");
    input_ascii_code(cpu, "L,12,L,10,R,8,L,12\n");
    input_ascii_code(cpu, "R,8,R,10,R,12\n");
    input_ascii_code(cpu, "L,10,R,12,R,8\n");
    input_ascii_code(cpu, "n\n");
    cpu.run();
    cpu.output.pop_back().unwrap()
}

fn main() -> io::Result<()> {
    let code = fs::read_to_string("./input/day17.in")?;
    let cpu = IntCodeCpu::from_code(&code.trim());

    println!("p1: {}", part1(&mut cpu.clone()));
    println!("p2: {}", part2(&mut cpu.clone()));
    Ok(())
}
