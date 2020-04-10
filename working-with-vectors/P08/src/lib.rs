pub fn compress<T: Copy + PartialEq>(li: &Vec<T>) -> Vec<T> {
    if li.is_empty() {
        vec![]
    } else {
        let mut res = vec![li[0]];
        let mut prev = li[0];
        li.clone().into_iter().skip(1).for_each(|item| {
            if item != prev {
                res.push(item);
                prev = item;
            }
        });
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
