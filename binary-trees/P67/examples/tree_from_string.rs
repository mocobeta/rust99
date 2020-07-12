use bintree_strrepr::Tree;

pub fn main() {
    let tree = Tree::from_string("a(b(d,e),c(,f(g,)))");
    println!("{:?}", tree);
}
