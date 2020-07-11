use P35::PrimeIterator;

pub fn list_primes_in_range(lower: u32, upper: u32) -> Vec<u32> {
    PrimeIterator::new()
        .skip_while(|&p| p < lower)
        .take_while(|&p| p <= upper)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list_primes_in_range() {
        assert_eq!(
            list_primes_in_range(7, 31),
            vec![7, 11, 13, 17, 19, 23, 29, 31]
        );
    }
}
