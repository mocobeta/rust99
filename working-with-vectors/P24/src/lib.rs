use P23::random_select;

pub fn lotto(n: usize, m: usize) -> Vec<usize> {
    let li: Vec<usize> = (1..m + 1).collect();
    random_select(n, &li)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test_lotto_6() {
        for _i in 0..100 {
            let drawn = lotto(6, 49);
            assert_eq!(drawn.len(), 6);
            assert!(drawn.iter().all(|&x| x <= 49));
            let uniq: HashSet<usize> = drawn.into_iter().collect();
            assert_eq!(uniq.len(), 6);
        }
    }
}
