pub fn duplicate_n<T: Copy>(n: usize, li: &Vec<T>) -> Vec<T> {
    let mut res = vec![];
    li.clone()
        .into_iter()
        .for_each(|elem| (0..n).for_each(|_| res.push(elem)));
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
