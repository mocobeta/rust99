pub fn is_palindrome<T: PartialEq>(li: &Vec<T>) -> bool {
    let mid = li.len() / 2;
    for i in 0..mid {
        if li.get(i).unwrap() != li.get(li.len() - (1 + i)).unwrap() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_odd_elements() {
        let li = vec![1, 2, 3, 2, 1];
        assert_eq!(is_palindrome(&li), true);
    }

    #[test]
    fn test_even_elements() {
        let li = vec![1, 2, 2, 1];
        assert_eq!(is_palindrome(&li), true);
    }

    #[test]
    fn test_odd_elements_non() {
        let li = vec![1, 3, 3, 2, 1];
        assert_eq!(is_palindrome(&li), false);
    }

    #[test]
    fn test_even_elements_non() {
        let li = vec![1, 3, 2, 1];
        assert_eq!(is_palindrome(&li), false);
    }

    #[test]
    fn test_empty_list() {
        let li = Vec::<usize>::new();
        assert_eq!(is_palindrome(&li), true);
    }

    #[test]
    fn test_one_elemens() {
        let li = vec![1];
        assert_eq!(is_palindrome(&li), true);
    }
}
