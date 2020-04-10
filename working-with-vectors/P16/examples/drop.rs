use P16::drop;

pub fn main() {
    let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
    println!("{:?}", drop(3, &li));
}
