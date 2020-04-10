pub fn last<T>(li: &Vec<T>) -> Option<&T> {
    li.last()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_last() {
        let li = vec![1, 1, 2, 3, 5, 8];
        assert_eq!(last(&li), Some(&8));
    }

    #[test]
    fn test_last_empty_list() {
        let li = Vec::<usize>::new();
        assert_eq!(last(&li), None);
    }
}
