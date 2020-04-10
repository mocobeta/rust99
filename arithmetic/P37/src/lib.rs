use P36::prime_factor_multiplicity;

pub fn totient(n: u32) -> u32 {
    let prime_factors = prime_factor_multiplicity(n);
    prime_factors
        .iter()
        .fold(1, |acc, (p, m)| acc * (p - 1) * p.pow(m - 1))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_totient() {
        assert_eq!(totient(6), 2);
        assert_eq!(totient(20), 8);
        assert_eq!(totient(123456), 41088);
    }
}
