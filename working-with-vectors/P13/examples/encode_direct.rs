use P13::encode_direct;

pub fn main() {
    let li = vec![
        'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
    ];
    println!("{:?}", encode_direct(&li));
}
