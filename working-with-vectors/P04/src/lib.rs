pub fn length<T>(li: &Vec<T>) -> usize {
    li.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length() {
        let li = vec![1, 1, 2, 3, 5, 8];
        assert_eq!(length(&li), 6);
    }
}
