use P67::*;

pub fn main() {
    let tree = Tree::from_string("a(b(d,e),c(,f(g,)))");
    println!("{}", tree);
}
