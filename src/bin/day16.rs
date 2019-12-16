use std::fs;
use std::io;

fn calc_fft(input: &mut Vec<i8>, cycles: usize) -> isize {
    let base_pattern = vec![0, 1, 0, -1];
    for _ in 0..cycles {
        for i in 0..input.len() - 1 {
            input[i] = (input
                .iter()
                .enumerate()
                .skip(i)
                .map(|(j, d)| *d as isize * base_pattern[((j + 1) / (i + 1)) % 4] as isize)
                .sum::<isize>()
                .abs()
                % 10) as i8;
        }
    }
    input[..8]
        .iter()
        .map(|e| e.to_string())
        .collect::<String>()
        .parse::<isize>()
        .unwrap()
}

fn get_offset(input: &[i8]) -> usize {
    input
        .iter()
        .take(7)
        .map(|d| d.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn calc_fft_with_offset(input: &[i8], cycles: usize) -> isize {
    let offset: usize = get_offset(&input);

    let mut digits: Vec<i8> = input
        .iter()
        .cycle()
        .take(10000 * input.len())
        .skip(offset)
        .copied()
        .collect();

    for _ in 0..cycles {
        for i in (0..digits.len() - 1).rev() {
            digits[i] = (digits[i] + digits[i + 1]) % 10;
        }
    }

    digits[..8]
        .iter()
        .map(|e| e.to_string())
        .collect::<String>()
        .parse::<isize>()
        .unwrap()
}

fn main() -> io::Result<()> {
    let mut input: Vec<i8> = fs::read_to_string("./input/day16.in")?
        .trim()
        .chars()
        .map(|e| e.to_digit(10).unwrap() as i8)
        .collect();

    let input_b = input.clone();

    println!("p1: {}", calc_fft(&mut input, 100));
    println!("p2: {}", calc_fft_with_offset(&input_b, 100));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_fft() {
        let mut data: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        calc_fft(&mut data, 1);
        assert_eq!(data, vec![4, 8, 2, 2, 6, 1, 5, 8]);
        calc_fft(&mut data, 1);
        assert_eq!(data, vec![3, 4, 0, 4, 0, 4, 3, 8]);
        calc_fft(&mut data, 1);
        assert_eq!(data, vec![0, 3, 4, 1, 5, 5, 1, 8]);
        calc_fft(&mut data, 1);
        assert_eq!(data, vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn test_calc_fft2() {
        let mut data = vec![
            8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8, 6, 4, 5,
            5, 9, 5,
        ];
        calc_fft(&mut data, 100);

        assert_eq!(vec![2, 4, 1, 7, 6, 1, 7, 6], &data[0..8]);
    }
}
