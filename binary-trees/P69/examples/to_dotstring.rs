use P67::Tree;
use P69::*;

pub fn main() {
    let dstr = to_dotstring(&Tree::from_string("a(b(d,e),c(,f(g,)))"));
    println!("{}", dstr);
}
