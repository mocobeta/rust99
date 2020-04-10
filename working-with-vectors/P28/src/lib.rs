use std::collections::HashMap;

pub fn lsort<T: Copy>(li: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut li_copy = li.clone();
    li_copy.sort_by(|a, b| a.len().cmp(&b.len()));
    li_copy
}

pub fn lsort_freq<T: Copy>(li: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    // map of (length -> list of lists)
    let length_map = li
        .iter()
        .fold(HashMap::<usize, Vec<Vec<T>>>::new(), |mut map, x| {
            let e = map.entry(x.len()).or_insert(vec![]);
            e.push(x.clone());
            map
        });
    // list of (length, freq) pair
    let mut length_freqs: Vec<(&usize, usize)> = length_map
        .iter()
        .map(|(len, lists)| (len, lists.len()))
        .collect();
    // sort by length frequency
    length_freqs.sort_by(|a, b| a.1.cmp(&b.1));
    // make final result
    let mut res = vec![];
    for (len, _) in length_freqs {
        if let Some(x) = length_map.get(&len) {
            res.extend_from_slice(x);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lsort() {
        let li = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e'],
            vec!['f', 'g', 'h'],
            vec!['d', 'e'],
            vec!['i', 'j', 'k', 'l'],
            vec!['m', 'n'],
            vec!['o'],
        ];
        let expected = vec![
            vec!['o'],
            vec!['d', 'e'],
            vec!['d', 'e'],
            vec!['m', 'n'],
            vec!['a', 'b', 'c'],
            vec!['f', 'g', 'h'],
            vec!['i', 'j', 'k', 'l'],
        ];
        assert_eq!(lsort(&li), expected);
    }

    #[test]
    fn test_lsort_freq() {
        let li = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e'],
            vec!['f', 'g', 'h'],
            vec!['d', 'e'],
            vec!['i', 'j', 'k', 'l'],
            vec!['m', 'n'],
            vec!['o'],
        ];

        let actual = lsort_freq(&li);
        assert!(
            (actual[0] == vec!['o'] && actual[1] == vec!['i', 'j', 'k', 'l'])
                || (actual[0] == vec!['i', 'j', 'k', 'l'] && actual[1] == vec!['o'])
        );
        assert!(
            (actual[2] == vec!['a', 'b', 'c'] && actual[3] == vec!['f', 'g', 'h'])
                || (actual[2] == vec!['f', 'g', 'h'] && actual[3] == vec!['a', 'b', 'c'])
        );
        assert!(actual[4] == vec!['d', 'e'] || actual[4] == vec!['m', 'n']);
        assert!(actual[5] == vec!['d', 'e'] || actual[5] == vec!['m', 'n']);
        assert!(actual[6] == vec!['d', 'e'] || actual[6] == vec!['m', 'n']);
    }
}
