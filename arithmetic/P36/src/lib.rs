use std::collections::HashMap;
use P35::prime_factors;

pub fn prime_factor_multiplicity(n: u32) -> Vec<(u32, u32)> {
    // create factor -> multiplicity map
    let mut factors: Vec<(u32, u32)> = prime_factors(n)
        .iter()
        .fold(HashMap::<u32, u32>::new(), |mut map, x| {
            let e = map.entry(*x).or_insert(0);
            *e += 1;
            map
        })
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect();
    // sort factors by ascending order
    factors.sort_by(|a, b| a.0.cmp(&b.0));
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
