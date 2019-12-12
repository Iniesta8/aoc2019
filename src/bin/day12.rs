use std::fs;
use std::io;

#[derive(Debug, Clone)]
struct Moon {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
}

impl Moon {
    fn new(position: (i32, i32, i32)) -> Self {
        Moon {
            position,
            velocity: (0, 0, 0),
        }
    }

    fn apply_gravity(&mut self, other: &Moon) {
        if other.position.0 < self.position.0 {
            self.velocity.0 -= 1;
        } else if other.position.0 > self.position.0 {
            self.velocity.0 += 1;
        }

        if other.position.1 < self.position.1 {
            self.velocity.1 -= 1;
        } else if other.position.1 > self.position.1 {
            self.velocity.1 += 1;
        }

        if other.position.2 < self.position.2 {
            self.velocity.2 -= 1;
        } else if other.position.2 > self.position.2 {
            self.velocity.2 += 1;
        }
    }

    fn apply_velocity(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    fn potential_energy(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn parse_input(input: &str) -> Vec<Moon> {
    input
        .lines()
        .map(|pos| {
            let mut moon = Moon::new((0, 0, 0));
            for (i, p) in pos[1..pos.len() - 1].split(',').enumerate() {
                let val = p.split('=').last().unwrap().parse::<i32>().unwrap();
                match i {
                    0 => moon.position.0 = val,
                    1 => moon.position.1 = val,
                    2 => moon.position.2 = val,
                    _ => panic!("unknown coord axis"),
                }
            }
            moon
        })
        .collect()
}

fn time_step(moons: &mut Vec<Moon>) {
    let moons_cloned = moons.clone();
    for moon in moons.iter_mut() {
        for other in moons_cloned.iter() {
            if moon.position != other.position {
                moon.apply_gravity(other);
            }
        }
        moon.apply_velocity();
    }
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./input/day12.in")?;

    let mut moons: Vec<Moon> = parse_input(&input);

    for _ in 0..1_000 {
        time_step(&mut moons);
    }

    println!(
        "p1: {}",
        moons.iter().map(|m| m.total_energy()).sum::<i32>()
    );

    Ok(())
}
