pub fn range(start: i32, end: i32) -> Vec<i32> {
    (start..end + 1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_range_4to9() {
        assert_eq!(range(4, 9), vec![4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_range_neg2to4() {
        assert_eq!(range(-2, 4), vec![-2, -1, 0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_range_9to4() {
        assert_eq!(range(9, 4), vec![]);
    }
}
