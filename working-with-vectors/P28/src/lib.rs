use std::collections::HashMap;

pub fn lsort<T>(li: &Vec<Vec<T>>) -> Vec<&Vec<T>> {
    let mut lengths: Vec<(usize, &Vec<T>)> = li.iter().map(|x| (x.len(), x)).collect();
    lengths.sort_by(|a, b| a.0.cmp(&b.0));
    lengths.iter().map(|&(_, x)| x).collect()
}

pub fn lsort_freq<T>(li: &Vec<Vec<T>>) -> Vec<&Vec<T>> {
    // map of (length -> list of lists)
    let length_map = li
        .iter()
        .fold(HashMap::<usize, Vec<&Vec<T>>>::new(), |mut map, x| {
            let e = map.entry(x.len()).or_insert(vec![]);
            e.push(x);
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
        if let Some(lists) = length_map.get(&len) {
            res.extend_from_slice(lists);
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
        let actual = lsort(&li);
        assert_eq!(actual[0], &vec!['o']);
        assert_eq!(actual[1], &vec!['d', 'e']);
        assert_eq!(actual[2], &vec!['d', 'e']);
        assert_eq!(actual[3], &vec!['m', 'n']);
        assert_eq!(actual[4], &vec!['a', 'b', 'c']);
        assert_eq!(actual[5], &vec!['f', 'g', 'h']);
        assert_eq!(actual[6], &vec!['i', 'j', 'k', 'l']);
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
            (actual[0] == &vec!['o'] && actual[1] == &vec!['i', 'j', 'k', 'l'])
                || (actual[0] == &vec!['i', 'j', 'k', 'l'] && actual[1] == &vec!['o'])
        );
        assert!(
            (actual[2] == &vec!['a', 'b', 'c'] && actual[3] == &vec!['f', 'g', 'h'])
                || (actual[2] == &vec!['f', 'g', 'h'] && actual[3] == &vec!['a', 'b', 'c'])
        );
        assert!(actual[4] == &vec!['d', 'e'] || actual[4] == &vec!['m', 'n']);
        assert!(actual[5] == &vec!['d', 'e'] || actual[5] == &vec!['m', 'n']);
        assert!(actual[6] == &vec!['d', 'e'] || actual[6] == &vec!['m', 'n']);
    }
}
