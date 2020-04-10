use P23::random_select;

pub fn main() {
    let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    println!("{:?}", random_select(3, &li));
}
