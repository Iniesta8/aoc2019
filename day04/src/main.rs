use std::iter::from_fn;

fn digits_inc_or_same(num: u32) -> bool {
    let mut it = rev_digit_iter(num);
    let mut lastc = it.next().unwrap();
    for d in it {
        if lastc < d {
            return false;
        }
        lastc = d;
    }
    true
}

fn adjacent_digits_same(num: u32) -> bool {
    let mut it = rev_digit_iter(num);
    let mut lastc = it.next().unwrap();
    for d in it {
        if lastc == d {
            return true;
        }
        lastc = d;
    }
    false
}

fn rev_digit_iter(mut num: u32) -> impl Iterator<Item = u32> {
    from_fn(move || {
        if num == 0 {
            return None;
        } else {
            let n = num % 10;
            num /= 10;
            return Some(n);
        }
    })
}

fn main() {
    let p1 = (138241..=674034)
        .filter(|&x| adjacent_digits_same(x) && digits_inc_or_same(x))
        .count();

    println!("p1: {}", p1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits_inc_or_same() {
        assert_eq!(digits_inc_or_same(111111), true);
        assert_eq!(digits_inc_or_same(223450), false);
    }

    #[test]
    fn test_two_adjacent_digits_same() {
        assert_eq!(adjacent_digits_same(122345), true);
        assert_eq!(adjacent_digits_same(111111), true);
    }
}
