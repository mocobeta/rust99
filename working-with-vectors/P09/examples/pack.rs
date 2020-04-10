use P09::pack;

pub fn main() {
    let li = vec![
        'a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e',
    ];
    println!("{:?}", pack(&li));
}
