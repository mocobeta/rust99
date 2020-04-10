use P12::decode;

pub fn main() {
    let li = vec![(4, 'a'), (1, 'b'), (2, 'c'), (2, 'a'), (1, 'd'), (4, 'e')];
    println!("{:?}", decode(&li));
}
