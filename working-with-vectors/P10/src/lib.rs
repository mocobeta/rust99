use P09;

pub fn encode<T: Copy + PartialEq>(li: &Vec<T>) -> Vec<(usize, T)> {
    P09::pack(li).iter().map(|x| (x.len(), x[0])).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode() {
        let li = vec![
            'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
        ];
        assert_eq!(
            encode(&li),
            vec![(4, 'a'), (1, 'b'), (2, 'c'), (2, 'a'), (1, 'd'), (4, 'e')]
        );
    }

    #[test]
    fn test_encode_empty() {
        let li: Vec<i32> = vec![];
        assert_eq!(encode(&li), vec![]);
    }
}
