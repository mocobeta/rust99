use bintree_strrepr::Tree;

pub fn main() {
    let tree = Tree::node(
        'a',
        Tree::node('b', Tree::leaf('d'), Tree::leaf('e')),
        Tree::node(
            'c',
            Tree::end(),
            Tree::node('f', Tree::leaf('g'), Tree::end()),
        ),
    );
    println!("{}", tree);
}
