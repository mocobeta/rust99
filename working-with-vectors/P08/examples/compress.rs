use P08::compress;

pub fn main() {
    let li = vec![
        'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
    ];
    println!("{:?}", compress(&li));
}
