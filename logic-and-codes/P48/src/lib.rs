pub fn table(argsize: usize, expr: fn(Vec<bool>) -> bool) -> Vec<Vec<bool>> {
    var_list(argsize)
        .into_iter()
        .map(|vars| {
            let mut res = vars.clone();
            res.push(expr(vars));
            res
        })
        .collect()
}

// calculate bool matrix for given size
fn var_list(argsize: usize) -> Vec<Vec<bool>> {
    var_list_rec(argsize, &vec![])
}

fn var_list_rec(k: usize, acc: &Vec<bool>) -> Vec<Vec<bool>> {
    if k == 0 {
        vec![acc.clone()]
    } else {
        let mut res = vec![];
        let mut acc2_true = acc.clone();
        acc2_true.push(true);
        res.extend(var_list_rec(k - 1, &acc2_true));
        let mut acc2_false = acc.clone();
        acc2_false.push(false);
        res.extend(var_list_rec(k - 1, &acc2_false));
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use P47::*;
    #[test]
    fn test_var_list() {
        assert_eq!(
            var_list(3),
            vec![
                vec![true, true, true],
                vec![true, true, false],
                vec![true, false, true],
                vec![true, false, false],
                vec![false, true, true],
                vec![false, true, false],
                vec![false, false, true],
                vec![false, false, false]
            ]
        );
    }
    #[test]
    fn test_table() {
        let f = |args: Vec<bool>| -> bool {
            let a = args.get(0).unwrap();
            let b = args.get(1).unwrap();
            let c = args.get(2).unwrap();
            a.and(&b.or(c)).equ(&a.and(&b).or(&a.and(&c)))
        };
        assert_eq!(
            table(3, f),
            vec![
                vec![true, true, true, true],
                vec![true, true, false, true],
                vec![true, false, true, true],
                vec![true, false, false, true],
                vec![false, true, true, true],
                vec![false, true, false, true],
                vec![false, false, true, true],
                vec![false, false, false, true]
            ]
        );
    }
}
