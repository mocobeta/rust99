use P31::Primes;

pub fn prime_factors(n: u32) -> Vec<u32> {
    let piter = PrimeIterator::new();
    let mut factors = vec![];
    let mut r = n;
    for prime in piter {
        if r == 1 {
            break;
        }
        while r % prime == 0 {
            factors.push(prime);
            r = r / prime;
        }
    }
    factors
}

pub struct PrimeIterator {
    primes: Primes,
    next: u32,
}

impl PrimeIterator {
    pub fn new() -> Self {
        PrimeIterator {
            primes: Primes::new(),
            next: 2,
        }
    }
}

impl Iterator for PrimeIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.next;
        self.next += 1;
        while !self.primes.is_prime(self.next) {
            self.next += 1;
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prime_sequence() {
        let primes: Vec<u32> = PrimeIterator::new().take(20).collect();
        assert_eq!(
            primes,
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71]
        );
    }
    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(9), vec![3, 3]);
        assert_eq!(prime_factors(20), vec![2, 2, 5]);
        assert_eq!(prime_factors(111), vec![3, 37]);
        assert_eq!(prime_factors(315), vec![3, 3, 5, 7]);
    }
}
