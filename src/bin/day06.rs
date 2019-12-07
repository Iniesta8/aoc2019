use std::collections::HashMap;
use std::fs;

fn solve_p1(edges: &[(&str, &str)]) -> usize {
    let mut orbit_map: HashMap<&str, &str> = HashMap::new();
    for (orbited, orbiter) in edges {
        orbit_map.insert(orbiter, orbited);
    }

    let mut count = 0;
    for (_, orbiter) in edges {
        let mut orbited = orbiter;
        while orbit_map.contains_key(orbited) {
            count += 1;
            orbited = orbit_map.get(orbited).unwrap();
        }
    }

    count
}

fn solve_p2(edges: &[(&str, &str)]) -> usize {
    let mut orbit_map: HashMap<&str, &str> = HashMap::new();
    for (orbited, orbiter) in edges {
        orbit_map.insert(orbiter, orbited);
    }

    let mut hops = 0;
    let mut route: HashMap<&str, usize> = HashMap::new();
    let mut orbited = orbit_map.get("YOU").unwrap();
    while orbit_map.contains_key(orbited) {
        hops += 1;
        orbited = orbit_map.get(orbited).unwrap();
        route.insert(orbited, hops);
    }

    hops = 0;
    orbited = orbit_map.get("SAN").unwrap();
    while orbit_map.contains_key(orbited) {
        hops += 1;
        orbited = orbit_map.get(orbited).unwrap();
        if route.contains_key(orbited) {
            return hops + route.get(orbited).unwrap();
        }
    }

    panic!("no route found");
}

fn main() {
    let input = fs::read_to_string("./input/day06.txt").unwrap();
    let edges: Vec<(&str, &str)> = input
        .split('\n')
        .filter(|ele| !ele.is_empty())
        .map(|ele| {
            let edge: Vec<&str> = ele.split(')').collect();
            (edge[0], edge[1])
        })
        .collect();

    println!("p1: {}", solve_p1(&edges));
    println!("p2: {}", solve_p2(&edges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_p1() {
        assert_eq!(
            solve_p1(&[
                ("COM", "B"),
                ("B", "C"),
                ("C", "D"),
                ("D", "E"),
                ("E", "F"),
                ("B", "G"),
                ("G", "H"),
                ("D", "I"),
                ("E", "J"),
                ("J", "K"),
                ("K", "L"),
            ]),
            42
        );
    }

    #[test]
    fn test_solve_p2() {
        assert_eq!(
            solve_p2(&[
                ("COM", "B"),
                ("B", "C"),
                ("C", "D"),
                ("D", "E"),
                ("E", "F"),
                ("B", "G"),
                ("G", "H"),
                ("D", "I"),
                ("E", "J"),
                ("J", "K"),
                ("K", "L"),
                ("K", "YOU"),
                ("I", "SAN"),
            ]),
            4
        );
    }
}
