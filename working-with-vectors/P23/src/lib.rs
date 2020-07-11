use rand::prelude::*;

pub fn random_select<T: Copy>(n: usize, li: &Vec<T>) -> Vec<T> {
    let mut rng = thread_rng();
    let mut res: Vec<T> = vec![];
    let mut tmp = li.clone();
    for _i in 0..n {
        let idx = rng.gen_range(0, tmp.len());
        res.push(tmp.remove(idx));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test_random_select_3() {
        let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        for _i in 0..100 {
            let selected = random_select(3, &li);
            assert_eq!(selected.len(), 3);
            let uniq: HashSet<char> = selected.into_iter().collect();
            assert_eq!(uniq.len(), 3);
        }
    }
}
