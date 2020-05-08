use bintree::Tree;
use P56::*;

pub fn main() {
    let tree = Tree::node('a', Tree::leaf('b'), Tree::leaf('c'));
    println!(
        "{} is {}.",
        tree,
        if is_symmetric(&tree) {
            "symmetric"
        } else {
            "not symmetric"
        }
    );

    let tree = Tree::node(
        'a',
        Tree::node('b', Tree::leaf('d'), Tree::end()),
        Tree::node('c', Tree::leaf('e'), Tree::end()),
    );
    println!(
        "{} is {}.",
        tree,
        if is_symmetric(&tree) {
            "symmetric"
        } else {
            "not symmetric"
        }
    );
}
