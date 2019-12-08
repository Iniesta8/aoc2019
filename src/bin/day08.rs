use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

fn get_layers(data: &[u8], lsize: usize) -> Vec<Vec<u8>> {
    data.chunks(lsize).map(|ch| ch.to_vec()).collect()
}

fn verify_data(data: &[u8], lsize: usize) -> u32 {
    let layers = get_layers(&data, lsize);

    let (_zeros, ones, twos) = layers
        .iter()
        .map(|e| {
            let mut digits_count: (u32, u32, u32) = (0, 0, 0);
            for d in e.iter() {
                match d {
                    0 => digits_count.0 += 1,
                    1 => digits_count.1 += 1,
                    2 => digits_count.2 += 1,
                    _ => panic!("unknown color"),
                }
            }
            digits_count
        })
        .min()
        .unwrap();

    ones * twos
}

fn decode_ssif(data: &[u8], lsize: usize) -> Vec<u8> {
    let layers = get_layers(&data, lsize);

    let mut img_data = vec![2u8; lsize];

    for layer in layers {
        for (ip, lp) in img_data.iter_mut().zip(layer.iter()) {
            if *ip == 2 && *lp != 2 {
                *ip = *lp;
            }
        }
    }
    img_data
}

fn print_image(img_data: &[u8], width: usize) {
    img_data.chunks(width).for_each(|l| {
        let line: String = l
            .iter()
            .map(|p| match p {
                1 => '█',
                _ => ' ',
            })
            .collect();
        println!("{}", line);
    })
}

fn main() {
    let input = fs::read_to_string("./input/day08.txt").unwrap();

    let digits: Vec<u8> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    println!("p1: {}", verify_data(&digits, LAYER_SIZE));

    println!("p2: ");
    print_image(&decode_ssif(&digits, LAYER_SIZE), WIDTH);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_ssif() {
        assert_eq!(
            decode_ssif(&vec![0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0], 4),
            &[0, 1, 1, 0]
        );
    }
}
