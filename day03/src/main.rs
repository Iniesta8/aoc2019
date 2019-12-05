use std::collections::{HashMap, HashSet};
use std::io;

fn calc_distance(pos1: (i32, i32), pos2: (i32, i32)) -> u32 {
    ((pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs()) as u32
}

fn get_route(route: &mut HashMap<(i32, i32), u32>, route_raw: &[&str]) {
    let mut cur_pos = (0, 0);
    let mut total_dist = 0;
    for dir in route_raw {
        let mut dist = dir[1..].parse::<i32>().unwrap();
        match &dir[..1] {
            "L" => {
                while dist != 0 {
                    cur_pos.0 -= 1;
                    total_dist += 1;
                    route.insert(cur_pos, total_dist);
                    dist -= 1;
                }
            }
            "U" => {
                while dist != 0 {
                    cur_pos.1 += 1;
                    total_dist += 1;
                    route.insert(cur_pos, total_dist);
                    dist -= 1;
                }
            }
            "R" => {
                while dist != 0 {
                    cur_pos.0 += 1;
                    total_dist += 1;
                    route.insert(cur_pos, total_dist);
                    dist -= 1;
                }
            }
            "D" => {
                while dist != 0 {
                    cur_pos.1 -= 1;
                    total_dist += 1;
                    route.insert(cur_pos, total_dist);
                    dist -= 1;
                }
            }
            _ => {
                panic!("unknown direction");
            }
        };
    }
}

fn main() -> io::Result<()> {
    let mut wire1_input = String::new();
    let mut wire2_input = String::new();
    io::stdin().read_line(&mut wire1_input)?;
    io::stdin().read_line(&mut wire2_input)?;

    let wire1: Vec<&str> = wire1_input.trim().split(',').collect();
    let wire2: Vec<&str> = wire2_input.trim().split(',').collect();

    let mut wire1_map: HashMap<(i32, i32), u32> = HashMap::new();
    get_route(&mut wire1_map, &wire1);
    let mut wire2_map: HashMap<(i32, i32), u32> = HashMap::new();
    get_route(&mut wire2_map, &wire2);

    let wire1_set: HashSet<(i32, i32)> = wire1_map.keys().copied().collect();
    let wire2_set: HashSet<(i32, i32)> = wire2_map.keys().copied().collect();

    let is = wire1_set.intersection(&wire2_set);

    println!(
        "p1: {}",
        is.clone()
            .map(|ele| calc_distance((0, 0), *ele))
            .min()
            .unwrap()
    );

    println!(
        "p2: {}",
        is.map(|ele| wire1_map.get(&ele).unwrap() + wire2_map.get(&ele).unwrap())
            .min()
            .unwrap()
    );

    Ok(())
}
