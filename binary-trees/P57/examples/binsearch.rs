use bintree::Tree;
use P57::*;

pub fn main() {
    let mut tree = Tree::end();
    add_value(&mut tree, 2);
    println!("add 2: {}", tree);

    add_value(&mut tree, 3);
    println!("add 3: {}", tree);

    add_value(&mut tree, 0);
    println!("add 0: {}", tree);
}
