use P20::remove_at;

pub fn main() {
    let li = vec!['a', 'b', 'c', 'd'];
    println!("remove element at position 1: {:?}", remove_at(1, &li));
}
