use P18::slice;

pub fn main() {
    let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
    println!("{:?}", slice(3, 7, &li));
}
