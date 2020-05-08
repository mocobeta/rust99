use bintree::Tree;
use P62B::*;

pub fn main() {
    let tree = Tree::node(
        'a',
        Tree::leaf('b'),
        Tree::node('c', Tree::leaf('d'), Tree::leaf('e')),
    );
    println!("{:?}", at_level(&tree, 2));
}
