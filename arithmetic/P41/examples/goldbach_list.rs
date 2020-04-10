use P41::*;

pub fn main() {
    for (n, p1, p2) in goldbach_list(9, 20) {
        println!("{} = {} + {}", n, p1, p2);
    }
}
