pub fn insert_at<T: Copy>(elem: T, pos: usize, li: &Vec<T>) -> Vec<T> {
    let mut li_copy = li.clone();
    li_copy.insert(pos, elem);
    li_copy
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert_at_1() {
        let li = vec!['a', 'b', 'c', 'd'];
        assert_eq!(insert_at('n', 1, &li), vec!['a', 'n', 'b', 'c', 'd']);
    }
}
