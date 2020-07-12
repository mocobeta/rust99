use P57::from_list;
use P64::*;

pub fn main() {
    let tree = from_list(&vec![
        'n', 'k', 'm', 'c', 'a', 'h', 'g', 'e', 'u', 'p', 's', 'q',
    ]);
    println!("{}", layout_bintree(&tree))
}
