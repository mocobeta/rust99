pub fn encode_direct<T: Copy + PartialEq>(li: &Vec<T>) -> Vec<(usize, T)> {
    if li.is_empty() {
        vec![]
    } else {
        let mut res = vec![];
        let mut prev = li[0];
        let mut cnt = 1;
        li.clone().into_iter().skip(1).for_each(|e| {
            if e != prev {
                res.push((cnt, prev));
                prev = e;
                cnt = 1;
            } else {
                cnt += 1;
            }
        });
        res.push((cnt, prev));
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
