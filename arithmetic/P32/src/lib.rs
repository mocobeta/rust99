pub fn gcd(a: u32, b: u32) -> u32 {
    fn gcd_rec(m: u32, n: u32) -> u32 {
        assert!(
            m >= n,
            "m must be larger than or equal to n; got m={}, n={}",
            m,
            n
        );
        if n == 0 {
            m
        } else {
            gcd_rec(n, m % n)
        }
    }

    let (m, n) = if a >= b { (a, b) } else { (b, a) };
    gcd_rec(m, n)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(36, 63), 9);
        assert_eq!(gcd(63, 36), 9);
        assert_eq!(gcd(7, 21), 7);
        assert_eq!(gcd(19, 13), 1);
    }
}
