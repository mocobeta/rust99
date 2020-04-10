pub fn rotate<T: Copy>(n: i32, li: &Vec<T>) -> Vec<T> {
    if n > li.len() as i32 || n < -(li.len() as i32) {
        panic!("|n| must be smaller than length of the list")
    }
    let split = if n >= 0 { n } else { n + li.len() as i32 } as usize;

    let (lref, rref) = li.split_at(split);
    let left = lref.to_vec();
    let mut right = rref.to_vec();
    right.extend(left);
    right
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rotate_plus3() {
        let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
        assert_eq!(
            rotate(3, &li),
            vec!['d', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'a', 'b', 'c']
        );
    }

    #[test]
    fn test_rotate_minus2() {
        let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
        assert_eq!(
            rotate(-2, &li),
            vec!['j', 'k', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']
        );
    }

    #[test]
    fn test_rotate_0() {
        let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
        assert_eq!(
            rotate(0, &li),
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k']
        );
    }

    #[test]
    fn test_rotate_bond_val() {
        let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
        assert_eq!(
            rotate(li.len() as i32, &li),
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k']
        );
    }
}
