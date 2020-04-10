pub fn drop<T: Copy>(n: usize, li: &Vec<T>) -> Vec<T> {
    let mut res = vec![];
    for i in 0..li.len() {
        if (i + 1) % n != 0 {
            res.push(li[i]);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_drop_3() {
        let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
        assert_eq!(drop(3, &li), vec!['a', 'b', 'd', 'e', 'g', 'h', 'j', 'k']);
    }

    #[test]
    fn test_drop_2() {
        let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
        assert_eq!(drop(2, &li), vec!['a', 'c', 'e', 'g', 'i', 'k']);
    }

    #[test]
    fn test_drop_empty() {
        let li: Vec<i32> = vec![];
        assert_eq!(drop(3, &li), vec![]);
    }
}
