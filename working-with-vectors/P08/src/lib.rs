pub fn compress<T: Copy + PartialEq>(li: &Vec<T>) -> Vec<T> {
    if li.is_empty() {
        vec![]
    } else {
        let mut res = vec![li[0]];
        for i in 1..li.len() {
            if li.get(i) != res.last() {
                res.push(*li.get(i).unwrap());
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compress() {
        let li = vec![
            'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
        ];
        assert_eq!(compress(&li), vec!['a', 'b', 'c', 'a', 'd', 'e']);
    }

    #[test]
    fn test_compress_empty() {
        let li: Vec<i32> = vec![];
        assert_eq!(compress(&li), vec![]);
    }
}
