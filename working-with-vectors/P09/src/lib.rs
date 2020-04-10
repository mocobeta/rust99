pub fn pack<T: Copy + PartialEq>(li: &Vec<T>) -> Vec<Vec<T>> {
    if li.is_empty() {
        vec![]
    } else {
        let mut res = vec![vec![li[0]]];
        let mut prev = li[0];
        li.clone().into_iter().skip(1).for_each(|item| {
            if item == prev {
                if let Some(x) = res.last_mut() {
                    (*x).push(item);
                }
            } else {
                res.push(vec![item]);
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
    fn test_pack() {
        let li = vec![
            'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
        ];
        assert_eq!(
            pack(&li),
            vec![
                vec!['a', 'a', 'a', 'a'],
                vec!['b'],
                vec!['c', 'c'],
                vec!['a', 'a'],
                vec!['d'],
                vec!['e', 'e', 'e', 'e']
            ]
        );
    }
    #[test]
    fn test_pack_empty() {
        let li: Vec<i32> = vec![];
        assert_eq!(pack(&li), vec![] as Vec<Vec<i32>>);
    }
}
