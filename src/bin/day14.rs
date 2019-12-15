use regex::Regex;
use std::cmp::{max, Ordering};
use std::collections::HashMap;
use std::fs;
use std::io;

type Reaction = HashMap<String, (isize, Vec<(String, isize)>)>;
type Stock = HashMap<String, isize>;

// Parses puzzle input and returns a vector of reactions
fn parse_reactions(input: &str) -> Reaction {
    let re = Regex::new(r"(\d+) ([A-Z]+)").unwrap();
    input
        .lines()
        .map(|l| {
            let mut from = re
                .captures_iter(l)
                .map(|c| (c[2].to_string(), c[1].parse::<isize>().unwrap()))
                .collect::<Vec<(String, isize)>>();
            let to = from.pop().unwrap();
            (to.0, (to.1, from))
        })
        .collect::<HashMap<String, (isize, Vec<(String, isize)>)>>()
}

fn get_num_ores(
    reactions: &Reaction,
    stock: &mut Stock,
    needed_chem: &str,
    needed_qty: isize,
) -> isize {
    let (prod_qty, chemicals) = reactions.get(&needed_chem.to_string()).unwrap();

    let availabe = stock.entry(needed_chem.to_string()).or_insert(0);
    let needed = max(0, needed_qty - *availabe);
    let times = needed / prod_qty + (needed % prod_qty != 0) as isize;
    let excess = times * prod_qty - needed_qty;
    *availabe += excess;

    let mut num_ores = 0;
    for chem in chemicals {
        if chem.0 == "ORE" {
            num_ores += chem.1 * times;
        } else {
            num_ores += get_num_ores(reactions, stock, &chem.0, chem.1 * times);
        }
    }
    num_ores
}

const TRILLION: isize = 1_000_000_000_000;

fn get_max_fuel(reactions: &Reaction, stock: &mut Stock, avail_ores: isize) -> isize {
    let mut lb = avail_ores / get_num_ores(reactions, stock, "FUEL", 1);
    let mut ub = lb * 2;

    while lb != ub {
        let m = (lb + ub + 1) / 2;
        match get_num_ores(&reactions, stock, "FUEL", m).cmp(&avail_ores) {
            Ordering::Equal => return m,
            Ordering::Greater => ub = (lb + ub) / 2,
            Ordering::Less => lb = m,
        }
    }
    ub
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./input/day14.in")?;

    let reactions = parse_reactions(&input);
    println!(
        "p1: {}",
        get_num_ores(&reactions, &mut HashMap::new(), "FUEL", 1)
    );

    println!(
        "p2: {}",
        get_max_fuel(&reactions, &mut HashMap::new(), TRILLION)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_reactions() {
        let reactions = parse_reactions(&String::from(
            "10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL",
        ));

        assert_eq!(reactions.len(), 6);
    }
}
