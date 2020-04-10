pub fn nth<T>(n: usize, li: &Vec<T>) -> Option<&T> {
    li.get(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nth() {
        let li = vec![1, 1, 2, 3, 5, 8];
        assert_eq!(nth(2, &li), Some(&2));
    }

    #[test]
    fn test_nth_oob() {
        let li = vec![1, 1, 2, 3, 5, 8];
        assert_eq!(nth(10, &li), None);
    }
}
