pub fn and(a: bool, b: bool) -> bool {
    a && b
}

pub fn or(a: bool, b: bool) -> bool {
    a || b
}

pub fn not(a: bool) -> bool {
    !a
}

pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

pub fn nor(a: bool, b: bool) -> bool {
    !(a || b)
}

pub fn xor(a: bool, b: bool) -> bool {
    (a || b) && (!a || !b)
}

pub fn impli(a: bool, b: bool) -> bool {
    (a && b) || !a
}

pub fn equ(a: bool, b: bool) -> bool {
    impli(a, b) && impli(b, a)
}

pub fn table2(f: fn(bool, bool) -> bool) -> Vec<(bool, bool, bool)> {
    vec![
        (true, true, f(true, true)),
        (true, false, f(true, false)),
        (false, true, f(false, true)),
        (false, false, f(false, false)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand() {
        assert_eq!(nand(true, true), false);
        assert_eq!(nand(true, false), true);
        assert_eq!(nand(false, true), true);
        assert_eq!(nand(false, false), true);
    }

    #[test]
    fn test_nor() {
        assert_eq!(nor(true, true), false);
        assert_eq!(nor(true, false), false);
        assert_eq!(nor(false, true), false);
        assert_eq!(nor(false, false), true);
    }
    #[test]
    fn test_xor() {
        assert_eq!(xor(true, true), false);
        assert_eq!(xor(true, false), true);
        assert_eq!(xor(false, true), true);
        assert_eq!(xor(false, false), false);
    }

    #[test]
    fn test_impli() {
        assert_eq!(impli(true, true), true);
        assert_eq!(impli(true, false), false);
        assert_eq!(impli(false, true), true);
        assert_eq!(impli(false, false), true);
    }

    #[test]
    fn test_equ() {
        assert_eq!(equ(true, true), true);
        assert_eq!(equ(true, false), false);
        assert_eq!(equ(false, true), false);
        assert_eq!(equ(false, false), true);
    }

    #[test]
    fn test_table2() {
        let f = |a, b| and(a, or(a, b));
        assert_eq!(
            table2(f),
            vec![
                (true, true, true),
                (true, false, true),
                (false, true, false),
                (false, false, false)
            ]
        );
    }
}
