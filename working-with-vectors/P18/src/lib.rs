pub fn slice<T: Copy>(i: usize, k: usize, li: &Vec<T>) -> Vec<T> {
    if i > k {
        vec![]
    } else {
        li[i..k].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_slice_3to7() {
        let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
        assert_eq!(slice(3, 7, &li), vec!['d', 'e', 'f', 'g']);
    }

    #[test]
    fn test_slice_3to3() {
        let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
        assert_eq!(slice(3, 3, &li), vec![]);
    }

    #[test]
    fn test_slice_7to3() {
        let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
        assert_eq!(slice(7, 3, &li), vec![]);
    }
}
