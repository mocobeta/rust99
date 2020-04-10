pub fn penultimate<T>(li: &Vec<T>) -> Option<&T> {
    if li.len() > 1 {
        li.get(li.len() - 2)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_penultimate() {
        let li = vec![1, 1, 2, 3, 5, 8];
        assert_eq!(penultimate(&li), Some(&5));
    }

    #[test]
    fn test_penultimate_empty() {
        let li = Vec::<usize>::new();
        assert_eq!(penultimate(&li), None);
    }

    #[test]
    fn test_penultimate_one_elem() {
        let li = vec![1];
        assert_eq!(penultimate(&li), None);
    }
}
