pub fn encode_direct<T: Copy + PartialEq>(li: &Vec<T>) -> Vec<(usize, T)> {
    if li.is_empty() {
        vec![]
    } else {
        let mut res = vec![];
        let mut i = 0usize;
        while i < li.len() {
            let mut j = i;
            while j < li.len() {
                j += 1;
                if j == li.len() || li[i] != li[j] {
                    res.push((j - i, li[i]));
                    i = j;
                    break;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode_direct() {
        let li = vec![
            'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
        ];
        assert_eq!(
            encode_direct(&li),
            vec![(4, 'a'), (1, 'b'), (2, 'c'), (2, 'a'), (1, 'd'), (4, 'e')]
        );
    }
}
