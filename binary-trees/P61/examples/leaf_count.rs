use bintree::Tree;
use P61::*;

pub fn main() {
    let tree = Tree::node('x', Tree::leaf('x'), Tree::end());
    println!("{}", leaf_count(&tree));
}
