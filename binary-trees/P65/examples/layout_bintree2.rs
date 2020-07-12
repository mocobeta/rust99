use P57::from_list;
use P65::*;

pub fn main() {
    let tree = from_list(&vec!['n', 'k', 'm', 'c', 'a', 'e', 'd', 'g', 'u', 'p', 'q']);
    println!("{}", layout_bintree2(&tree))
}
