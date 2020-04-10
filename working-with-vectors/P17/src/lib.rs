pub fn split<T: Copy>(n: usize, li: &Vec<T>) -> (Vec<T>, Vec<T>) {
    let mut left = li.clone();
    let right = left.split_off(n);
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split() {
        let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
        assert_eq!(
            split(3, &li),
            (
                vec!['a', 'b', 'c'],
                vec!['d', 'e', 'f', 'g', 'h', 'i', 'j', 'k']
            )
        );
    }
}
