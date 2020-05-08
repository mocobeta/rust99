use bintree::Tree;
use P65::*;

pub fn main() {
    let tree = Tree::node(
        'a',
        Tree::node('b', Tree::end(), Tree::leaf('c')),
        Tree::leaf('d'),
    );
    println!("{}", layout_bintree2(&tree))
}
