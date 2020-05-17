use mtree::MTree;
use P70C::*;

pub fn main() {
    let tree = MTree::node('a', vec![MTree::leaf('f')]);
    println!("node count = {}", node_count(&tree));
}
