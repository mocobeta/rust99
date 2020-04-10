use P19::rotate;

pub fn main() {
    let li = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'];
    println!("rotate on position 3: {:?}", rotate(3, &li));
    println!("rotate on position -2: {:?}", rotate(-2, &li));
}
