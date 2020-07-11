pub fn duplicate<T: Copy>(li: &Vec<T>) -> Vec<T> {
    let mut res = vec![];
    for elem in li {
        res.push(*elem);
        res.push(*elem);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_duplicate() {
        let li = vec!['a', 'b', 'c', 'c', 'd'];
        assert_eq!(
            duplicate(&li),
            vec!['a', 'a', 'b', 'b', 'c', 'c', 'c', 'c', 'd', 'd']
        );
    }
}
