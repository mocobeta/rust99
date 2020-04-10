use P17::split;

pub fn main() {
    let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
    println!("{:?}", split(3, &li));
}
