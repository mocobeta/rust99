pub fn gray(n: usize) -> Vec<String> {
    let mut code = GrayCode::new();
    code.generate(n)
}

struct GrayCode {
    bits: usize,
    cache: Vec<String>,
}

impl GrayCode {
    pub fn new() -> Self {
        GrayCode {
            bits: 1,
            cache: vec!["0".to_string(), "1".to_string()],
        }
    }

    pub fn generate(&mut self, n: usize) -> Vec<String> {
        if n <= self.bits {
            self.cache
                .clone()
                .into_iter()
                .filter(|x| x.len() <= n)
                .collect()
        } else {
            let mut res = vec![];
            // enumerate code with highest order bit 0
            let code_n_1 = self.generate(n - 1);
            let code_0: Vec<String> = code_n_1
                .into_iter()
                .map(|c| {
                    let mut nc = "0".to_string();
                    nc.extend(c.chars());
                    nc
                })
                .collect();
            res.extend(code_0);
            // enumerate code with highest order bit 1
            let mut code_n_1_rev = self.generate(n - 1);
            code_n_1_rev.reverse();
            let code_1: Vec<String> = code_n_1_rev
                .into_iter()
                .map(|c| {
                    let mut nc = "1".to_string();
                    nc.extend(c.chars());
                    nc
                })
                .collect();
            res.extend(code_1);
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gray0() {
        assert_eq!(gray(0), vec![] as Vec<String>);
    }

    #[test]
    fn test_gray1() {
        assert_eq!(gray(1), vec!["0", "1"]);
    }

    #[test]
    fn test_gray2() {
        assert_eq!(gray(2), vec!["00", "01", "11", "10"]);
    }

    #[test]
    fn test_gray3() {
        assert_eq!(
            gray(3),
            vec!["000", "001", "011", "010", "110", "111", "101", "100"]
        );
    }
}
