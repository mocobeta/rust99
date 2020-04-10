use P32::gcd;

pub fn is_coprime_to(a: u32, b: u32) -> bool {
    gcd(a, b) == 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_coprime_to() {
        assert_eq!(is_coprime_to(35, 64), true);
        assert_eq!(is_coprime_to(35, 65), false);
    }
}
