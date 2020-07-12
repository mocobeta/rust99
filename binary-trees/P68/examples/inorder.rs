use bintree_strrepr::Tree;
use P68::*;

pub fn main() {
    let tree = Tree::from_string("a(b(d,e),c(,f(g,)))");
    println!("{:?}", inorder(&tree));
}
