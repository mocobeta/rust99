// naive implementation
/*
pub fn is_prime(n: u32) -> bool {
    let root = (n as f32).sqrt().ceil() as u32;
    for i in 2..root {
        if n % i == 0 {
            return false;
        }
    }
    true
}
*/

pub struct Primes {
    prime_tbl: Vec<u32>,
}

impl Primes {
    pub fn new() -> Self {
        Primes { prime_tbl: vec![2] }
    }

    // a bit more efficient implementation with memoization
    pub fn is_prime(&mut self, n: u32) -> bool {
        if n == 1 {
            return false;
        } else if n == 2 || n == 3 {
            return true;
        }
        let sq_root = (n as f32).sqrt().ceil() as u32;
        if self.prime_tbl.last().unwrap() < &sq_root {
            self.populate_prime_tbl(sq_root);
        }
        for p in &self.prime_tbl {
            if n % p == 0 {
                return false;
            }
        }
        true
    }

    fn populate_prime_tbl(&mut self, upper: u32) {
        let start = *self.prime_tbl.last().unwrap();
        for i in start..upper + 1 {
            if self.is_prime(i) {
                self.prime_tbl.push(i);
            }
        }
    }

    // get current prime numbers already computed for debug
    pub fn prime_tbl(&self) -> &[u32] {
        &self.prime_tbl[..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_prime() {
        let mut primes = Primes::new();
        assert_eq!(primes.is_prime(1), false);
        assert_eq!(primes.is_prime(2), true);
        assert_eq!(primes.is_prime(3), true);
        assert_eq!(primes.is_prime(53), true);
        assert_eq!(primes.is_prime(1957), false);
    }
}
