#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use P34;
    use P37;

    #[bench]
    pub fn bench_p34_totient(b: &mut Bencher) {
        b.iter(|| P34::totient(123456));
    }

    #[bench]
    pub fn bench_p37_totient(b: &mut Bencher) {
        b.iter(|| P37::totient(123456));
    }
}
