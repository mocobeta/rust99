use P21::insert_at;

pub fn main() {
    let li = vec!['a', 'b', 'c', 'd'];
    println!("insert 'n' at position 1: {:?}", insert_at('n', 1, &li));
}
