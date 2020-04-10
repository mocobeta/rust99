use P41::*;

pub fn main() {
    for (n, p1, p2) in goldbach_list_limited(1, 2000, 50) {
        println!("{} = {} + {}", n, p1, p2);
    }
}
