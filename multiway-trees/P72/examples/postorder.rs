use P70::str_to_tree;
use P72::*;

pub fn main() {
    let tree = str_to_tree("afg^^c^bd^e^^^");
    println!("{:?}", postorder(&tree));
}
