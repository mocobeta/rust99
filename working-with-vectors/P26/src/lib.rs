pub fn combinations<T: Copy>(k: usize, li: &Vec<T>) -> Vec<Vec<T>> {
    comb_rec(k, &li, &vec![])
}

fn comb_rec<T: Copy>(k: usize, rem: &Vec<T>, acc: &Vec<T>) -> Vec<Vec<T>> {
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
            let mut rem2: Vec<T> = rem.clone().into_iter().skip(i).collect();
            let x = rem2.remove(0);
            let mut acc2 = acc.clone();
            acc2.push(x);
            res.extend(comb_rec(k - 1, &rem2, &acc2));
        }
        res
    }
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
