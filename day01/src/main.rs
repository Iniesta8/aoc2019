use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let sum1: u32 = input
        .lines()
        .map(|x| x.parse::<u32>().unwrap() / 3 - 2)
        .sum();

    println!("part 1: {}", sum1);

    let mut sum2: i32 = 0;
    for val in input.lines() {
        let mut v = val.parse::<i32>().unwrap() / 3 - 2;
        while v > 0 {
            sum2 += v;
            v = v / 3 - 2;
        }
    }

    println!("part 2: {}", sum2);

    Ok(())
}
