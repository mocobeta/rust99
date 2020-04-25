use P46::*;

pub trait LogicalOps {
    fn and(&self, other: &bool) -> bool;
    fn or(&self, other: &bool) -> bool;
    fn nand(&self, other: &bool) -> bool;
    fn nor(&self, other: &bool) -> bool;
    fn xor(&self, other: &bool) -> bool;
    fn impli(&self, other: &bool) -> bool;
    fn equ(&self, other: &bool) -> bool;
}

impl LogicalOps for bool {
    fn and(&self, other: &bool) -> bool {
        and(*self, *other)
    }
    fn or(&self, other: &bool) -> bool {
        or(*self, *other)
    }
    fn nand(&self, other: &bool) -> bool {
        nand(*self, *other)
    }
    fn nor(&self, other: &bool) -> bool {
        nor(*self, *other)
    }
    fn xor(&self, other: &bool) -> bool {
        xor(*self, *other)
    }
    fn impli(&self, other: &bool) -> bool {
        impli(*self, *other)
    }
    fn equ(&self, other: &bool) -> bool {
        equ(*self, *other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use P46::table2;
    #[test]
    fn test_table2() {
        let f = |a: bool, b: bool| a.and(&a.or(&not(b)));
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
