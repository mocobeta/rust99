use P40::goldbach;

pub fn main() {
    let (p1, p2) = goldbach(28);
    println!("{} = {} + {}", 28, p1, p2);

    let (p1, p2) = goldbach(1_000_000);
    println!("{} = {} + {}", 1_000_000, p1, p2);
}
