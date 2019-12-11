use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::io;

struct Asteroid {
    position: (usize, usize),
    in_sight_count: usize,
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a == 0 {
        return b.abs();
    }
    if b == 0 {
        return a.abs();
    }
    while b != 0 {
        let h = a % b;
        a = b;
        b = h;
    }
    a.abs()
}

fn get_direction((to_x, to_y): (usize, usize), (from_x, from_y): (usize, usize)) -> (i32, i32) {
    let xs = to_x as i32 - from_x as i32;
    let ys = to_y as i32 - from_y as i32;
    let gcd = gcd(xs, ys);
    if gcd == 0 {
        (0, 0)
    } else {
        (xs / gcd, ys / gcd)
    }
}

fn get_directions(asteroids: &[(usize, usize)], from: (usize, usize)) -> Vec<(i32, i32)> {
    asteroids.iter().map(|&a| get_direction(a, from)).collect()
}

fn is_in_sight(
    asteroids: &[(usize, usize)],
    (to_x, to_y): (usize, usize),
    (from_x, from_y): (usize, usize),
) -> bool {
    if !asteroids.contains(&(to_x, to_y)) {
        return false;
    }
    if from_x == to_x && from_y == to_y {
        return false;
    }

    let (mut tp_x, mut tp_y) = (from_x, from_y);

    while (tp_x, tp_y) != (to_x, to_y) {
        let (xdir, ydir) = get_direction((to_x, to_y), (from_x, from_y));

        tp_x = (tp_x as i32 + xdir) as usize;
        tp_y = (tp_y as i32 + ydir) as usize;
        if asteroids.contains(&(tp_x, tp_y)) && (tp_x, tp_y) != (to_x, to_y) {
            return false;
        }
    }

    true
}

fn find_best_asteroid(map: &HashMap<(usize, usize), usize>) -> Asteroid {
    let mut best = Asteroid {
        position: (0, 0),
        in_sight_count: 0,
    };

    for (k, v) in map.iter() {
        if *v > best.in_sight_count {
            best.position = *k;
            best.in_sight_count = *v;
        }
    }

    best
}

fn get_visible_counts_per_pos(asteroids: &[(usize, usize)]) -> HashMap<(usize, usize), usize> {
    let mut vis_counts: HashMap<(usize, usize), usize> = HashMap::new();

    for from in asteroids {
        for to in asteroids {
            if is_in_sight(&asteroids, *to, *from) {
                if let Some(x) = vis_counts.get_mut(from) {
                    *x += 1;
                } else {
                    vis_counts.insert(*from, 1);
                }
            }
        }
    }
    vis_counts
}

fn parse_raw_map(raw_map: &str) -> Vec<(usize, usize)> {
    let mut asteroids = vec![];
    for (i, line) in raw_map.lines().enumerate() {
        for (j, col) in line.trim().chars().enumerate() {
            if col == '#' {
                asteroids.push((j, i));
            }
        }
    }
    asteroids
}

fn main() -> io::Result<()> {
    let raw_map = fs::read_to_string("./input/day10.in")?;
    let asteroids = parse_raw_map(&raw_map);

    let best_asteroid: Asteroid = find_best_asteroid(&get_visible_counts_per_pos(&asteroids));

    println!(
        "p1: best position is {:?}, there are {} asteroids in sight",
        best_asteroid.position, best_asteroid.in_sight_count
    );

    let laser_directions: Vec<(i32, i32)> = get_directions(&asteroids, best_asteroid.position)
        .into_iter()
        .filter(|e| *e != (0, 0))
        .unique()
        .sorted_by(|a, b| {
            (f64::from(a.0).atan2(f64::from(a.1)))
                .partial_cmp(&f64::from(b.0).atan2(f64::from(b.1)))
                .unwrap()
                .reverse()
        })
        .collect();
    let mut asteroids_by_direction = asteroids
        .into_iter()
        .map(|e| (get_direction(e, best_asteroid.position), e))
        .into_group_map();

    let mut vapor_count = 0;
    for dir in &laser_directions {
        if let Some(asteroids) = asteroids_by_direction.get_mut(dir) {
            if !asteroids.is_empty() {
                let target = asteroids.remove(0);
                vapor_count += 1;
                if vapor_count == 200 {
                    println!(
                        "p2: 200th asteroid to be vaporized is at position ({}, {})",
                        target.0, target.1
                    );
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_sight() {
        let asteroids = parse_raw_map(
            ".#..#
        .....
        #####
        ....#
        ...##",
        );

        assert_eq!(is_in_sight(&asteroids, (4, 0), (1, 0)), true);
        assert_eq!(is_in_sight(&asteroids, (2, 0), (1, 0)), false);
        assert_eq!(is_in_sight(&asteroids, (1, 2), (1, 0)), true);
        assert_eq!(is_in_sight(&asteroids, (1, 0), (3, 4)), false);
    }
}
