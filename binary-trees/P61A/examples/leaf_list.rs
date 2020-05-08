use bintree::Tree;
use P61A::*;

pub fn main() {
    let tree = Tree::node(
        'a',
        Tree::leaf('b'),
        Tree::node('c', Tree::leaf('d'), Tree::leaf('e')),
    );
    println!("{:?}", leaf_list(&tree));
}
