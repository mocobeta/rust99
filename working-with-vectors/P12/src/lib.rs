pub fn decode<T: Copy>(li: &Vec<(usize, T)>) -> Vec<T> {
    let mut res = vec![];
    for (n, elem) in li {
        for _i in 0..*n {
            res.push(*elem)
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode() {
        let li = vec![(4, 'a'), (1, 'b'), (2, 'c'), (2, 'a'), (1, 'd'), (4, 'e')];
        assert_eq!(
            decode(&li),
            vec!['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e']
        );
    }

    #[test]
    fn test_decode_empty() {
        let li: Vec<(usize, char)> = vec![];
        assert_eq!(decode(&li), vec![]);
    }
}
