use P39::list_primes_in_range;

pub fn goldbach(n: u32) -> (u32, u32) {
    if n <= 2 || n % 2 != 0 {
        panic!("{} is not an even number greater than 2")
    }
    let primes = list_primes_in_range(1, n);
    match primes.iter().find(|&&p| primes.contains(&(n - p))) {
        Some(p) => (*p, n - *p),
        None => panic!("it breaks Goldbach's conjecture: {}", n),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list_primes_in_range() {
        assert_eq!(goldbach(28), (5, 23));
    }
}
