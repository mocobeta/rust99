use P35::PrimeIterator;

pub fn prime_factor_multiplicity(n: u32) -> Vec<(u32, u32)> {
    let piter = PrimeIterator::new();
    let mut factors = vec![];
    let mut r = n;
    for prime in piter {
        if r == 1 {
            break;
        }
        let mut c = 0;
        while r % prime == 0 {
            c += 1;
            r = r / prime;
        }
        if c > 0 {
            factors.push((prime, c));
        }
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prime_factor_multiplicity() {
        assert_eq!(prime_factor_multiplicity(9), vec![(3, 2)]);
        assert_eq!(prime_factor_multiplicity(20), vec![(2, 2), (5, 1)]);
        assert_eq!(prime_factor_multiplicity(111), vec![(3, 1), (37, 1)]);
        assert_eq!(prime_factor_multiplicity(315), vec![(3, 2), (5, 1), (7, 1)]);
    }
}
