#![feature(vec_remove_item)]

use P26::combinations;

pub fn group3<T: Copy + PartialEq>(li: &Vec<T>) -> Vec<Vec<Vec<T>>> {
    let mut all_groups = vec![];
    let combs1 = combinations(2, li);
    for comb1 in combs1 {
        let li_sub = subtract(li, &comb1);
        let combs2 = combinations(3, &li_sub);
        for comb2 in combs2 {
            let comb3 = subtract(&li_sub, &comb2);
            let mut group = vec![];
            group.push(comb1.clone());
            group.push(comb2);
            group.push(comb3);
            all_groups.push(group);
        }
    }
    all_groups
}

pub fn group<T: Copy + PartialEq>(sizes: &[usize], li: &Vec<T>) -> Vec<Vec<Vec<T>>> {
    group_rec(sizes, li, &vec![])
}

fn group_rec<T: Copy + PartialEq>(
    sizes: &[usize],
    rem: &Vec<T>,
    acc: &Vec<Vec<T>>,
) -> Vec<Vec<Vec<T>>> {
    if sizes.len() == 1 {
        if sizes[0] != rem.len() {
            panic!("incompatible size: {}; remainder={}", sizes[0], rem.len());
        }
        let mut acc2 = acc.clone();
        acc2.push(rem.clone());
        vec![acc2]
    } else {
        let mut groups = vec![];
        let combs = combinations(sizes[0], rem);
        for comb in combs {
            let mut acc2 = acc.clone();
            acc2.push(comb.clone());
            let rem2 = subtract(rem, &comb);
            groups.extend(group_rec(&sizes[1..], &rem2, &acc2));
        }
        groups
    }
}

fn subtract<T: Copy + PartialEq>(li: &Vec<T>, sub: &Vec<T>) -> Vec<T> {
    let mut diff = li.clone();
    for x in sub {
        diff.remove_item(x);
    }
    diff
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test_group3() {
        let li = vec![
            "Aldo", "Beat", "Carla", "David", "Evi", "Flip", "Gary", "Hugo", "Ida",
        ];
        let all_groups = group3(&li);
        for group in &all_groups {
            // checks for each group
            assert_eq!(group[0].len(), 2);
            assert_eq!(group[1].len(), 3);
            assert_eq!(group[2].len(), 4);

            let mut elems = Vec::<&str>::new();
            elems.extend(group[0].clone());
            elems.extend(group[1].clone());
            elems.extend(group[2].clone());
            assert_eq!(elems.len(), 9);

            let uniq: HashSet<&str> = elems.into_iter().collect();
            assert_eq!(uniq.len(), 9);
        }

        let total = all_groups.len();
        assert_eq!(total, 1260); // C(9, 2) x C(7, 3) x C(4, 4) = 1260
        let uniq: HashSet<Vec<Vec<&str>>> = all_groups.into_iter().collect();
        assert_eq!(uniq.len(), total);
    }

    #[test]
    fn test_group() {
        let li = vec![
            "Aldo", "Beat", "Carla", "David", "Evi", "Flip", "Gary", "Hugo", "Ida",
        ];
        let all_groups = group(&[2, 2, 5], &li);
        for group in &all_groups {
            // checks for each group
            assert_eq!(group[0].len(), 2);
            assert_eq!(group[1].len(), 2);
            assert_eq!(group[2].len(), 5);

            let mut elems = Vec::<&str>::new();
            elems.extend(group[0].clone());
            elems.extend(group[1].clone());
            elems.extend(group[2].clone());
            assert_eq!(elems.len(), 9);

            let uniq: HashSet<&str> = elems.into_iter().collect();
            assert_eq!(uniq.len(), 9);
        }

        let total = all_groups.len();
        assert_eq!(all_groups.len(), total); // C(9, 2) x C(7, 2) x C(5, 5) = 756
        let uniq: HashSet<Vec<Vec<&str>>> = all_groups.into_iter().collect();
        assert_eq!(uniq.len(), total);
    }
}
