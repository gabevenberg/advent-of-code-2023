pub fn arr_lcm(input: &[usize]) -> usize {
    input.iter().copied().reduce(lcm).unwrap_or(0)
}

pub fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(&first, &second)
}

pub fn gcd(a: &usize, b: &usize) -> usize {
    let mut a = *a;
    let mut b = *b;
    while b != 0 {
        let tmp = b;
        b = a.rem_euclid(b);
        a = tmp;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arr_lcm() {
        assert_eq!(arr_lcm(&[4, 6]), 12);
        assert_eq!(arr_lcm(&[5, 2]), 10);
        assert_eq!(arr_lcm(&[5, 2, 6]), 30);
        assert_eq!(arr_lcm(&[5, 2, 6, 3]), 30);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(5, 2), 10);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(&8, &12), 4);
        assert_eq!(gcd(&54, &24), 6);
    }
}
