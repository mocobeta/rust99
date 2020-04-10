pub fn reverse<T: Copy>(li: &Vec<T>) -> Vec<T> {
    let mut li_copy = li.clone();
    li_copy.reverse();
    li_copy
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse() {
        let li = vec![1, 1, 2, 3, 5, 8];
        assert_eq!(reverse(&li), vec![8, 5, 3, 2, 1, 1]);
    }
}
