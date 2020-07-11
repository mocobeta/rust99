pub fn duplicate_n<T: Copy>(n: usize, li: &Vec<T>) -> Vec<T> {
    let mut res = vec![];
    for elem in li {
        for _i in 0..n {
            res.push(*elem);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_duplicate_n() {
        let li = vec!['a', 'b', 'c', 'c', 'd'];
        assert_eq!(
            duplicate_n(3, &li),
            vec!['a', 'a', 'a', 'b', 'b', 'b', 'c', 'c', 'c', 'c', 'c', 'c', 'd', 'd', 'd']
        );
    }
}
