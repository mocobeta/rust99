use mtree::MTree;
use P73::*;

pub fn main() {
    let tree = MTree::node(
        'a',
        vec![
            MTree::node('f', vec![MTree::leaf('g')]),
            MTree::leaf('c'),
            MTree::node('b', vec![MTree::leaf('d'), MTree::leaf('e')]),
        ],
    );
    println!("{}", lispy_tree(&tree));
}
