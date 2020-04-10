pub fn gcd(a: u32, b: u32) -> u32 {
    let (m, n) = if a >= b { (a, b) } else { (b, a) };
    gcd_rec(m, n)
}

fn gcd_rec(m: u32, n: u32) -> u32 {
    if m < n {
        panic!("m must be larger than or equal to n; got m={}, n={}", m, n);
    }
    if n == 0 {
        m
    } else {
        gcd_rec(n, m % n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(36, 63), 9);
        assert_eq!(gcd(63, 36), 9);
        assert_eq!(gcd(7, 21), 7);
    }

    #[test]
    fn test_gcd_one() {
        assert_eq!(gcd_rec(19, 13), 1);
    }
}
