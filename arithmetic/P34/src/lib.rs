use P33::is_coprime_to;

pub fn totient(m: u32) -> u32 {
    (1..m).fold(0, |tot, x| if is_coprime_to(m, x) { tot + 1 } else { tot })
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
