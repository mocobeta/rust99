use P40::goldbach;

pub fn goldbach_list(lower: u32, upper: u32) -> Vec<(u32, u32, u32)> {
    (lower..upper + 1)
        .filter(|&n| n > 2 && n % 2 == 0)
        .map(|n| {
            let (p1, p2) = goldbach(n);
            (n, p1, p2)
        })
        .collect()
}

pub fn goldbach_list_limited(lower: u32, upper: u32, min: u32) -> Vec<(u32, u32, u32)> {
    goldbach_list(lower, upper)
        .into_iter()
        .filter(|&(_, p1, p2)| p1 > min && p2 > min)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_goldbach_list() {
        assert_eq!(
            goldbach_list(10, 20),
            vec![
                (10, 3, 7),
                (12, 5, 7),
                (14, 3, 11),
                (16, 3, 13),
                (18, 5, 13),
                (20, 3, 17)
            ]
        );
    }

    #[test]
    fn test_goldbach_list_limited() {
        assert_eq!(
            goldbach_list_limited(1, 2000, 50),
            vec![
                (992, 73, 919),
                (1382, 61, 1321),
                (1856, 67, 1789),
                (1928, 61, 1867)
            ]
        );
    }
}
