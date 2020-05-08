use P68::*;

pub fn main() {
    let trees = from_preorder(&vec!['a', 'b', 'c']);
    for tree in trees {
        println!("{}", tree);
    }
}
