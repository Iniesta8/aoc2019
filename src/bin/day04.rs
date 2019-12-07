use std::iter::from_fn;

fn digits_inc_or_same(num: u32) -> bool {
    let mut it = rev_digit_iter(num);
    let mut lastd = it.next().unwrap();
    for d in it {
        if lastd < d {
            return false;
        }
        lastd = d;
    }
    true
}

fn adjacent_digits_same(num: u32) -> bool {
    let mut it = rev_digit_iter(num);
    let mut lastd = it.next().unwrap();
    for d in it {
        if lastd == d {
            return true;
        }
        lastd = d;
    }
    false
}

fn adjacent_digits_same_advanced(num: u32) -> bool {
    let mut count = 1;

    let mut it = rev_digit_iter(num);
    let mut lastd = it.next().unwrap();
    for d in it {
        if lastd == d {
            count += 1;
        } else {
            if count == 2 {
                return true;
            }
            count = 1;
        }
        lastd = d;
    }

    if count == 2 {
        return true;
    }

    false
}

fn rev_digit_iter(mut num: u32) -> impl Iterator<Item = u32> {
    from_fn(move || {
        if num == 0 {
            None
        } else {
            let d = num % 10;
            num /= 10;
            Some(d)
        }
    })
}

fn main() {
    let p1 = (138_241..=674_034)
        .filter(|&x| digits_inc_or_same(x) && adjacent_digits_same(x))
        .count();

    let p2 = (138_241..=674_034)
        .filter(|&x| digits_inc_or_same(x) && adjacent_digits_same_advanced(x))
        .count();

    println!("p1: {}", p1);
    println!("p2: {}", p2);
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

    #[test]
    fn test_adjacent_digits_same_advanced() {
        assert_eq!(adjacent_digits_same_advanced(122345), true);
        assert_eq!(adjacent_digits_same_advanced(111122), true);
        assert_eq!(adjacent_digits_same_advanced(123444), false);
        assert_eq!(adjacent_digits_same_advanced(112233), true);
        assert_eq!(adjacent_digits_same_advanced(111111), false);
        assert_eq!(adjacent_digits_same_advanced(112222), true);
    }
}
