use P23::random_select;

pub fn random_permute<T: Copy>(li: &Vec<T>) -> Vec<T> {
    random_select(li.len(), li)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_random_permute() {
        let li = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        for _i in 0..100 {
            let permutation = random_permute(&li);
            assert_eq!(permutation.len(), li.len());
            assert!(permutation.iter().all(|&x| li.contains(&x)));
        }
    }
}
