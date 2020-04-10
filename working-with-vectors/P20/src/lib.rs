pub fn remove_at<T: Copy>(k: usize, li: &Vec<T>) -> (Vec<T>, T) {
    let mut li_copy = li.clone();
    let x = li_copy.remove(k);
    (li_copy, x)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_at_1() {
        let li = vec!['a', 'b', 'c', 'd'];
        assert_eq!(remove_at(1, &li), (vec!['a', 'c', 'd'], 'b'));
    }
}
