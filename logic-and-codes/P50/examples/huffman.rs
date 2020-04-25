use P50::*;

pub fn main() {
    let symbols = vec![
        ('a', 45),
        ('b', 13),
        ('c', 12),
        ('d', 16),
        ('e', 9),
        ('f', 5),
    ];
    println!("{:?}", huffman(&symbols));
}
