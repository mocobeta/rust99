use P25::random_permute;

fn main() {
    let li = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    println!("{:?}", random_permute(&li));
    println!("{:?}", random_permute(&li));
    println!("{:?}", random_permute(&li));
}
