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

fn is_in_sight(
    asteroids: &[Vec<bool>],
    (from_x, from_y): (usize, usize),
    (to_x, to_y): (usize, usize),
) -> bool {
    if !asteroids[from_y][from_x] || !asteroids[to_y][to_x] {
        return false;
    }

    if from_x == to_x && from_y == to_y {
        return false;
    }

    let (mut tp_x, mut tp_y) = (from_x, from_y);

    while (tp_x, tp_y) != (to_x, to_y) {
        let xs = to_x as i32 - from_x as i32;
        let ys = to_y as i32 - from_y as i32;
        let gcd = gcd(xs, ys);

        tp_x = (tp_x as i32 + (xs / gcd)) as usize;
        tp_y = (tp_y as i32 + (ys / gcd)) as usize;
        if asteroids[tp_y][tp_x] && (tp_x, tp_y) != (to_x, to_y) {
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

fn get_visible_counts_per_pos(asteroids: &[Vec<bool>]) -> HashMap<(usize, usize), usize> {
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();

    let xmax = asteroids[0].len();
    let ymax = asteroids.len();

    for i in 0..xmax {
        for j in 0..ymax {
            for k in 0..xmax {
                for l in 0..ymax {
                    if is_in_sight(&asteroids, (i, j), (k, l)) {
                        if let Some(x) = map.get_mut(&(i, j)) {
                            *x += 1;
                        } else {
                            map.insert((i, j), 1);
                        }
                    }
                }
            }
        }
    }

    map
}

fn get_visible_asteroids(
    asteroids: &[Vec<bool>],
    (from_x, from_y): (usize, usize),
) -> Vec<(usize, usize)> {
    let xmax = asteroids[0].len();
    let ymax = asteroids.len();

    let mut visible_asteroids = vec![];

    for i in 0..xmax {
        for j in 0..ymax {
            if is_in_sight(&asteroids, (from_x, from_y), (i, j)) {
                visible_asteroids.push((i, j));
            }
        }
    }

    visible_asteroids
}

fn parse_raw_map(raw_map: &str) -> Vec<Vec<bool>> {
    let mut asteroids = vec![];
    for line in raw_map.lines() {
        let v: Vec<bool> = line.trim().chars().map(|e| e == '#').collect();
        asteroids.push(v);
    }
    asteroids
}

fn main() -> io::Result<()> {
    let raw_map = fs::read_to_string("./input/day10.txt")?;
    let asteroids = parse_raw_map(&raw_map);
    let best_asteroid: Asteroid = find_best_asteroid(&get_visible_counts_per_pos(&asteroids));

    println!(
        "p1: best position is {:?}, there are {} asteroids in sight",
        best_asteroid.position, best_asteroid.in_sight_count
    );

    let visible_asteroids = get_visible_asteroids(&asteroids, best_asteroid.position);

    dbg!(&visible_asteroids, &visible_asteroids.len());

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

        assert_eq!(is_in_sight(&asteroids, (1, 0), (4, 0)), true);
        assert_eq!(is_in_sight(&asteroids, (1, 0), (2, 0)), false);
        assert_eq!(is_in_sight(&asteroids, (1, 0), (1, 2)), true);
        assert_eq!(is_in_sight(&asteroids, (3, 4), (1, 0)), false);
    }
}
