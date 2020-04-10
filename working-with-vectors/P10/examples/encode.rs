use P10::encode;

pub fn main() {
    let li = vec![
        'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
    ];
    println!("{:?}", encode(&li));
}
