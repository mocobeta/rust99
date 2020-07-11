pub fn combinations<T: Copy>(k: usize, li: &[T]) -> Vec<Vec<T>> {
    fn comb_rec<T: Copy>(k: usize, rem: &[T], acc: &Vec<T>) -> Vec<Vec<T>> {
        if k == 1 {
            rem.into_iter()
                .map(|&x| {
                    let mut acc2 = acc.clone();
                    acc2.push(x);
                    acc2
                })
                .collect()
        } else {
            let mut res = vec![];
            for i in 0..(rem.len() - k + 1) {
                let (left, right) = rem.split_at(i + 1);
                let x = left[left.len() - 1];
                let mut acc2 = acc.clone();
                acc2.push(x);
                res.extend(comb_rec(k - 1, &right, &acc2));
            }
            res
        }
    }

    comb_rec(k, &li, &vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test_combimations() {
        let li = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        let combs = combinations(3, &li);
        assert_eq!(combs.len(), 20);
        for comb in &combs {
            assert_eq!(comb.len(), 3);
            let uniq: HashSet<char> = comb.clone().into_iter().collect();
            assert_eq!(uniq.len(), 3);
        }

        let uniq: HashSet<Vec<char>> = combs.clone().into_iter().collect();
        assert_eq!(uniq.len(), 20);
    }
}
